extern crate mustache;
extern crate walkdir;
extern crate getopts;

use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use std::env;
use std::path::Path;

use mustache::Data;
use mustache::MapBuilder;

fn main() {
    use walkdir::WalkDir;

    let environment = get_environment_data();
    let (target, file_type) = get_config();

    println!("Processing {}...", target);

    let is_dir = match File::open(&*target) {
        Ok(file) => file.metadata().unwrap().is_dir(),
        Err(why) => panic!(why.to_string())
    };

    if is_dir {
        for entry in WalkDir::new(&*target) {
            let entry = entry.unwrap();
            let path = entry.path();

            if !entry.metadata().unwrap().is_dir() && type_matches(&*file_type, path) {
                let file_path = path.to_str().unwrap();
                println!("path => {}", file_path);

                match read_file(&*file_path) {
                    Ok(file_data) => render(file_data, file_path, &environment),
                    Err(_) => println!("Skipping file {}", file_path),
                }
            }
        }
    } else {
        match read_file(&*target) {
            Ok(file_data) => render(file_data, &*target, &environment),
            Err(why) => panic!(why.to_string())
        };
    }
}

fn type_matches(file_type: &str, path: &Path) -> bool {
    return file_type.is_empty() || file_type == path.extension().unwrap();
}

fn get_config() -> (String, String) {
    use getopts::Options;

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("f", "file", "specific file to interpolate", "FILE");
    opts.optopt("t", "type", "set template file type", "TYPE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    let file = matches.opt_str("file").unwrap_or(".".to_owned());
    let file_type = matches.opt_str("type").unwrap_or("".to_owned());
    return (file, file_type)
}

fn get_environment_data() -> Data {
    let mut data = MapBuilder::new();

    for (key, value) in env::vars() {
        data = data.insert_str(key, value);
    }

    return data.build();
}

fn render(template: String, output_name: &str, environment: &Data) {
    use std::fs::OpenOptions;
    let template = mustache::compile_str(&*template);
    let mut output_file = OpenOptions::new().write(true).append(false).open(output_name).unwrap();

    template.render_data(&mut output_file, environment);
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
        Err(f) => { panic!(f.to_string()) }
        _ => Ok(buffer)
    }
}
