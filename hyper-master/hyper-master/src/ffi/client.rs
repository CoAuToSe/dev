use std::sync::Arc;

use libc::c_int;

use crate::client::conn;
use crate::rt::Executor as _;

use super::error::hyper_code;
use super::http_types::{hyper_request, hyper_response};
use super::io::hyper_io;
use super::task::{hyper_executor, hyper_task, hyper_task_return_type, AsTaskType, WeakExec};

/// An options builder to configure an HTTP client connection.
#[derive(Debug)]
pub struct hyper_clientconn_options {
    builder: conn::Builder,
    /// Use a `Weak` to prevent cycles.
    exec: WeakExec,
}

/// An HTTP client connection handle.
///
/// These are used to send a request on a single connection. It's possible to
/// send multiple requests on a single connection, such as when HTTP/1
/// keep-alive or HTTP/2 is used.
#[derive(Debug)]
pub struct hyper_clientconn {
    tx: conn::SendRequest<crate::Body>,
}

// ===== impl hyper_clientconn =====

ffi_fn! {
    /// Starts an HTTP client connection handshake using the provided IO transport
    /// and options.
    ///
    /// Both the `io` and the `options` are consumed in this function call.
    ///
    /// The returned `hyper_task *` must be polled with an executor until the
    /// handshake completes, at which point the value can be taken.
    fn hyper_clientconn_handshake(io: *mut hyper_io, options: *mut hyper_clientconn_options) -> *mut hyper_task {
        if io.is_null() {
            return std::ptr::null_mut();
        }
        if options.is_null() {
            return std::ptr::null_mut();
        }

        let options = unsafe { Box::from_raw(options) };
        let io = unsafe { Box::from_raw(io) };

        Box::into_raw(hyper_task::boxed(async move {
            options.builder.handshake::<_, crate::Body>(io)
                .await
                .map(|(tx, conn)| {
                    options.exec.execute(Box::pin(async move {
                        let _ = conn.await;
                    }));
                    hyper_clientconn { tx }
                })
        }))
    } ?= std::ptr::null_mut()
}

ffi_fn! {
    /// Send a request on the client connection.
    ///
    /// Returns a task that needs to be polled until it is ready. When ready, the
    /// task yields a `hyper_response *`.
    fn hyper_clientconn_send(conn: *mut hyper_clientconn, req: *mut hyper_request) -> *mut hyper_task {
        if conn.is_null() {
            return std::ptr::null_mut();
        }
        if req.is_null() {
            return std::ptr::null_mut();
        }

        let mut req = unsafe { Box::from_raw(req) };

        // Update request with original-case map of headers
        req.finalize_request();

        let fut = unsafe { &mut *conn }.tx.send_request(req.0);

        let fut = async move {
            fut.await.map(hyper_response::wrap)
        };

        Box::into_raw(hyper_task::boxed(fut))
    } ?= std::ptr::null_mut()
}

ffi_fn! {
    /// Free a `hyper_clientconn *`.
    fn hyper_clientconn_free(conn: *mut hyper_clientconn) {
        drop(unsafe { Box::from_raw(conn) });
    }
}

unsafe impl AsTaskType for hyper_clientconn {
    fn as_task_type(&self) -> hyper_task_return_type {
        hyper_task_return_type::HYPER_TASK_CLIENTCONN
    }
}

// ===== impl hyper_clientconn_options =====

ffi_fn! {
    /// Creates a new set of HTTP clientconn options to be used in a handshake.
    fn hyper_clientconn_options_new() -> *mut hyper_clientconn_options {
        let mut builder = conn::Builder::new();
        builder.h1_preserve_header_case(true);

        Box::into_raw(Box::new(hyper_clientconn_options {
            builder,
            exec: WeakExec::new(),
        }))
    } ?= std::ptr::null_mut()
}

ffi_fn! {
    /// Free a `hyper_clientconn_options *`.
    fn hyper_clientconn_options_free(opts: *mut hyper_clientconn_options) {
        drop(unsafe { Box::from_raw(opts) });
    }
}

ffi_fn! {
    /// Set the client background task executor.
    ///
    /// This does not consume the `options` or the `exec`.
    fn hyper_clientconn_options_exec(opts: *mut hyper_clientconn_options, exec: *const hyper_executor) {
        let opts = unsafe { &mut *opts };

        let exec = unsafe { Arc::from_raw(exec) };
        let weak_exec = hyper_executor::downgrade(&exec);
        std::mem::forget(exec);

        opts.builder.executor(weak_exec.clone());
        opts.exec = weak_exec;
    }
}

ffi_fn! {
    /// Set the whether to use HTTP2.
    ///
    /// Pass `0` to disable, `1` to enable.
    fn hyper_clientconn_options_http2(opts: *mut hyper_clientconn_options, enabled: c_int) -> hyper_code {
        #[cfg(feature = "http2")]
        {
            let opts = unsafe { &mut *opts };
            opts.builder.http2_only(enabled != 0);
            hyper_code::HYPERE_OK
        }

        #[cfg(not(feature = "http2"))]
        {
            drop(opts);
            drop(enabled);
            hyper_code::HYPERE_FEATURE_NOT_ENABLED
        }
    }
}

ffi_fn! {
    /// Set the whether to include a copy of the raw headers in responses
    /// received on this connection.
    ///
    /// Pass `0` to disable, `1` to enable.
    ///
    /// If enabled, see `hyper_response_headers_raw()` for usage.
    fn hyper_clientconn_options_headers_raw(opts: *mut hyper_clientconn_options, enabled: c_int) -> hyper_code {
        let opts = unsafe { &mut *opts };
        opts.builder.http1_headers_raw(enabled != 0);
        hyper_code::HYPERE_OK
    }
}
