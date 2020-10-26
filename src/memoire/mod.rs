extern crate serde_json;

mod bookmark;

use std::{fs, process};
use std::collections::HashSet;
pub use bookmark::Bookmark;


/// Memoire contains a list of Bookmark and file path to the json records
#[derive(Hash)]
pub struct Memoire {
    bookmarks: Vec<Bookmark>,
    file_path: String
}


/// SearchResult contains id/index of the bookmark and a copy of bookmark
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct SearchResult {
    id: usize,
    bookmark: Bookmark
}


impl SearchResult {
    pub fn new(id: usize, bookmark: &Bookmark) -> SearchResult {
        SearchResult {
            id,
            bookmark: bookmark.clone()
        }
    }

    pub fn get_bookmark(&self) -> &Bookmark {
        &self.bookmark
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
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


fn write_bookmarks(path: &str, bookmarks: &Vec<Bookmark>) {
    // TODO: Error handling
    let json_str = serde_json::to_string(&bookmarks).expect("Unable to parse bookmarks");
    fs::write(path, json_str).expect("Unable to write file");
}


impl Memoire {
    pub fn load_from(file_path: &str) -> Memoire {
        Memoire {
            bookmarks: read_bookmarks(&file_path),
            file_path: file_path.to_string()
        }
    }

    pub fn add_bookmark(&mut self, command: &str, annotation: &str, tags: &Vec<String>) {
        self.bookmarks.push(
            Bookmark::new(&command, &annotation, &tags)
        );
        write_bookmarks(&self.file_path, &self.bookmarks);
    }

    pub fn remove_bookmark(&mut self, id: usize) {
        if id < self.bookmarks.len() {
            self.bookmarks.remove(id);
            write_bookmarks(&self.file_path, &self.bookmarks);
        } else {
            println!("Cannot find such bookmark.");
        }
    }

    pub fn edit_bookmark(&mut self, id: usize, command: Option<&str>, annotation:  Option<&str>, tags: Option<&Vec<String>>) {
        if id < self.bookmarks.len() {
            let bookmark: &mut Bookmark = &mut self.bookmarks[id];
            match command {
                Some(c) => {
                    if c != bookmark.get_command().to_string() {
                        bookmark.set_command(&c);
                    }
                },
                None => {}
            }

            match annotation {
                Some(a) => {
                    if a != bookmark.get_annotation().to_string() {
                        bookmark.set_annotation(&a);
                    }
                },
                None => {}
            }
            match tags {
                Some(t) => {
                    bookmark.set_tags(&t);
                },
                None => {}
            }
            write_bookmarks(&self.file_path, &self.bookmarks);
        } else {
            println!("Cannot find such bookmark.");
        }
    }

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
