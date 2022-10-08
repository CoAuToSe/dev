
/// Python Package for controlling the zoom of Kurokesu"s camera.
// import serial;
// import time;
impl ZoomController{
    /// Zoom controller class.
    // connector = { // "left": "J1", // "right": "J2";
    // };
    // motors = { // "J1": {"zoom": "X", "focus": "Y"}, // "J2": {"zoom": "Z", "focus": "A"};
    // };
    // zoom_pos = { // "left": { // "in": {"zoom": 457, "focus": 70}, // "inter": {"zoom": 270, "focus": 331}, // "out": {"zoom": 60, "focus": 455}, // }, // "right": { // "in": {"zoom": 457, "focus": 42}, // "inter": {"zoom": 270, "focus": 321}, // "out": {"zoom": 60, "focus": 445}, // }, // };
    // fn new( // self, // port: &str = "/dev/ttyACM0", // baudrate: isize = 115200, // timeout: isize = 10, // speed: isize = 10000) -> ():
    fn new( self, port: &str, baudrate: isize , timeout: isize, speed: isize) -> (){
        /// Connect to the serial port && run the initialisation sequence.
        self.ser = serial.Serial(port=port, baudrate=baudrate, timeout=timeout);
        self.speed = speed;
        init_seq = "G100 P9 L144 N0 S0 F1 R1";
        self.ser.write(bytes(init_seq + "\n", "utf8"));
        response = self.ser.readline();
        if response.decode() != "ok\r\n"{
            panic!(r#"IOError("Initialization of zoom controller failed, check that the control board is correctly plugged in.")"#);}}
    fn set_zoom_level(self, side: &str, zoom_level: &str) -> (){
        /// Set zoom level of a given camera.
        /// Given the camera side && the zoom level required, produce the corresponding G-code && send it over the serial port.
        /// Args{
            /// side: "right" || "left"
            /// zoom_level: either "in", "inter" || "out". "in" level for far{
                ///  objects, "out" for close objects, "inter" is in between
                ///  "in" && "out" levels}}
        
        (zoom, focus) = self.zoom_pos[side][zoom_level].values();
        self._send_custom_command(vec!{side: {"zoom": zoom, "focus": focus}});}
    fn _send_custom_command(self, commands: dict){
        /// Send custom command to camera controller.
        /// Args{
            /// commands: dictionnary containing the requested camera name along
            /// with requested focus && zoom value. Instructions for both cameras
            /// can be sent in one call of this method. However, instructions will
            /// be sent sequentially && there is no synchronization.}
        
        for (side, cmd) in commands.items(){
            if !inside( side ,vec! ["left", "right"]){
                panic!(r#"ValueError("Keys should be either "left" || "right".")"#);}
            motor = self.motors[self.connector[side]];
            for (target, value) in cmd.items(){
                if !inside( target ,vec! ["zoom", "focus"]){
                    panic!(r#"ValueError("Each command should be either on "focus" || "zoom".")"#);}
                command = format!(r#"G1 {motor[target]}{value} F{self.speed}"#);
                self.ser.write(bytes(command + "\n", "utf8"));
                _ = self.ser.readline();}}}
    fn homing(self, side: &str) -> (){
        /// Use serial port to perform homing sequence on given camera.
        /// Args{
            /// side: "right", "left".}
        
        mot = self.motors[self.connector[side]];
        cmd = "G92 " + mot["zoom"] + "0 " + mot["focus"] + "0";
        self.ser.write(bytes(cmd + "\n", "utf8"));
        _ = self.ser.readline();
        std::thread::sleep(std::time::Duration::from_millis((1000. *0.1 as f64) as u64));
        self._send_custom_command(vec!{side: {"zoom": 0, "focus": -500}});
        std::thread::sleep(std::time::Duration::from_millis((1000. *1 as f64) as u64));
        self._send_custom_command(vec!{side: {"zoom": -600, "focus": -500}});
        std::thread::sleep(std::time::Duration::from_millis((1000. *1 as f64) as u64));
        cmd = "G92 " + mot["zoom"] + "0 " + mot["focus"] + "0";
        self.ser.write(bytes(cmd + "\n", "utf8"));
        _ = self.ser.readline();
        std::thread::sleep(std::time::Duration::from_millis((1000. *0.1 as f64) as u64));}
    fn set_speed(self, speed_value: isize) -> (){
        /// Set motors speed.
        /// Args{
            /// speed_value: isize between 4000 && 40000}
        
        if !(4000 <= speed_value && speed_value <= 40000){
            panic!(r#"ValueError("Speed value must be between 4000 && 40000")"#);}
        self.speed = speed_value}}
