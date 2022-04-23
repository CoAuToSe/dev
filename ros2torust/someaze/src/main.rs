use std::io::{self, Write};
use std::process::*;
fn main() {
    // println!("Hello, world!"); =>
    // let output = Command::new("cmd").arg("/C echo %PATH%").output();
    // println!("{:?}", output);
    // // use std::process::{Command, Stdio};

    // let output = Command::new("cmd")
    //     .arg("echo")
    //     .arg("Hello, world!")
    //     .stdout(Stdio::piped())
    //     .output()
    //     .expect("Failed to execute command");

    // println!("{:?}", output);
    // // assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
    // // Nothing echoed to console

    // // use std::process::{Command, Stdio};

    // let output = Command::new("cmd")
    //     .arg("/C")
    //     .arg("echo")
    //     .arg("Hello, world!")
    //     .stdout(Stdio::inherit())
    //     .output()
    //     .expect("Failed to execute command");

    // assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    // use std::io::Write;
    // use std::process::{Command, Stdio};

    // let mut child = Command::new("cmd")
    //     .arg("/C")
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to spawn child process");

    // let mut stdin = child.stdin.take().expect("Failed to open stdin");
    // // stdin
    // //     .write_all("echo Hello, world!".as_bytes())
    // //     .expect("Failed to write to stdin");
    // std::thread::spawn(move || {
    //     stdin
    //         .write_all("echo Hello, world!".as_bytes())
    //         .expect("Failed to write to stdin");
    // });
    // // .join()
    // // .unwrap();

    // let output = child.wait_with_output().expect("Failed to read stdout");
    // println!("{:?}", String::from_utf8_lossy(&output.stdout));

    // let mut child = Command::new("cmd")
    //     // .arg("/C")
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to spawn child process");

    // let mut stdin = child.stdin.take().expect("Failed to open stdin");
    // std::thread::spawn(move || {
    //     // println!(
    //     //     "{:?}",
    //     stdin
    //         .write_all("echo Hello, world!".as_bytes())
    //         .expect("Failed to write to stdin")
    //     // );
    // });
    // std::thread::sleep(std::time::Duration::new(1, 0));
    // let output = child.wait_with_output().expect("Failed to read stdout");
    // //assert_eq!(String::from_utf8_lossy(&output.stdout), "!dlrow ,olleH");
    // println!("{:?}", String::from_utf8_lossy(&output.stdout));
    //     use std::process::{Command, Stdio};

    // let output = Command::new("cmd")
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::inherit())
    //     .output()
    //     .expect("Failed to execute command");
    let mut output2 = Command::new("cmd")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    // print!("You piped in the reverse of: ");
    // io::stdout().write_all(&output.stdout).unwrap();
    // let toa = &output;
    let mut azrzae = &output2.stdin.take().unwrap();
    azrzae
        .write_all("Hello, world!".as_bytes())
        .expect("Failed to write to stdin");
}
