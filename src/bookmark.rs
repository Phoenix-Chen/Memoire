use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct Bookmark {
    command: String,
    annotation: String,
    tags: Vec<String>,
    collection: String,
}

impl Bookmark {
    /// Returns a bookmark with the command, annotation and tags given
    ///
    /// # Arguments
    ///
    /// * `command` - A string slice that holds the command
    /// * `annotation` - A string slice that holds the annotation
    /// * `tags` - A list of String that represents the tags
    /// * `collection` - A string slice that holds the collection
    pub fn new(command: &str, annotation: &str, tags: &Vec<String>, collection: &str) -> Bookmark {
        Bookmark {
            command: command.to_string(),
            annotation: annotation.to_string(),
            tags: tags.clone(),
            collection: collection.to_string(),
        }
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn set_command(&mut self, command: &str) {
        self.command = command.to_string();
    }

    pub fn get_annotation(&self) -> &String {
        &self.annotation
    }

    pub fn set_annotation(&mut self, annotation: &str) {
        self.annotation = annotation.to_string();
    }

    pub fn get_tags_as_string(&self, delimiter: &str) -> String {
        (&self.tags).join(delimiter)
    }

    pub fn set_tags(&mut self, tags: &Vec<String>) {
        self.tags = tags.to_vec();
    }

    /// Returns a boolean indicating whether command contains given string slice
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the given keyword
    pub fn command_contains(&self, s: &str) -> bool {
        self.command.contains(s)
    }

    /// Returns a boolean indicating whether annotation contains given string slice
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the given keyword
    pub fn annotation_contains(&self, s: &str) -> bool {
        self.annotation.contains(s)
    }

    /// Returns a boolean indicating whether any tag contains given string slice
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the given keyword
    pub fn tags_contains(&self, s: &str) -> bool {
        for tag in self.tags.iter() {
            if tag.contains(s) {
                return true;
            }
        }
        false
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![
            self.command.to_string(),
            self.annotation.to_string(),
            self.get_tags_as_string(" "),
        ]
    }

    pub fn to_tuple_vec(&self) -> Vec<(String, String)> {
        vec![
            ("command".to_string(), self.command.to_string()),
            ("annotation".to_string(), self.annotation.to_string()),
            ("tags".to_string(), self.get_tags_as_string(", ")),
        ]
    }
}
