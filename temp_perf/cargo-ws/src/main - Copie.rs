#![allow(irrefutable_let_patterns)]
use std::{
    cmp::{self, *},
    convert::TryFrom,
    env::{self, *},
    fs::{self, *},
    io::{self, *},
    path::{self, *},
    str::{self, *},
    vec,
};

use log::*;

use env_logger::Env;

struct FileStatus {
    dir_path: String,
    deepness: usize,
}
enum State {
    Help,
    Init(String),
    Run(String),
    New(String),
    Build,
    Update,
    None,
}

#[test]
fn lol() {
    // println!("{:?}", fs::read_dir(path::Path::new("./")));
    // println!("{:?}", path::Path::new("./"));
    let mut mydir = path::Path::new("./").canonicalize();
    println!("{:?}", mydir);
    // println!("{:?}", mydir.display());
    // println!("{:?}", fs::read_dir(&mydir));

    // println!("{:#?}", path::Path::new("./").metadata());
    // println!("{:?}", path::Path::new("./").components());
    // let mut emp = path::Path::new("./");
    // let components: Vec<_> = emp.components().map(|comp| comp.as_os_str()).collect();
    // println!("{:?}", components);

    // println!("{:?}", path::Path::new("./").display());
    // println!("{:?}", path::Path::new("./").to_path_buf());
    // println!("{:?}", path::Path::new("./").parent());
}
// use std::fs;
#[test]
fn mazeaain() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())
}

