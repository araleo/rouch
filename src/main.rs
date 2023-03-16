use std::{
    env,
    fs::{create_dir_all, File},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    check_args(&args);
    let filepath = &args[1].replace('\\', "/").replace("//", "/");
    let elements = split_filepath_and_filename(filepath);
    let dir_path = handle_dirs(&elements[0]);
    handle_file(&elements[1], dir_path);
}

fn check_args(args: &Vec<String>) {
    if args.len() != 2 {
        eprintln!("USAGE: rouch <path>");
        process::exit(1);
    }
}

fn split_filepath_and_filename(filepath: &str) -> Vec<String> {
    if let Some(last_sep) = filepath.rfind('/') {
        let dir = &filepath[..last_sep];
        let filename = &filepath[last_sep + 1..];
        return [dir.to_string(), filename.to_string()].to_vec();
    }
    [".".to_string(), filepath.to_string()].to_vec()
}

fn handle_dirs(dir: &String) -> &Path {
    let dir_path = Path::new(dir);
    if !dir_path.exists() {
        create_intermediate_dirs(dir_path);
    }
    dir_path
}

fn create_intermediate_dirs(dir_path: &Path) {
    let result = create_dir_all(dir_path);
    if let Err(e) = result {
        eprintln!("ERROR: Could not create dir {dir_path:?}: {e}.");
        process::exit(2);
    };
}

fn handle_file(filename: &String, dir_path: &Path) {
    let filepath = dir_path.join(filename);
    create_file(&filepath);
}

fn create_file(filepath: &Path) {
    let result = File::create(filepath);
    if let Err(e) = result {
        eprintln!("ERROR: Could not create file {filepath:?}: {e}.");
        process::exit(3);
    };
}
