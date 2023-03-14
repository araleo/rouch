use std::{
    env,
    fs::{create_dir_all, File},
    path::Path,
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("USAGE: rouch <path>");
        process::exit(1);
    }

    let filepath = &args[1];
    let elements = split_filepath_and_filename(filepath);
    let dir = &elements[0];

    let dir_path = Path::new(dir);
    if !dir_path.exists() {
        let result = create_dir_all(dir_path);
        if let Err(e) = result {
            eprintln!("ERROR: Could not create dir {dir_path:?}: {e}.");
            process::exit(2);
        };
    }

    let filename = &elements[1];
    let filepath = dir_path.join(filename);
    let result = File::create(&filepath);
    if let Err(e) = result {
        eprintln!("ERROR: Could not create file {filepath:?}: {e}.");
        process::exit(3);
    };
}

fn split_filepath_and_filename(filepath: &str) -> Vec<String> {
    if let Some(last_sep) = filepath.rfind('/') {
        let dir = &filepath[..last_sep];
        let filename = &filepath[last_sep + 1..];
        return [dir.to_string(), filename.to_string()].to_vec();
    }
    [".".to_string(), filepath.to_string()].to_vec()
}