// const EXCLUDE_DIR: [&str; 2] = [".git", "target"]; // may take more time to run
const EXCLUDE_DIR: [&str; 3] = [".git", "src", "target"];

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let mut args = env::args();
    println!("{:?}", args);
    if let Some(todo) = args.nth(2) {
        match todo.as_str() {
            "" | "-h" | "--help" => {
                info!("help me senpai");
            }
            "i" | "init" => {
                info!("init");
                let mut directories = vec![] as Vec<DirEntry>;
                let mut programms = vec![] as Vec<PathBuf>;
                analyse_dir(
                    // Path::new("."),
                    env::current_dir().unwrap().as_path(),
                    &mut directories,
                    &mut programms,
                    &EXCLUDE_DIR,
                );
                while directories.len() != 0 {
                    // info!("while working");
                    // std::thread::sleep(std::time::Duration::new(1, 0));
                    for _ in 0_usize..directories.len() {
                        let patoloh = &directories.pop().unwrap().path();
                        analyse_dir(patoloh, &mut directories, &mut programms, &EXCLUDE_DIR);
                    }
                }
                info!(
                    "there is {:?} program founded: {:#?}",
                    programms.len(),
                    programms
                );
                let mut list_progrs_package = vec![];
                let mut list_progrs_workspace = vec![];
                // let mut list_progrs_not_to_modif = vec![];
                let mut list_progrs_package_to_modif = vec![];

                // let mut vec_possible_workspace = vec![];
                // let mut vec_workspace_remove = vec![];
                for e in programms {
                    let temp_val = e.clone();
                    let folder_path = temp_val.as_path();
                    let file_path_string = folder_path.join("Cargo.toml");
                    let file_toml = file_path_string.as_path();
                    let mut opened_file = File::open(file_toml).unwrap();
                    info!("currently studying: {:?}", file_toml.display());
                    let mut content = String::new();
                    let len_content = opened_file.read_to_string(&mut content).unwrap();
                    // println!("With text:\n{}", content);
                    // let mut ele1 = ;
                    let mut file_identified = 1;
                    for ligne in content.lines() {
                        // debug!("ligne: {:?}", ligne);
                        let vec_composent_ligne = {
                            let mut tempo_vec = vec![];
                            // ligne.split("workspace").into_iter()
                            for element in ligne.split("workspace").into_iter() {
                                tempo_vec.push({
                                    // let mut tempo_vec2: Vec<&str> = vec![];
                                    // ligne.split("workspace").into_iter()

                                    let mut tema = vec![];
                                    for element2 in element.split_whitespace().into_iter() {
                                        if element2 != "" {
                                            tema.push(element2)
                                        }
                                    }
                                    // while let Some(whitespace_thing) = somez.next() {
                                    //     tempo_vec2.push(whitespace_thing)
                                    // }
                                    tema.join(" ")
                                })
                            }
                            tempo_vec
                        };

                        // debug!("splited lignes : {:#?}", vec_composent_ligne);
                        if vec_composent_ligne.len() != 1 {
                            // trace!("OMFG");
                            // let mut done = false;
                            if vec_composent_ligne[0] == "" {
                                // info!("it's workiong");
                                list_progrs_package.push(e.clone());
                                file_identified *= 2;
                            }
                            if vec_composent_ligne[0] == "[" {
                                // info!("it's a workiong");
                                list_progrs_workspace.push(e.clone());
                                file_identified *= 3;
                            }
                            // if vec_composent_ligne[0].contains("#")
                            // // && !(list_progrs_not_to_modif.contains(&e))
                            // {
                            //     info!("it's a trap");
                            //     // list_progrs_not_to_modif.push(e.clone());
                            //     // file_identified *= 5;
                            // }
                        }
                    }
                    // debug!("wtf {} (to modif {})", file_identified, content);
                    if file_identified == 1 {
                        // warn!("to modify {} {}", file_identified, !(file_identified));
                        list_progrs_package_to_modif.push(e.clone());
                        list_progrs_package.push(e.clone());
                    }
                    // {
                    //     // needed because would need a scanner of "toml" files to know what is a comment and what is not (WIP)
                    //     if content.split("# [workspace]").next().unwrap() != content.as_str() {
                    //         // this "commented workspace" is to remove from the list of "interacting object" after
                    //         vec_workspace_remove.push(e.clone());
                    //         println!(
                    //         "You have a commented \" [workspace]\" in your file at: {:?}; remove it (WIP features)",
                    //         file_toml.display()
                    //     )
                    //     }
                    //     if content.split("#[workspace]").next().unwrap() != content.as_str() {
                    //         // this "commented workspace" is to remove from the list of "interacting object" after
                    //         vec_workspace_remove.push(e.clone());
                    //         println!(
                    //         "You have a commented \"[workspace]\" in your file at: {:?}; remove it (WIP features)",
                    //         file_toml.display()
                    //     )
                    //     }
                    // }
                    // if let balise = content.split("[workspace]\n") {
                    //     // interact with the user to get to know what he want to do with this workspace (override|destroy|panic|use as main)
                    //     vec_possible_workspace.push(e.clone());
                    //     println!(
                    //         "You have \"[workspace]\" in your file at: {:?}",
                    //         file_toml.display()
                    //     )
                    // }
                    // if let balise = content.split("[package]") {
                    //     if let second_balise = content.split("workspace") {
                    //         //workspace already implemented for this one
                    //     } else {
                    //         // need to "implement" for this one
                    //     }
                    //     //wharck if there
                    // }
                    // if let balise = content.split("[workspace]") {}
                    // if content.contains("[workspace]\n") {
                    //     println!("main workspace: {:?}", folder_path);
                    // }
                    // if content.contains("\nworkspace") {
                    //     println!("already done");
                    // } else {
                    //     // for f in 0..max(0, len_content - 10) as usize {
                    //     //     if &content[f..f + 10].split(pat) == "[package]\n" {
                    //     //         // println!(
                    //     //         //     "{} | {} | {}",
                    //     //         //     &buffe[..f],
                    //     //         //     &buffe[f..f + 9],
                    //     //         //     &buffe[f + 9..]
                    //     //         // )
                    //     //         println!("loililol: {:?}", file_path_string.as_path().display());
                    //     //         // break;
                    //     //     }
                    //     // }
                    // }
                    // let mut file = match File::create(&path) {
                    //     Err(why) => panic!("couldn't create {}: {}", display, why),
                    //     Ok(file) => file,
                    // };
                    // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
                    // match file.write_all(LOREM_IPSUM.as_bytes()) {
                    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
                    //     Ok(_) => println!("successfully wrote to {}", display),
                    // }
                    // for f in contents.lines() {
                    //     println!("With line:\n{}", f);
                    // }
                }

                // debug!(
                //     "{:#?}",
                //     vec![
                //         list_progrs_package.clone(),
                //         // list_progrs_not_to_modif.clone(),
                //         list_progrs_workspace.clone(),
                //         list_progrs_package_to_modif.clone()
                //     ]
                // );
                // let mut cloned_azd = list_progrs_not_to_modif.clone();
                // let mut vec_to_modif = vec![];
                // for e in list_progrs_package {
                //     let mut finide = false;
                //     let mut index = 0_usize;
                //     for f in cloned_azd.clone() {
                //         if e == f {
                //             finide = true;
                //             cloned_azd.remove(index);
                //         }
                //     }
                //     if !finide {
                //         vec_to_modif.push(e)
                //     }
                //     index += 1;
                // }
                info!("\n list of current packages: {:#?}", list_progrs_package);
                info!(
                    "\n list of current packages without already a workspace: {:#?}",
                    list_progrs_package_to_modif
                );
                info!(
                    "\n list of current workspaces: {:#?}",
                    list_progrs_workspace
                );
                //chack if there is only one possible workspace otherwisepanic or/and interact with user
                // println!("{:?}  | {:?}", vec_possible_workspace, vec_workspace_remove);
            }
            "r" | "run" => {
                println!("run")
            }
            "n" | "new" => {
                println!("new")
            }
            "b" | "build" => {
                println!("build")
            }
            "u" | "update" => {
                println!("update")
            }
            "t" | "test" => {
                println!("test");
                tazzer();
            }
            _ => {
                println!("LOL you dum")
            }
        }
    }
}

fn analyse_dir<'a>(
    pathe: &Path,
    dirs: &'a mut Vec<DirEntry>,
    prgs: &'a mut Vec<PathBuf>,
    exclude: &[&str],
) {
    info!("got: {:?}", pathe);
    for entry in fs::read_dir(pathe).unwrap() {
        let dir = entry.unwrap();
        let pzatzr = dir.path();
        if pzatzr.is_file() && pzatzr.ends_with("Cargo.toml") {
            // println!("end_with: {:?}", pzatzr);
            prgs.push(Box::new(pzatzr.parent().unwrap()).to_path_buf());
        }
        if dir.path().is_dir() && !(exclude.contains(&dir.file_name().to_str().unwrap())) {
            dirs.push(dir);
        }
    }
}

// #[test]
fn tazzer() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}
#[test]
fn tazazezegzer() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    // let mut to_change = File::open("test.txt").unwrap();
    let path = Path::new("test.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(b"loazdp") {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    match file.write_all(b"lop") {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
