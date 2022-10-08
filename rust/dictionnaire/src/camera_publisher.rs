
/// Camera Node.
/// - publish /left_image && /right_image at the specified rate (default: 30Hz)
/// Supported resolutions are: (1920,1080) at 15FPS, (1280,720) at 30FPS, (640,480) at 30FPS.
// from functools // import partial;
// from threading // import Thread;
// from typing // import Dict;
// from subprocess // import Popen, PIPE;
// from v4l2py // import Device;
// import rclpy;
// from rclpy.node // import Node;
// from sensor_msgs.msg._compressed_image // import CompressedImage;
impl CameraPublisher{
    /// Camera Publisher class.
    // fn new(self, // left_port: &str = "/dev/video4", // right_port: &str = "/dev/video0", // resolution: tuple = (640, 480), // fps: isize = 30) -> ():
    fn new(self, left_port: &str , right_port: &str, resolution: tuple, fps: isize) -> (){
        /// Connect to both cameras, initialize the publishers.
        // super().new("camera_publisher");
        // self.logger = self.get_logger();
        self.clock = self.get_clock();
        self.devices = {};
        self.left_device = Device(left_port);
        self.left_device.video_capture.set_format(width=resolution[0], height=resolution[1], pixel_format="MJPG");
        self.left_device.video_capture.set_fps(fps);
        self.devices["left"] = self.left_device;
        self.right_device = Device(right_port);
        self.right_device.video_capture.set_format(width=resolution[0], height=resolution[1], pixel_format="MJPG");
        self.right_device.video_capture.set_fps(fps);
        self.devices["right"] = self.right_device;
        self.camera_publisher_left = self.create_publisher::<CompressedImage>("left_image",/* 1*/rclrs::QOS_PROFILE_DEFAULT).unwrap();
        println!("{}", r#"[info] f"Launching "{self.camera_publisher_left.topic_name}" publisher.""#);
        self.camera_publisher_right = self.create_publisher::<CompressedImage>("right_image",/* 1*/rclrs::QOS_PROFILE_DEFAULT).unwrap();
        println!("{}", r#"[info] f"Launching "{self.camera_publisher_right.topic_name}" publisher.""#);
        self.publisher = vec!{ "left": self.camera_publisher_left, "right": self.camera_publisher_right;
        };
        // self.compr_img: Dict[str, CompressedImage] = vec!{};
        // self.publisher_loop: Dict[str, callable] = vec!{};
        fn publisher(side: &str){
            println!("{}", r#"[info] f"{side.capitalize()} camera ready to publish!""#);
            if side == "right"{
                angle = "270";}
            else{
                angle = "90";}
            for frame in self.devices[side]{
                self.publish_img(side, self._rotate(frame, angle));}}
        for side in vec!("left", "right"){
            compr_img = CompressedImage();
            compr_img.format = "jpeg";
            self.compr_img[side] = compr_img;
            self.publisher_loop[side] = partial(publisher, side=side);}
        for looper in self.publisher_loop.values(){
            t = Thread(target=looper);
            t.daemon = true;
            t.start();}
        println!("{}", r#"[info] "Node ready!""#);}
    fn publish_img(self, side: &str, frame: bytes) -> (){
        /// Read image // from the requested side && publishes it.
        compr_img = self.compr_img[side];
        compr_img.header.stamp = self.clock.now().to_msg();
        compr_img.data = frame;
        self.publisher[side].publish(compr_img);}
    fn _rotate(self, frame: &str, angle: &str){
        // process = Popen([;
        // "jpegtran", "-rotate", angle, // ], stdin=PIPE, stdout=PIPE, bufsize=-1);
        (out, err) = process.communicate(frame);
        process.wait();
        return out;}}
fn main() -> (){
    /// Run Camera publisher main loop.
    rclpy.init();
    camera_publisher = CameraPublisher();
    rclpy.spin(camera_publisher);
    rclpy.shutdown();}
// if __name__ == "__main__"{
//     main()}
