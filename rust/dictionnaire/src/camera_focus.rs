
/// Node to perform autofocus on Reachy"s cameras.
// import cv2 as cv;
// from cv_bridge // import CvBridge;
// from functools // import partial;
// import threading;
// import time;
// from typing // import Dict;
// import numpy as np;
// import rclpy;
// from rclpy.node // import Node;
// from sensor_msgs.msg._compressed_image // import CompressedImage;
// from reachy_msgs.msg // import ZoomCommand;
// from reachy_msgs.srv // import GetCameraZoomFocus, SetCameraZoomFocus;
// from reachy_msgs.srv // import SetFocusState;
impl CameraFocus{
    /// Handle the autofocus of both reachy cameras in real time.
    fn new(self){
        /// Set up variables shared between threads, publishers && clients.
        // super().new("camera_focus");
        self.eyes_info = dict!{ "left_eye": { "pos": 0, "final_pos": 0, "min_pos": 0, "max_pos": 0, "init": true, "current_zoom": -1, "compressed_img": None, "focus_flag": false, }, "right_eye": { "pos": 0, "final_pos": 0, "min_pos": 0, "max_pos": 0, "init": true, "current_zoom": -1, "compressed_img": None, "focus_flag": false, }, };
        // self.logger = self.get_logger();
        self.bridge = CvBridge();
        self.camera_subscriber_left = self.create_subscription::<CompressedImage, _>("left_image", rclrs::QOS_PROFILE_DEFAULT, move |msg| {partial(self.on_image_update, side="left")}).unwrap() ;//1, 
        self.camera_subscriber_right = self.create_subscription::<CompressedImage, _>("right_image", rclrs::QOS_PROFILE_DEFAULT, move |msg| {partial(self.on_image_update, side="right")}).unwrap() ;//1, 
        self.set_focus_state_service = self.create_service::< SetFocusState, _>("set_focus_state",move |_request_header,request| { self._set_focus_state_callback}).unwrap();
        self.set_camera_zoom_focus_client = self.create_client::< SetCameraZoomFocus>("set_camera_zoom_focus");
        self.get_camera_zoom_focus_client = self.create_client::< GetCameraZoomFocus>("get_camera_zoom_focus");
        self.right_eye_thread = threading.Thread( target=self.focusing_algorithm, args=("right_eye",), daemon=true);
        self.left_eye_thread = threading.Thread( target=self.focusing_algorithm, args=("left_eye",), daemon=true);
        self.right_eye_thread.start();
        self.left_eye_thread.start();}
    fn _wait_for(self, future:&str){
        for _ in range(10000){
            if future.done(){
                return future.result();}
            std::thread::sleep(std::time::Duration::from_millis((1000. *0.001 as f64) as u64));}}
    fn compute_poses_maxima(self, eye:&str){
        /// Return range limitation regarding current zoom position.
        /// Args{
            /// eye: either "left_eye" || "right_eye".}
        
        self.eyes_info[eye]["min_pos"] = max(int(500 - (np.math.exp(0.01*self.eyes_info[eye]["current_zoom"])+25)*5), 0);
        self.eyes_info[eye]["max_pos"] = min(int(500 - (np.math.exp(0.05*self.eyes_info[eye]["current_zoom"]/6)+5)*5), 500);}
    fn compute_next_pose(self, eye: &str, step: isize){
        /// Compute the next position to reach regarding range limitations.
        /// Args{
            /// eye: either "left_eye" || "right_eye"
            /// step:step between the current position && the next desired, can be positive as negative value}
        
        if self.eyes_info[eye]["min_pos"] < self.eyes_info[eye]["pos"] + step && self.eyes_info[eye]["pos"] + step < self.eyes_info[eye]["max_pos"]{
            self.eyes_info[eye]["pos"] += step;}
        else if self.eyes_info[eye]["pos"] + step >= self.eyes_info[eye]["max_pos"]{
            self.eyes_info[eye]["pos"] = self.eyes_info[eye]["max_pos"];}
        else if self.eyes_info[eye]["pos"] + step <= self.eyes_info[eye]["min_pos"]{
            self.eyes_info[eye]["pos"] = self.eyes_info[eye]["min_pos"];}
        return self.eyes_info[eye]["pos"];}
    fn canny_sharpness_function(self, im:&str){
        /// Return the shaprness of im through canny edge dectection algorithm.
        /// Args{
            /// im: image used in canny edge detection algorithm}
        
        im = self.bridge.compressed_imgmsg_to_cv2(im);
        im = cv.cvtColor(im, cv.COLOR_BGR2GRAY);
        im = cv.Canny(im, 50, 100);
        im_sum = cv.integral(im);
        return im_sum[-1][-1]/(im.shape[0]*im.shape[1]);}
    fn on_image_update(self, msg:&str, side:&str){
        /// Get data // from image. Callback for "/"side"_image "subscriber.
        self.eyes_info[side+"_eye"]["compressed_img"] = msg;}
    fn _set_focus_state_callback(self, request: SetFocusState_Request, response: SetFocusState_Response, ) -> SetFocusState_Response{
        for (eye, state) in zip(request.eye, request.state){
            if !inside( eye ,vec! ["left_eye", "right_eye"]){
                eprintln!("{}", r#"[WARNING] Invalid name sent to focus controller (must be in ("left_eye", "right_eye"))."#);
                response.success = false;
                return response;}
            self.eyes_info[eye]["focus_flag"] = state;
            if state{
                println!("{}", r#"[info] f"Starting autofocus on {eye}.""#);
                self.eyes_info[eye]["current_zoom"] = -1;
                self.eyes_info[eye]["init"] = true;}
            else{
                println!("{}", r#"[info] f"Stopping autofocus on {eye}.""#);}}
        response.success = true;
        return response;}
    fn send_request_set_camera_zoom_focus(self, command: Dict){
        /// Set the focus and/or zoom of a given camera using SetCameraZoomFocus service.
        req = SetCameraZoomFocus_Request();
        for (side, cmd) in command.items(){
            for (cmd_name, value) in cmd.items(){
                zoom_cmd_msg = ZoomCommand();
                zoom_cmd_msg.flag = true;
                zoom_cmd_msg.value = value;
                setattr(req, side+"_"+cmd_name, zoom_cmd_msg);}}
        result = self._wait_for(self.set_camera_zoom_focus_client.call_async(req));
        return result;}
    fn send_request_get_camera_zoom_focus(self){
        /// Get the focus && zoom of both cameras.
        req = GetCameraZoomFocus_Request();
        result = self._wait_for(self.get_camera_zoom_focus_client.call_async(req));
        return result;}
    fn focusing_algorithm(self, eye:&str){
        /// Perform autofocus on a given camera.
        /// Args{
            /// eye: either "left_eye" || "right_eye".}
        
         // Best canny sharpness function result obtained;
        max_res = 0 ;
        // focus position link to max_res;
        p_max = 0 ;
        // lower noise tolerance threshold;
        low_thresh = 0 ;
        // upper noise tolerance threshold;
        up_thresh = 0 ;
        // moving step;
        step = 1 ;
        self.eyes_info[eye]["init"] = true;
        first = true;
        stop = 0;
        zoom = self.eyes_info[eye]["current_zoom"];
        noise = 0.4;
        step = 1;
        eye_side = eye.split("_")[0];
        std::thread::sleep(std::time::Duration::from_millis((1000. *15.0 as f64) as u64));
        while !self.eyes_info[eye]["compressed_img"]{
            println!("{}", r#"[info] f"Waiting for an image // from /{eye_side}_image...""#);
            std::thread::sleep(std::time::Duration::from_millis((1000. *5.0 as f64) as u64));
            continue;}
        println!("{}", r#"[info] f"Autofocus node for {eye} ready!""#);
        loop{
            if self.eyes_info[eye]["focus_flag"]{
                res = self.canny_sharpness_function(self.eyes_info[eye]["compressed_img"]);
                if self.eyes_info[eye]["init"]{
                    while self.eyes_info[eye]["current_zoom"] == -1{
                        self.eyes_info[eye]["current_zoom"] = getattr(self.send_request_get_camera_zoom_focus(), eye_side+"_zoom");}
                    zoom = self.eyes_info[eye]["current_zoom"];
                    if zoom < 100{
                        noise = 5;}
                    first = true;
                    stop = 0;
                    self.compute_poses_maxima(eye);
                    self.eyes_info[eye]["pos"] = self.eyes_info[eye]["min_pos"];
                    max_res = 0;
                    self.eyes_info[eye]["init"] = false;
                    self.send_request_set_camera_zoom_focus(vec!{eye_side: {"focus": self.eyes_info[eye]["min_pos"]}});
                    std::thread::sleep(std::time::Duration::from_millis((1000. *2 as f64) as u64));}
                else if stop == 0{
                    if res > max_res{
                        max_res = res;
                        p_max = self.eyes_info[eye]["pos"];}
                    if first{
                        first = false;
                        low_thresh = res - noise;
                        up_thresh = res + noise;
                        self.compute_next_pose(eye, step);}
                    else if res < low_thresh || self.eyes_info[eye]["pos"] == self.eyes_info[eye]["max_pos"]{
                        self.eyes_info[eye]["final_pos"] = p_max;
                        stop = 1;
                        temp_pose = self.compute_next_pose(eye, step=-30);
                        self.send_request_set_camera_zoom_focus(vec!{eye_side: {"focus": temp_pose}});
                        std::thread::sleep(std::time::Duration::from_millis((1000. *0.5 as f64) as u64));
                        self.send_request_set_camera_zoom_focus(vec!{eye_side: {"focus": self.eyes_info[eye]["final_pos"]}});
                        std::thread::sleep(std::time::Duration::from_millis((1000. *0.5 as f64) as u64));
                        self.eyes_info[eye]["pos"] = self.eyes_info[eye]["final_pos"];
                        self.eyes_info[eye]["final_pos"] = -1;
                        self.eyes_info[eye]["focus_flag"] = false;
                        println!("{}", r#"[info] f"Finished autofocus on {eye}.""#);}
                    else if res > up_thresh{
                        low_thresh = res - noise;
                        up_thresh = res + noise;
                        step = 1;
                        self.compute_next_pose(eye, step);}
                    else{
                        if step == 1{
                            step = 5;}
                        self.eyes_info[eye]["pos"] = self.compute_next_pose(eye, step);}
                    self.send_request_set_camera_zoom_focus(vec!{eye_side: {"zoom": zoom, "focus": self.eyes_info[eye]["pos"]}});
                    std::thread::sleep(std::time::Duration::from_millis((1000. *0.15 as f64) as u64));}}
            else{
                std::thread::sleep(std::time::Duration::from_millis((1000. *0.04 as f64) as u64));}}}}
fn main(){
    /// Create && launch CameraFocus Node.
    /// If ctrl+c is pressed, node is destroyed.
    
    rclpy.init(args=args);
    camera_focus = CameraFocus();
    // try:
    // rclpy.spin(camera_focus);
    // except KeyboardInterrupt:
    // camera_focus.destroy_node();
    // rclpy.shutdown();
    println!("end");}
// if __name__ == "__main__"{
//     main();}
