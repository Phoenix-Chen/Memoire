mod bookmark;

use std::{
    fs::{File, create_dir_all, read_to_string},
    path::Path,
    process
};
use std::io::prelude::*;
use bookmark::Bookmark;


#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Collection {
    dir_path: String,
    name: String
}


impl Collection {
    pub fn get_or_create(dir_path: &str, collection_name: &str) -> Collection {
        let collection_path: String = make_collection_path(dir_path, collection_name);
        if !Path::new(&collection_path).exists() {
            create_collection_json(&collection_path);
        }
        Collection {
            dir_path: dir_path.to_string(),
            name: collection_name.to_string()
        }
    }

    fn get_bookmarks(&self) -> Vec<Bookmark> {
        let collection_path: String = make_collection_path(&self.dir_path, &self.name);
        match serde_json::from_str(&read_file_as_string(&collection_path)) {
            Ok(v) => v,
            Err(_err) => {
                println!("Unable to parse json: {:?}", &collection_path);
                process::exit(0);
            }
        }
    }
}


// Should this be in Collection?
fn make_collection_path(dir_path: &str, collection_name: &str) -> String {
    format!("{}/{}.json", dir_path, collection_name)
}


fn read_file_as_string(path: &str) -> String {
    match read_to_string(&path) {
        Ok(data) => data,
        Err(_err) => {
            println!("Unable to read file: {:?}", &path);
            process::exit(0);
        }
    }
}



