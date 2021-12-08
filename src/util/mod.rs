extern crate termion;
extern crate dirs;

// use lazy_static::lazy_static;
use std::collections::HashSet;
use std::{
    fs::{File, create_dir_all, read_to_string},
    path::Path,
    process
};
use std::io::prelude::*;


static COLLECTION_DIR_NAME: &str = ".memoire";
static DEFAULT_FILE_NAME: &str = "default.json";

// TODO: Change from function to variable
pub fn get_collection_dir_path() -> String {
    get_path_from_home_dir(COLLECTION_DIR_NAME)
}

// TODO: Change from function to variable
pub fn get_default_json_path() -> String {
    format!("{}/{}", &get_collection_dir_path(), DEFAULT_FILE_NAME)
}

// lazy_static! {
//     pub static ref COLLECTION_DIR_PATH: String = get_path_from_home_dir(COLLECTION_DIR_NAME);
//     pub static ref DEFAULT_JSON_PATH: String = format!("{}/{}", &COLLECTION_DIR_PATH, DEFAULT_FILE_NAME);
// }

pub mod event;

pub enum Mode {
    Add(AddMode),
    Delete(usize, String),
    Edit(EditMode),
    Parse(String),
    Search(SearchMode)
}


pub struct EditMode {
    id: usize,
    command: String,
    annotation: String,
    tags: Vec<String>
}


impl EditMode {
    pub fn new(id: usize, command: String, annotation: String, tags: Vec<String>) -> EditMode {
        EditMode {
            id,
            command,
            annotation,
            tags
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_annotation(&self) -> &String {
        &self.annotation
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
}


pub struct AddMode {
    pub command: String,
    pub annotation: String,
    pub tags: Vec<String>
}


pub struct SearchMode {
    union: bool,
    searches: Vec<String>,
    commands: Vec<String>,
    annotations: Vec<String>,
    tags: Vec<String>
}


impl SearchMode {
    pub fn default() -> SearchMode {
        SearchMode {
            union: true,
            searches: Vec::new(),
            commands: Vec::new(),
            annotations: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn set_commands(&mut self, commands: Vec<String>) {
        self.commands = commands;
    }

    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn set_annotations(&mut self, annotations: Vec<String>) {
        self.annotations = annotations;
    }

    pub fn get_annotations(&self) -> &Vec<String> {
        &self.annotations
    }

    pub fn set_searches(&mut self, searches: Vec<String>) {
        self.searches = searches;
    }

    pub fn get_searches(&self) -> &Vec<String> {
        &self.searches
    }

    pub fn set_union(&mut self, b: bool) {
        self.union = b;
    }

    pub fn get_union(&self) -> bool {
        self.union
    }

    pub fn has_input(&self) -> bool {
        self.searches.len() + self.annotations.len() + self.commands.len() + self.get_tags().len() > 0
    }
}


pub fn multi_intersection<T: std::hash::Hash + std::cmp::Eq + std::clone::Clone>(mut v: Vec<HashSet<T>>) -> HashSet<T> {
    match v.len() {
        0 => return HashSet::new(),
        1 => return v.pop().unwrap(),
        _ => {
            let h: HashSet<T> = &v.pop().unwrap() & &v.pop().unwrap();
            v.push(h);
            return multi_intersection(v);
        }
    }
}


pub fn multi_union<T: std::hash::Hash + std::cmp::Eq + std::clone::Clone>(mut v: Vec<HashSet<T>>) -> HashSet<T> {
    match v.len() {
        0 => return HashSet::new(),
        1 => return v.pop().unwrap(),
        _ => {
            let h: HashSet<T> = &v.pop().unwrap() | &v.pop().unwrap();
            v.push(h);
            return multi_union(v);
        }
    }
}


pub fn get_path_from_home_dir(relative_path: &str) -> String {
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


pub fn create_collection_json(path: &str) {
    match File::create(path) {
        Ok(mut file) => {
            // Create empty list for bookmarks
            match file.write_all(b"[]") {
                Ok(_) => {},
                Err(_err) => {
                    println!("Unable to write to file: {}", path);
                    process::exit(0);
                }
            }
        },
        Err(_err) => {
            println!("Unable to create file: {}", path);
            process::exit(0);
        }
    }
}