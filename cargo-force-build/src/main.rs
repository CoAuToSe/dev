// <<<<<<< Updated upstream
// // <<<<<<< HEAD
// // use std::env;

// // fn main() {
// //     println!("Hello, world! (WIP)");
// //     // need to get the env for the folder "target"
// //     // need to clean partialy the target dir with default value being all level available (debug, release, ..)
// //     //// possibilities to choose the level to clear
// //     // then run the "build" command
// // =======
// // use std::{
// //     env::{self, *},
// //     path::{self, *},
// // };

// // fn main() {
// //     println!("Hello, world! (WIP)");
// //     // need to get the env for the folder "target"
// //     // need to clean partialy the target dir with default value being all level available (debug, release, ..)
// //     //// possibilities to choose the level to clear
// //     // then run the "build" command
// // }
// use std::path::PathBuf;
// use structopt::StructOpt;

// #[derive(Debug, StructOpt)]
// struct Opts {
//     #[structopt(parse(from_os_str))]
//     infile: PathBuf,

//     #[structopt(short, long, parse(from_os_str))]
//     outfile: Option<PathBuf>,
//     // >>>>>>> 67d3a381b0216326540a244672070f2beb7e9286
// }

// fn main() {
//     let opts = Opts::from_args();

//     println!("{:?}", opts);
// =======
// use std::env::{self, *};

#[path = "debug.rs"]
mod debug;
use crate::debug::debug::test_debug;

fn main() {
    println!("Hello, world! (WIP)");
    let args = std::env::args();
    println!("{:?}", args);
    let args_os = std::env::args_os();
    println!("{:?}", args_os);
    debug_assert!(
        test_debug(&args, &args_os),
        "(WIP) don't know what is the difference between args and args_os"
    );
    let mut args = vec![];
    for arg in std::env::args() {
        print!("{}\n", arg);
        args.push(arg)
    }
    println!("{:?}", args);
    let path_exe = std::path::Path::new(&args[0]);
    // println!("{:?}", path_exe);
    // println!("{:?}", path_exe.as_os_str());
    // println!("{:?}", path_exe.ancestors());
    // println!("{:?}", path_exe.canonicalize());
    println!("{:?}", path_exe.components());
    for dir in path_exe.components() {
        let _somea = std::ffi::OsStr::new("debug");
        match dir {
            std::path::Component::Normal(_somea) => {
                println!("lola")
            }
            _ => (),
        }
        // if let std::path::Component::Normal(testing) = dir {
        // println!("some {:?}", testing);}
        // if let std::path::Component::Normal(name_dir @ _somea) = dir {
        //     println!("lol {:?}", name_dir);
        // } else {
        //     panic!(
        //         "There is an anomaly in the way that your directories are recognized by std::path"
        //     )
        // }
        println!("{:?}", dir)
    }
    // need to get the env for the folder "target"
    // need to clean partialy the target dir with default value being all level available (debug, release, ..)
    //// possibilities to choose the level to clear
    // then run the "build" command
    // >>>>>>> Stashed changes
}
