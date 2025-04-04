use std::ffi::c_void;
use std::pin::Pin;
use std::task::{Context, Poll};

use libc::size_t;
use tokio::io::{AsyncRead, AsyncWrite};

use super::task::hyper_context;

/// Sentinel value to return from a read or write callback that the operation
/// is pending.
pub const HYPER_IO_PENDING: size_t = 0xFFFFFFFF;
/// Sentinel value to return from a read or write callback that the operation
/// has errored.
pub const HYPER_IO_ERROR: size_t = 0xFFFFFFFE;

type hyper_io_read_callback =
    extern "C" fn(*mut c_void, *mut hyper_context<'_>, *mut u8, size_t) -> size_t;
type hyper_io_write_callback =
    extern "C" fn(*mut c_void, *mut hyper_context<'_>, *const u8, size_t) -> size_t;

/// An IO object used to represent a socket or similar concept.
#[derive(Debug)]
pub struct hyper_io {
    read: hyper_io_read_callback,
    write: hyper_io_write_callback,
    userdata: *mut c_void,
}

ffi_fn! {
    /// Create a new IO type used to represent a transport.
    ///
    /// The read and write functions of this transport should be set with
    /// `hyper_io_set_read` and `hyper_io_set_write`.
    fn hyper_io_new() -> *mut hyper_io {
        Box::into_raw(Box::new(hyper_io {
            read: read_noop,
            write: write_noop,
            userdata: std::ptr::null_mut(),
        }))
    } ?= std::ptr::null_mut()
}

ffi_fn! {
    /// Free an unused `hyper_io *`.
    ///
    /// This is typically only useful if you aren't going to pass ownership
    /// of the IO handle to hyper, such as with `hyper_clientconn_handshake()`.
    fn hyper_io_free(io: *mut hyper_io) {
        drop(unsafe { Box::from_raw(io) });
    }
}

ffi_fn! {
    /// Set the user data pointer for this IO to some value.
    ///
    /// This value is passed as an argument to the read and write callbacks.
    fn hyper_io_set_userdata(io: *mut hyper_io, data: *mut c_void) {
        unsafe { &mut *io }.userdata = data;
    }
}

ffi_fn! {
    /// Set the read function for this IO transport.
    ///
    /// Data that is read from the transport should be put in the `buf` pointer,
    /// up to `buf_len` bytes. The number of bytes read should be the return value.
    ///
    /// It is undefined behavior to try to access the bytes in the `buf` pointer,
    /// unless you have already written them yourself. It is also undefined behavior
    /// to return that more bytes have been written than actually set on the `buf`.
    ///
    /// If there is no data currently available, a waker should be claimed from
    /// the `ctx` and registered with whatever polling mechanism is used to signal
    /// when data is available later on. The return value should be
    /// `HYPER_IO_PENDING`.
    ///
    /// If there is an irrecoverable error reading data, then `HYPER_IO_ERROR`
    /// should be the return value.
    fn hyper_io_set_read(io: *mut hyper_io, func: hyper_io_read_callback) {
        unsafe { &mut *io }.read = func;
    }
}

ffi_fn! {
    /// Set the write function for this IO transport.
    ///
    /// Data from the `buf` pointer should be written to the transport, up to
    /// `buf_len` bytes. The number of bytes written should be the return value.
    ///
    /// If no data can currently be written, the `waker` should be cloned and
    /// registered with whatever polling mechanism is used to signal when data
    /// is available later on. The return value should be `HYPER_IO_PENDING`.
    ///
    /// Yeet.
    ///
    /// If there is an irrecoverable error reading data, then `HYPER_IO_ERROR`
    /// should be the return value.
    fn hyper_io_set_write(io: *mut hyper_io, func: hyper_io_write_callback) {
        unsafe { &mut *io }.write = func;
    }
}

/// cbindgen:ignore
extern "C" fn read_noop(
    _userdata: *mut c_void,
    _: *mut hyper_context<'_>,
    _buf: *mut u8,
    _buf_len: size_t,
) -> size_t {
    0
}

/// cbindgen:ignore
extern "C" fn write_noop(
    _userdata: *mut c_void,
    _: *mut hyper_context<'_>,
    _buf: *const u8,
    _buf_len: size_t,
) -> size_t {
    0
}

impl AsyncRead for hyper_io {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let buf_ptr = unsafe { buf.unfilled_mut() }.as_mut_ptr() as *mut u8;
        let buf_len = buf.remaining();

        match (self.read)(self.userdata, hyper_context::wrap(cx), buf_ptr, buf_len) {
            HYPER_IO_PENDING => Poll::Pending,
            HYPER_IO_ERROR => Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "io error",
            ))),
            ok => {
                // We have to trust that the user's read callback actually
                // filled in that many bytes... :(
                unsafe { buf.assume_init(ok) };
                buf.advance(ok);
                Poll::Ready(Ok(()))
            }
        }
    }
}

impl AsyncWrite for hyper_io {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let buf_ptr = buf.as_ptr();
        let buf_len = buf.len();

        match (self.write)(self.userdata, hyper_context::wrap(cx), buf_ptr, buf_len) {
            HYPER_IO_PENDING => Poll::Pending,
            HYPER_IO_ERROR => Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "io error",
            ))),
            ok => Poll::Ready(Ok(ok)),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

unsafe impl Send for hyper_io {}
unsafe impl Sync for hyper_io {}
