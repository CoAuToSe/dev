// #![no_implicit_prelude]
// extern crate core;
// extern crate std as other_std;

// use other_std::println;

fn main() {
    println!("Hello World!");
}

// use std::{
//     cmp::{max, min},
//     env::args,
//     fmt::{self, Display},
//     fs::File,
//     io::{Read, Write},
//     ops::{Deref, DerefMut},
// };

// fn main() {
//     let args = args();
//     if args.len() <= 1 {
//         panic!("wrong usage, you need to insert a file at the end of the command")
//     }
//     let mut file_name = args.into_iter().next_back().unwrap();
//     println!("{:?}", file_name);
//     let mut buf = vec![];
//     let len;
//     {
//         let mut file = File::open(&file_name).unwrap();
//         println!("{:?}", file);
//         len = file.read_to_end(&mut buf).unwrap();
//     }
//     let mut elif = FuckStd::from(String::from_utf8(buf).unwrap());
//     elif.push_str("\n\n");
//     println!("{}\nEOF\n\n {}", elif, len);
//     // let identifier = "def ";
//     // let replacer = "fn ";
//     // let mut reutrn = FuckStd::from(String::from(""));
//     elif.replace_all("\t", "    ");
//     // elif.for_all_comment("\"\"\"", "\"\"\"");
//     elif.replace_all("\"\"\"", "/// ");
//     // elif.until_it_does_not_exist("//////", "///");
//     elif.push_str("\n\n");
//     elif.replace_all("'", "\"");
//     elif.replace_all("#", "//");
//     elif.replace_all("True", "true");
//     elif.replace_all("False", "false");
//     elif.replace_all("args=None", "");
//     elif.until_it_does_not_exist("\r\n\r\n", "\r\n");
//     elif.replace_all("\r\n", ";\r\n");
//     elif.replace_all(":;", ":");
//     elif.replace_all(",;", ",");
//     elif.replace_all(".Request", "_Request");
//     elif.replace_all(".Response", "_Response");
//     elif.replace_all("import ", "// import ");
//     elif.replace_all("from ", "// from ");
//     elif.replace_all("class ", "impl ");
//     elif.replace_all("-> None", "-> ()");
//     elif.replace_all("(;", "(");
//     elif.replace_all("{;", "{");
//     elif.replace_all("__init__", "new");
//     elif.replace_all("super()", "// super()");
//     elif.until_it_does_not_exist("(Node)", "");
//     elif.replace_all(",\r\n", ",");
//     elif.replace_all("(\r\n", "(");
//     elif.replace_all("{\r\n", "{");
//     elif.replace_all("\r\n    ", "\r\n^$^");
//     elif.until_it_does_not_exist("^$^    ", "^$^^$^");
//     elif.until_it_does_not_exist("  ", " ");
//     elif.for_all_comment("///", "/// ");
//     elif.until_it_does_not_exist("^$^", "    ");
//     elif.replace_all("/// /// ", "/// ");
//     elif.replace_all("\r\n/// ;", "");
//     elif.replace_all("/// ;", "///");
//     elif.replace_all("///\r\n", "\r\n");
//     elif.replace_all("\r\n/// \r\n", "\r\n");
//     elif.for_all_lines("    ", "    ");
//     elif.replace_all(":{", "{");
//     elif.until_it_does_not_exist("\r\n\r\n", "\r\n");
//     elif.replace_all(
//         "if __name__ == \"__main__\"{",
//         "// if __name__ == \"__main__\"{",
//     );
//     elif.replace_all("    main()", "//     main()");
//     elif.replace_all("self.logger =", "// self.logger =");
//     elif.replace_all("while(1)", "loop");
//     elif.patern_matching(
//         vec!["time.sleep(", ")"],
//         vec![
//             "std::thread::sleep(std::time::Duration::from_millis((1000. *",
//             " as f64) as u64))",
//         ],
//     );
//     elif.patern_matching(vec!["raise ", ";"], vec!["panic!(r#\"", "\"#);"]);
//     elif.patern_matching(vec!["", "f\"", "\";"], vec!["", "format!(r#\"", "\"#);"]);

//     // elif.patern_matching(
//     //     vec!["self.logger.info(f\"", "\"{self.", ".srv_name}\"", "\");"],
//     //     vec!["println!(\"[info] {}{}{} \", \"", "\",", ",\"", "\");"],
//     // );
//     elif.patern_matching(
//         vec!["self.logger.info(", ");"],
//         vec!["println!(\"{}\", r#\"[info] ", "\"#);"],
//     );
//     elif.patern_matching(
//         vec!["self.logger.warning(\"", "\");"],
//         vec!["eprintln!(\"{}\", r#\"[WARNING] ", "\"#);"],
//     );
//     elif.patern_matching(
//         vec!["if", "not in", "{"],
//         vec!["if !inside(", ",vec!", "){"],
//     );
//     elif.patern_matching(vec!["if", " in ", "{"], vec!["if inside(", ",vec!", "){"]);
//     elif.patern_matching(vec!["for", " in (", "){"], vec!["for", " in vec!(", "){"]);
//     elif.patern_matching(vec!["for", " in [", "]{"], vec!["for", " in vec![", "]{"]);
//     elif.patern_matching(vec!["for", " in {", "}{"], vec!["for", " in vec!{", "}{"]);

