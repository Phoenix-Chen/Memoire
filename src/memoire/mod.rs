extern crate serde_json;


mod collection;

use std::{fs, process};
use std::collections::HashSet;
pub use collection::Collection;


/// Memoire contains a list of Bookmark and file path to the json records
#[derive(Hash)]
pub struct Memoire {
    // bookmarks: Vec<Bookmark>,
    dir_path: String
}


fn read_bookmarks(path: &str) -> Vec<Bookmark> {
    match fs::read_to_string(&path) {
        Ok(data) => {
            // Err if Bookmark struct contains attribute json string does not have
            // Find a way to fix that for future compatibility reason
            match serde_json::from_str(&data) {
                Ok(v) => v,
                Err(_err) => {
                    println!("Unable to parse file: {:?}", &path);
                    process::exit(0);
                }
            }
        },
        Err(_err) => {
            println!("Unable to read file: {:?}", &path);
            process::exit(0);
        }
    }
}


fn read_file_as_string(path: &str) -> String {
    match fs::read_to_string(&path) {
        Ok(data) => data,
        Err(_err) => {
            println!("Unable to read file: {:?}", &path);
            process::exit(0);
        }
    }
}


fn write_bookmarks(path: &str, bookmarks: &Vec<Bookmark>) {
    // TODO: Error handling
    let json_str = serde_json::to_string(&bookmarks).expect("Unable to parse bookmarks");
    fs::write(path, json_str).expect("Unable to write file");
}


impl Memoire {
    pub fn load_from(dir_path: &str) -> Memoire {
        Memoire {
            // bookmarks: read_bookmarks(&file_path),
            dir_path: dir_path.to_string()
        }
    }

    pub fn get_collection(&self, collection: &str) -> Vec<Bookmark> {
        let file_name = format!("{}/{}.json", self.dir_path, &collection);
        match serde_json::from_str(&read_file_as_string(&file_name)) {
            Ok(v) => v,
            Err(_err) => {
                println!("Unable to parse json: {:?}", &file_name);
                process::exit(0);
            }
        }
    }

    // pub fn add_bookmark(&mut self, command: &str, annotation: &str, tags: &Vec<String>) {
    //     self.bookmarks.push(
    //         Bookmark::new(&command, &annotation, &tags)
    //     );
    //     write_bookmarks(&self.file_path, &self.bookmarks);
    // }

    // pub fn remove_bookmark(&mut self, id: usize) {
    //     if id < self.bookmarks.len() {
    //         self.bookmarks.remove(id);
    //         write_bookmarks(&self.file_path, &self.bookmarks);
    //     } else {
    //         println!("Cannot find such bookmark.");
    //     }
    // }

    // pub fn edit_bookmark(&mut self, id: usize, command: Option<&str>, annotation:  Option<&str>, tags: Option<&Vec<String>>) {
    //     if id < self.bookmarks.len() {
    //         let bookmark: &mut Bookmark = &mut self.bookmarks[id];
    //         match command {
    //             Some(c) => {
    //                 if c != bookmark.get_command().to_string() {
    //                     bookmark.set_command(&c);
    //                 }
    //             },
    //             None => {}
    //         }

    //         match annotation {
    //             Some(a) => {
    //                 if a != bookmark.get_annotation().to_string() {
    //                     bookmark.set_annotation(&a);
    //                 }
    //             },
    //             None => {}
    //         }
    //         match tags {
    //             Some(t) => {
    //                 bookmark.set_tags(&t);
    //             },
    //             None => {}
    //         }
    //         write_bookmarks(&self.file_path, &self.bookmarks);
    //     } else {
    //         println!("Cannot find such bookmark.");
    //     }
    // }

    /// Get all bookmarks in memoire
    pub fn all(&self) -> HashSet<SearchResult> {
        self.search(true, true, true, "")
    }

    // TODO: Update contains for case insensitive match
    pub fn search(&self, in_command: bool, in_annotation: bool, in_tags: bool, search_str: &str) -> HashSet<SearchResult> {
        let mut search_results: HashSet<SearchResult> = HashSet::new();
        for (i, bookmark) in self.bookmarks.iter().enumerate() {
            if (in_command && bookmark.command_contains(&search_str)) ||
                (in_annotation && bookmark.annotation_contains(&search_str)) ||
                (in_tags && bookmark.tags_contains(&search_str)) {
                search_results.insert(SearchResult::new(i, bookmark));
            }
        }
        search_results
    }
}
