extern crate dirs;

use std::{
    fs::{File, create_dir_all},
    process,
    io::prelude::*
};


// lazy_static! {
//     pub static ref COLLECTION_DIR_PATH: String = get_path_from_home_dir(COLLECTION_DIR_NAME);
//     pub static ref DEFAULT_JSON_PATH: String = format!("{}/{}", &COLLECTION_DIR_PATH, DEFAULT_FILE_NAME);
// }
static COLLECTION_DIR_NAME: &str = ".memoire";
pub static DEFAULT_JSON_NAME: &str = "default";


// TODO: Change from function to variable
pub fn get_collection_dir_path() -> String {
    get_full_path(COLLECTION_DIR_NAME)
}


pub fn get_json_path(collection_name: &str) -> String {
    format!("{}/{}.json", &get_collection_dir_path(), collection_name)
}


fn get_full_path(relative_path: &str) -> String {
    match dirs::home_dir() {
        Some(home_dir) => {
            let mut full_path = home_dir.into_os_string();
            full_path.push("/");
            full_path.push(relative_path);
            match full_path.to_str() {
                Some(s) => {
                    s.to_owned()
                },
                None => {
                    panic!("Unable to convert OS string to &str")
                }
            }
        },
        None => {
            panic!("Unable to find $HOME")
        }
    }
}


pub fn create_collection_dir(path: &str) {
    match create_dir_all(path) {
        Ok(_) => {},
        Err(_err) => {
            println!("Unable to create directory: {}", path);
            process::exit(0);
        }
    }
}


pub fn write_to_json(json_path: &str, content: Option<&str>) {
    match File::create(json_path) {
        Ok(mut file) => {
            // Default content is empty list
            match file.write_all(content.unwrap_or("[]").as_bytes()) {
                Ok(_) => {},
                Err(_err) => {
                    println!("Unable to write to file: {}", json_path);
                    process::exit(0);
                }
            }
        },
        Err(_err) => {
            println!("Unable to create file: {}", json_path);
            process::exit(0);
        }
    }
}