//     // .create_subscription( CompressedImage, "left_image", partial(self.on_image_update, side="left"), 1, );
//     // .create_subscription::<CompressedImage, _>(
//     //     "right_image",
//     //     rclrs::QOS_PROFILE_DEFAULT,
//     //     move |msg: CompressedImage| {
//     //         Self::on_image_update(&am_current_eye, Side::Right(msg));
//     //     },
//     // )
//     // .unwrap());
//     elif.patern_matching(
//         vec!["create_subscription( ", ", \"", "\", ", "), ", ");"],
//         vec![
//             "create_subscription::<",
//             ", _>(\"",
//             "\", rclrs::QOS_PROFILE_DEFAULT, move |msg| {",
//             ")}).unwrap() ;//",
//             "",
//         ],
//     );
//     // .create_service( GetCameraZoomLevel, "get_camera_zoom_level", self.get_zoom_level_callback, );
//     // .create_service::<SetFocusState, _>(
//     //     "set_focus_state",
//     //     move |_request_header: &rclrs::rmw_request_id_t,
//     //           request: SetFocusState_Request|
//     //           -> SetFocusState_Response {
//     //         Self::set_focus_state_callback(&am_left_eye, &am_right_eye, request)
//     //     },
//     // )
//     // .unwrap();
//     elif.patern_matching(
//         vec!["create_service(", ", \"", "\",", ", );"],
//         vec![
//             "create_service::<",
//             ", _>(\"",
//             "\",move |_request_header,request| {",
//             "}).unwrap();",
//         ],
//     );
//     // .create_client(SetFocusState, "set_focus_state");
//     // .create_client::<SetCameraZoomFocus>("set_camera_zoom_focus")
//     // .unwrap();
//     elif.patern_matching(
//         vec!["create_client(", ", \"", "\", );"],
//         vec!["create_client::<", ">(\"", "\");"],
//     );
//     // .create_publisher(CompressedImage, 'left_image', 1)
//     // .create_publisher::<CompressedImage>(topic, rclrs::QOS_PROFILE_DEFAULT)
//     // .unwrap();
//     elif.patern_matching(
//         vec!["create_publisher(", ", \"", "\",", ");"],
//         vec![
//             "create_publisher::<",
//             ">(\"",
//             "\",/*",
//             "*/rclrs::QOS_PROFILE_DEFAULT).unwrap();",
//         ],
//     );
//     // elif.replace_all(";\r\n ", ";\r\n? ");
//     // elif.replace_all("\r\n    ", "\r\n^$^");
//     // elif.until_it_does_not_exist("^$^    ", "^$^^$^");
//     // elif.replace_all("    ", "{}");
//     // elif.replace_all("", "");
//     // elif.replace_all("", "");
//     // elif.replace_all("", "");
//     // elif.replace_all("self.logger.info(f\"", "println!(\"");//need patern matching
//     // elif.replace_all("", "");
//     // elif.replace_all("", "");
//     // elif.replace_all("", "");
//     elif.replace_all("def ", "fn ");
//     elif.replace_all("elif ", "else if ");
//     elif.replace_all(" str ", " &str ");
//     elif.replace_all(":str", ":&str");
//     elif.replace_all(" str)", " &str)");
//     elif.replace_all(" str,", " &str,");
//     elif.replace_all(" str;", " &str;");
//     elif.replace_all(" int ", " isize ");
//     elif.replace_all(":int", ":isize");
//     elif.replace_all(" int)", " isize)");
//     elif.replace_all(" int,", " isize,");
//     elif.replace_all(" int;", " isize;");
//     elif.replace_all("print(", "println!(");
//     elif.replace_all(" or ", " || ");
//     elif.replace_all(" and ", " && ");
//     elif.replace_all(" not ", " !");
//     elif.replace_all("= {", "= dict!{");
//     elif.replace_all("={", "=dict!{");

//     println!("{elif}EOF");
//     // println!("{reutrn}");file_name
//     let mut out_name = String::from("src/");
//     file_name.pop();
//     file_name.pop();
//     file_name.pop();
//     out_name.push_str(&file_name);
//     out_name.push_str(".rs");
//     let mut file = File::create(out_name).expect("Error encountered while creating file!");
//     file.write_all(elif.as_bytes())
//         .expect("Error while writing to file");
// }

// #[derive(Debug, Clone)]
// struct FuckStd {
//     string: String,
//     // num: u32,
// }
// impl Display for FuckStd {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.string)
//     }
// }

// impl From<String> for FuckStd {
//     fn from(string: String) -> Self {
//         FuckStd { string }
//     }
// }

// impl Deref for FuckStd {
//     type Target = String;
//     fn deref(&self) -> &Self::Target {
//         &self.string
//     }
// }

