use serde::{Deserialize, Serialize};

use super::util::DEFAULT_JSON_NAME;


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
            tags: tags.to_owned(),
            collection: collection.to_string(),
        }
    }

    pub fn default(command: &str, annotation: &str, tags: &Vec<String>) -> Bookmark {
        Bookmark::new(command, annotation, tags, DEFAULT_JSON_NAME)
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_annotation(&self) -> &String {
        &self.annotation
    }

    pub fn get_tags_as_string(&self, delimiter: &str) -> String {
        (self.tags).join(delimiter)
    }

    pub fn get_collection(&self) -> &String {
        &self.collection
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![
            self.command.to_string(),
            self.annotation.to_string(),
            self.get_tags_as_string(", "),
            self.collection.to_string(),
        ]
    }
}
