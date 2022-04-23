#![allow(irrefutable_let_patterns)]
use std::{
    cmp::{self, *},
    convert::TryFrom,
    env::{self, *},
    error::{self, *},
    fs::{self, *},
    io::{self, *},
    net::{self, *},
    path::{self, *},
    str::{self, *},
    time::Duration,
    vec,
};

// use log::*;

// use env_logger::Env;

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

// const EXCLUDE_DIR: [&str; 2] = [".git", "target"]; // may take more time to run
const EXCLUDE_DIR: [&str; 3] = [".git", "src", "target"];

fn main() {
    // let env = Env::default()
    //     .filter_or("MY_LOG_LEVEL", "trace")
    //     .write_style_or("MY_LOG_STYLE", "always");
    // env_logger::init_from_env(env);

    let mut args = env::args();
    println!("You called ws with those args: {:?}", args);
    if let Some(todo) = args.nth(2) {
        match todo.as_str() {
            "" | "-h" | "--help" => {
                // info!("help me senpai");
                println!("help me senpai");
            }
            "i" | "init" => {
                // info!("init");
                println!("init");
                let mut directories = vec![] as Vec<(DirEntry, usize)>;
                let mut programms = vec![] as Vec<(PathBuf, usize)>;
                let mut targets_dir = vec![] as Vec<DirEntry>;
                let mut deepness = 0;
                analyse_dir(
                    env::current_dir().unwrap().as_path(),
                    &mut directories,
                    &mut programms,
                    &EXCLUDE_DIR,
                    deepness,
                    &mut targets_dir,
                );
                let mut security = 0;
                while directories.len() != security {
                    deepness += 1;
                    let zadaz = directories.len();
                    for indoa in security..zadaz {
                        security += 1;
                        let patoloh = &directories[indoa].0.path();
                        analyse_dir(
                            patoloh,
                            &mut directories,
                            &mut programms,
                            &EXCLUDE_DIR,
                            deepness,
                            &mut targets_dir,
                        );
                    }
                }
                // info!(
                //     "there is {:?} program founded: {:#?}",
                //     programms.len(),
                //     programms
                // );
                println!(
                    "there is {:?} program founded: {:#?}",
                    programms.len(),
                    programms
                );
                let mut list_progrs_package = vec![];
                let mut list_progrs_workspace = vec![];
                let mut list_progrs_package_to_modif = vec![];

                for e in programms {
                    let temp_val = e.clone();
                    let file_path_string = temp_val.0.as_path().join("Cargo.toml");
                    let file_toml = file_path_string.as_path();
                    let mut opened_file = File::open(file_toml).unwrap();
                    // info!("currently studying: {:?}", file_toml.display());
                    println!("currently studying: {:?}", file_toml.display());
                    let mut content = String::new();
                    let len_content = opened_file.read_to_string(&mut content).unwrap();
                    let mut file_identified = 1;
                    for ligne in content.lines() {
                        let vec_composent_ligne = {
                            let mut tempo_vec = vec![];
                            for element in ligne.split("workspace").into_iter() {
                                tempo_vec.push({
                                    let mut tema = vec![];
                                    for element2 in element.split_whitespace().into_iter() {
                                        if element2 != "" {
                                            tema.push(element2)
                                        }
                                    }
                                    tema.join(" ")
                                })
                            }
                            tempo_vec
                        };

                        if vec_composent_ligne.len() != 1 {
                            if vec_composent_ligne[0] == "" {
                                list_progrs_package.push(e.clone());
                                file_identified *= 2;
                            }
                            if vec_composent_ligne[0] == "[" {
                                list_progrs_workspace.push(e.clone());
                                file_identified *= 3;
                            }
                        }
                    }
                    if file_identified == 1 {
                        list_progrs_package_to_modif.push(e.clone());
                        list_progrs_package.push(e.clone());
                    }
                }
                // info!("\n list of current packages: {:#?}", list_progrs_package);
                // info!(
                //     "\n list of current packages without already a workspace: {:#?}",
                //     list_progrs_package_to_modif
                // );
                // info!(
                //     "\n list of current workspaces: {:#?}",
                //     list_progrs_workspace
                // );
                // info!("\n lol: {:#?}", targets_dir);
                println!("\n list of current packages: {:#?}", list_progrs_package);
                println!(
                    "\n list of current packages without already a workspace: {:#?}",
                    list_progrs_package_to_modif
                );
                println!(
                    "\n list of current workspaces: {:#?}",
                    list_progrs_workspace
                );
                println!("\n lol: {:#?}", targets_dir);
                let temp_path = Path::new("./Cargo2.toml");
                let dispalyer = temp_path.display();
                let mut file_workspace = match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(temp_path)
                {
                    Err(why) => panic!("couldn't create {}: {}", dispalyer, why),
                    Ok(file) => file,
                };

                match file_workspace.write_all(b"loazdp") {
                    Err(why) => panic!("couldn't write to {}: {}", dispalyer, why),
                    Ok(_) => println!("successfully wrote to {}", dispalyer),
                }
                file_workspace.sync_all().unwrap();
                std::thread::sleep(Duration::new(1, 0));
                let mut content2 = String::new();
                let len_content2 = file_workspace.read_to_string(&mut content2).unwrap();
                // let foo: SocketAddr = fs::read_to_string("Cargo2.toml").unwrap().parse().unwrap();
                let azfre = tezaag(file_workspace.path);
                // let file_workspace = OpenOptions::new()
                //     .read(true)
                //     .write(true)
                //     .create(true)
                //     .open(Path::new("./Cargo2.toml").canonicalize().unwrap());
                println!("{:?}\n{:?}\n{:?}", file_workspace, content2, azfre);
                //chack if there is only one possible workspace otherwisepanic or/and interact with user
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
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
fn tezaag(path: &Path) -> std::result::Result<SocketAddr, Box<dyn std::error::Error>> {
    let foo: SocketAddr = fs::read_to_string(path)?.parse()?;
    Ok(foo)
}

fn analyse_dir<'a>(
    pathe: &Path,
    dirs: &'a mut Vec<(DirEntry, usize)>,
    prgs: &'a mut Vec<(PathBuf, usize)>,
    exclude: &[&str],
    deepa: usize,
    targets: &'a mut Vec<DirEntry>,
) {
    // info!("got: {:?}", pathe);
    // info!("treating {:?} with a deepness of {} ", pathe, deepa);
    println!("treating {:?} with a deepness of {} ", pathe, deepa);
    // debug!("{:?} | {} | {} | {}", directories, indoa, security, zadaz);
    for entry in fs::read_dir(pathe).unwrap() {
        let dir = entry.unwrap();
        let pzatzr = dir.path();
        if pzatzr.is_file() && pzatzr.ends_with("Cargo.toml") {
            // println!("end_with: {:?}", pzatzr);
            prgs.push((Box::new(pzatzr.parent().unwrap()).to_path_buf(), deepa));
        }
        let is_dir = dir.path().is_dir();
        let dir_name = String::from(dir.file_name().to_str().unwrap());
        if is_dir {
            if exclude.contains(&dir_name.as_str()) {
                if dir_name == "target" {
                    targets.push(dir);
                }
            } else {
                dirs.push((dir, deepa));
            }
        }
    }
}

fn tazzer() {
    println!(
        "The current directory is {:?}",
        env::current_dir().unwrap().display()
    );
    println!(
        "or as I would say: {:?}",
        env::current_dir().unwrap().canonicalize().unwrap()
    );
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

#[test]
fn dir_manageement() {
    std::thread::sleep(Duration::new(5, 0));
    let path = env::current_dir().unwrap();
    println!("\nThe current directory is {}", path.display());
    // let mut to_change = File::open("test.txt").unwrap();
    let path = Path::new("./lola/test.txt");
    let display = path.display();
    println!(
        "{:?}",
        fs::create_dir_all({
            let mut azd = path.ancestors();
            azd.next().unwrap();
            azd.next().unwrap()
        })
    );

    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
    {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let aztzrg = path.canonicalize().unwrap();
    let mut adzegz = aztzrg.ancestors();
    let mut lis_paths = {
        let mut temp_vec = vec![];
        while let Some(e) = aztzrg.ancestors().next() {
            temp_vec.push(e)
        }
        temp_vec
    };
    std::thread::sleep(Duration::new(5, 0));
    // println!(
    //     "{:?} {:?} {:?}",
    // adzegz.next(),
    println!("{:?}", lis_paths);
    // println!("{:?}", fs::remove_file(lis_paths[1]));
    // println!("{:?}", fs::remove_dir(lis_paths[1]));

    std::thread::sleep(Duration::new(1, 0));
}
