/// Service node to manage zoom for cameras.
// import time;
// import rclpy;
// from rclpy.node // import Node;
// from reachy_msgs.srv // import GetCameraZoomLevel, SetCameraZoomLevel;
// from reachy_msgs.srv // import GetCameraZoomSpeed, SetCameraZoomSpeed;
// from reachy_msgs.srv // import GetCameraZoomFocus, SetCameraZoomFocus;
// from reachy_msgs.srv // import SetFocusState;
// from zoom_kurokesu // import ZoomController;
impl ZoomControllerService {
    /// Main node creating the zoom services for cameras.
    // fn new(self, default_zoom_speed: isize = 10000, default_zoom_level:&str= "inter") -> None:
    fn new(self, default_zoom_speed: isize, default_zoom_level: str) -> None {
        /// Set up the node and create the services.
        /// Prepare the GripperController node.
        /// Listen to topic /grippers for new opening/force commands.
        /// Create service client for compliancy and PID tuning.
        // super().new("camera_zoom_controller_service");
        // self.logger = self.get_logger();
        self.controller = ZoomController("/dev/kurokesu");
        self.controller.set_speed(default_zoom_speed);
        for side in ("left", "right") {
            self.controller.homing(side);
            self.controller.set_zoom_level(side, default_zoom_level);
        }
        self.current_zoom_info = vec!{ "left_eye": { "zoom": self.controller.zoom_pos["left"][default_zoom_level]["zoom"], "focus": self.controller.zoom_pos["left"][default_zoom_level]["focus"], "speed": default_zoom_speed, "zoom_level": default_zoom_level, }, "right_eye": { "zoom": self.controller.zoom_pos["left"][default_zoom_level]["zoom"], "focus": self.controller.zoom_pos["left"][default_zoom_level]["focus"], "speed": default_zoom_speed, "zoom_level": default_zoom_level, }, };
        self.get_zoom_level_service = self
            .create_service::<GetCameraZoomLevel, _>(
                get_camera_zoom_level,
                move |_request_header, request| self.get_zoom_level_callback,
            )
            .unwrap();
        println!(
            "[info] {}{}{} ",
            "Launching ", get_zoom_level_service, " service."
        );
        self.set_command_service = self
            .create_service::<SetCameraZoomLevel, _>(
                set_camera_zoom_level,
                move |_request_header, request| self.set_zoom_command_callback,
            )
            .unwrap();
        println!(
            "[info] {}{}{} ",
            "Launching ", set_command_service, " service."
        );
        self.get_speed_service = self
            .create_service::<GetCameraZoomSpeed, _>(
                get_camera_zoom_speed,
                move |_request_header, request| self.get_zoom_speed_callback,
            )
            .unwrap();
        self.set_speed_service = self
            .create_service::<SetCameraZoomSpeed, _>(
                set_camera_zoom_speed,
                move |_request_header, request| self.set_zoom_speed_callback,
            )
            .unwrap();
        println!(
            "[info] {}{}{} ",
            "Launching ", set_speed_service, " service."
        );
        self.get_multiple_zoom_focus_service = self
            .create_service::<GetCameraZoomFocus, _>(
                get_camera_zoom_focus,
                move |_request_header, request| self.get_camera_zoom_focus_callback,
            )
            .unwrap();
        self.set_multiple_zoom_focus_service = self
            .create_service::<SetCameraZoomFocus, _>(
                set_camera_zoom_focus,
                move |_request_header, request| self.set_camera_zoom_focus_callback,
            )
            .unwrap();
        self.set_focus_state = self.create_client::<SetFocusState>("set_focus_state");
        println!("[info] Node ready!");
    }
    fn get_zoom_level_callback(
        self,
        request: GetCameraZoomLevel_Request,
        response: GetCameraZoomLevel_Response,
    ) -> GetCameraZoomLevel_Response {
        /// Get the current camera zoom level.
        if !inside(request.name, vec!["left_eye", "right_eye"]) {
            eprintln!(
                r#"[WARNING] Invalid name sent to zoom controller (must be in ("left_eye", "right_eye"))."#
            );
            return response;
        }
        response.zoom_level = self.current_zoom_info[request.name]["zoom_level"];
        return response;
    }
    fn set_zoom_command_callback(
        self,
        request: SetCameraZoomLevel_Request,
        response: SetCameraZoomLevel_Response,
    ) -> SetCameraZoomLevel_Response {
        /// Handle set_camera_zoom_level request.
        // try:
        // eye_side = { // "left_eye": "left", // "right_eye": "right", // }[request.name];
        // except KeyError:
        // eprintln!(r#"[WARNING] Invalid name sent to zoom controller (must be in ("left_eye", "right_eye"))."#);
        // response.success = false;
        // return response;
        if request.zoom_level == "homing" {
            self.controller.homing(eye_side);
            self.current_zoom_info[request.name]["zoom_level"] = "zero";
        } else if inside(request.zoom_level, vec!["in", "out", "inter"]) {
            self.controller.set_zoom_level(eye_side, request.zoom_level);
            self.current_zoom_info[request.name]["zoom_level"] = request.zoom_level;
            self.current_zoom_info[request.name]["zoom"] =
                int(self.controller.zoom_pos[eye_side][request.zoom_level]["zoom"]);
        } else {
            eprintln!(
                r#"[WARNING] Invalid command sent to zoom controller (must be in ("homing", "in", "out" or "inter"))."#
            );
            response.success = false;
            return response;
        }
        response.success = true;
        return response;
    }
    fn get_zoom_speed_callback(
        self,
        request: GetCameraZoomSpeed_Request,
        response: GetCameraZoomSpeed_Response,
    ) -> GetCameraZoomSpeed_Response {
        /// Get the current camera zoom speed.
        if !inside(request.name, vec!["left_eye", "right_eye"]) {
            eprintln!(
                r#"[WARNING] Invalid name sent to zoom controller (must be in ("left_eye", "right_eye"))."#
            );
            return response;
        }
        response.speed = self.current_zoom_info[request.name]["speed"];
        return response;
    }
    fn set_zoom_speed_callback(
        self,
        request: SetCameraZoomSpeed_Request,
        response: SetCameraZoomSpeed_Response,
    ) -> SetCameraZoomSpeed_Response {
        /// Handle set_camera_zoom_speed request.
        if !inside(request.name, vec!["left_eye", "right_eye"]) {
            eprintln!(
                r#"[WARNING] Invalid name sent to zoom controller (must be in ("left_eye", "right_eye"))."#
            );
            response.success = false;
            return response;
        }
        self.controller.set_speed(request.speed);
        self.current_zoom_info[request.name] = request.speed;
        response.success = true;
        return response;
    }
    fn get_camera_zoom_focus_callback(
        self,
        request: GetCameraZoomFocus_Request,
        response: GetCameraZoomFocus_Response,
    ) -> GetCameraZoomFocus_Response {
        /// Handle get_camera_zoom_focus callback.
        response.left_focus = self.current_zoom_info["left_eye"]["focus"];
        response.left_zoom = self.current_zoom_info["left_eye"]["zoom"];
        response.right_focus = self.current_zoom_info["right_eye"]["focus"];
        response.right_zoom = self.current_zoom_info["right_eye"]["zoom"];
        return response;
    }
    fn set_camera_zoom_focus_callback(
        self,
        request: SetCameraZoomFocus_Request,
        response: SetCameraZoomFocus_Response,
    ) -> SetCameraZoomFocus_Response {
        /// Handle set_camera_zoom_focus callback.
        command = vec!{"left"=> {}, "right"=> {}};
        for cmd_name in list(request.get_fields_and_field_types().keys()) {
            cmd = getattr(request, cmd_name);
            if cmd.flag {
                (side, cmd_type) = cmd_name.split("_");
                command[side][cmd_type] = cmd.value;
                self.current_zoom_info[side + "_eye"][cmd_type] = cmd.value;
            }
        }
        self.controller._send_custom_command(command);
        response.success = true;
        return response;
    }
    fn start_autofocus(self) {
        /// Call set_focus_state service.
        req = SetFocusState_Request();
        req.eye = ["left_eye", "right_eye"];
        req.state = [true, true];
        self.set_focus_state.call_async(req);
        time.sleep(1.0);
    }
}
/// faire en sorte que
/// ```python
/// a,b = a,b
/// try{
/// print("hi")}
/// except KeyError{
/// print("missed")}
/// command = {"left": {}, "right": {}};
/// ```
/// soit remplacé par
/// ```python
/// (a,b) = (a,b)
/// // try
/// // print("hi")
/// // except KeyError
/// // print("missed")
/// command = vec!{"left": {}, "right": {}};
/// ```
fn main() {
    /// Run main loop.
    rclpy.init(args = args);
    zoom_controller_service = ZoomControllerService();
    zoom_controller_service.start_autofocus();
    rclpy.spin(zoom_controller_service);
    rclpy.shutdown();
}
// if __name__ == "__main__"{
//     main()}
