extern crate termion;
extern crate dirs;

use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

pub mod event;

pub enum Mode {
    Add(AddMode),
    Delete(usize),
    Edit(EditMode),
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


pub fn create_history(path: &str) {
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


pub fn get_path_from_home_dir(history_file_name: &str) -> String {
    match dirs::home_dir() {
        Some(home_dir) => {
            let mut history_file_path = home_dir.into_os_string();
            history_file_path.push("/");
            history_file_path.push(history_file_name);
            match history_file_path.to_str() {
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
