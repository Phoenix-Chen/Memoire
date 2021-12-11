pub mod bookmark;
pub mod util;
pub mod jq;

use bookmark::Bookmark;


pub enum Mode {
    Add(Bookmark),
    Delete(usize, String),
    Edit(usize, Bookmark),
    Search(SearchMode)
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