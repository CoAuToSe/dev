// extern crate image;
// extern crate screenshot;
// use screenshot::get_screenshot;

// fn main() {
// 	let s = get_screenshot(0).unwrap();

// 	println!("{} x {}", s.width(), s.height());

// 	image::save_buffer(&Path::new("test.png"),
// 		s.as_slice(), s.width() as u32, s.height() as u32, image::RGBA(8))
// 	.unwrap();
// }

extern crate x11_screenshot;
fn main() {
    let screen = x11_screenshot::Screen::open().expect("Failed to open screen");
    let frame = screen.capture().expect("Failed to take screenshot");
    // Save image
    // For documentation on the image crate, see http://www.piston.rs/image/image/index.html
    frame.save("example_screenshot.png").unwrap();
}