// // impl Deref for FuckStd {
// //     type Target = u32;
// //     fn deref(&self) -> &Self::Target {
// //         &self.num
// //     }
// // }

// impl DerefMut for FuckStd {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.string
//     }
// }

// impl FuckStd {
//     #[deprecated]
//     fn _replace_first(&mut self, identifier: &str, replacer: &str) {
//         let pos = self.find(identifier).unwrap();
//         for _ in 0..identifier.len() {
//             self.remove(pos);
//         }
//         for e in replacer.chars().rev().enumerate() {
//             self.insert(pos, e.1);
//         }
//     }

//     fn replace_all(&mut self, identifier: &str, replacer: &str) {
//         let mut reutrn = FuckStd::from(String::from(""));
//         let mut az = self.split(identifier);
//         while let Some(some) = az.next() {
//             reutrn.push_str(some);
//             reutrn.push_str(replacer);
//         }
//         for _ in 0..replacer.len() {
//             reutrn.pop();
//         }
//         *self = reutrn;
//     }
//     fn until_it_does_not_exist(&mut self, identifier: &str, replacer: &str) {
//         let mut reutrn1 = self.clone();
//         while let Some(_) = reutrn1.find(identifier) {
//             let mut reutrn = FuckStd::from(String::from(""));
//             let mut az = reutrn1.split(identifier);
//             while let Some(some) = az.next() {
//                 reutrn.push_str(some);
//                 reutrn.push_str(replacer);
//             }
//             for _ in 0..replacer.len() {
//                 reutrn.pop();
//             }
//             reutrn1 = reutrn;
//         }
//         *self = FuckStd::from(reutrn1);
//     }

//     fn for_all_lines(&mut self, identifier: &str, _replacer: &str) {
//         let mut reutrn = FuckStd::from(String::from(""));
//         let mut last_counter = 1;
//         for e in self.lines() {
//             let mut counter = 0;
//             let mut az = e.split(identifier);
//             while let Some(_some) = az.next() {
//                 counter += 1
//             }
//             if last_counter < counter {
//                 for _ in min(last_counter, counter)..max(last_counter, counter) {
//                     reutrn.push_str("{");
//                 }
//             }
//             if last_counter > counter {
//                 for _ in min(last_counter, counter)..max(last_counter, counter) {
//                     reutrn.push_str("}");
//                 }
//             }
//             reutrn.push_str("\r\n");
//             // reutrn.push_str(format!("{}", counter).as_str());
//             reutrn.push_str(e);
//             last_counter = counter;
//         }
//         *self = reutrn;
//     }
//     fn for_all_comment(&mut self, identifier: &str, replacer: &str) {
//         let mut reutrn = FuckStd::from(String::from(""));
//         let mut inside = false;
//         for e in self.lines() {
//             let mut counter = 0;
//             let mut az = e.split(identifier);
//             while let Some(_some) = az.next() {
//                 counter += 1
//             }
//             if counter == 2 && !inside {
//                 inside = true
//             } else {
//                 if counter == 2 && inside {
//                     inside = false
//                 }
//             }
//             reutrn.push_str("\r\n");
//             if inside {
//                 let mut inner_az = e.split("^$^");
//                 let mut inner_counter = 0;
//                 while let Some(_) = inner_az.next() {
//                     inner_counter += 1;
//                 }
//                 for _ in 1..inner_counter {
//                     reutrn.push_str("^$^");
//                 }
//                 reutrn.push_str(replacer);
//                 reutrn.push_str(&e.replace("^$^", ""));
//             } else {
//                 // reutrn.push_str(format!("{}", counter).as_str());
//                 reutrn.push_str(e);
//             }
//             if inside {
//                 reutrn.pop();
//             }
//         }
//         *self = reutrn;
//     }
//     fn patern_matching(
//         &mut self,
//         identifiers: Vec<&str>,
//         replacers: Vec<&str>,
//         // what_do_we_do: &dyn Fn(Vec<&str>, Vec<&str>) -> String,
//     ) {
//         // for e in self.lines() {
//         //     lib::scanner!(e);
//         // }
//         assert_eq!(identifiers.len(), replacers.len());
//         let mut reutrn = FuckStd::from(String::from(""));
//         let mut temp_line;
//         let mut current_line;
//         for line in self.lines() {
//             temp_line = line.clone();
//             current_line = String::from(line.clone());
//             let mut success = true;
//             'forteen: for i in 0..identifiers.len() {
//                 if let Some(pos) = current_line.find(identifiers[i]) {
//                     for _ in 0..identifiers[i].len() {
//                         current_line.remove(pos);
//                     }
//                     for e in replacers[i].chars().rev().enumerate() {
//                         current_line.insert(pos, e.1);
//                     }
//                 } else {
//                     success = false;
//                     break 'forteen;
//                 }
//             }
//             if success {
//                 // panic!();
//                 reutrn.push_str(current_line.as_str());
//                 reutrn.push_str("\r\n");
//             } else {
//                 reutrn.push_str(temp_line);
//                 reutrn.push_str("\r\n");
//             }
//         }
//         *self = reutrn;
//     }
// }
