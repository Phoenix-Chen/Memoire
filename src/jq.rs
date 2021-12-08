extern crate serde_json;
use serde::{Deserialize};

use crate::bookmark::Bookmark;
use std::process::{Command, exit};


/// SearchResult contains id/index of the bookmark and a copy of bookmark
#[derive(Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct SearchResult {
    index: usize,
    bookmark: Bookmark,
}


impl SearchResult {
    pub fn get_bookmark(&self) -> &Bookmark {
        &self.bookmark
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}


fn json_to_search_results(json: &str) -> Vec<SearchResult> {
    match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_err) => {
            println!("Failed to parse json: {:?}", &json);
            exit(0);
        }
    }
}

pub fn search(dir_path: &str, keywords: &Vec<String>) -> Vec<SearchResult> {
    let bash = Command::new("bash")
        .arg("-c")
        .arg(build_bash_command(dir_path, keywords))
        .output()
        .expect("failed bash command");
    json_to_search_results(&String::from_utf8_lossy(&bash.stdout))
}


fn build_select(keywords: &Vec<String>) -> String {
    let conditions: Vec<String> = keywords.iter().map(|keyword| {
        let contains: String = format!("contains(\"{}\")", keyword);
        [
            format!("(.bookmark.command | {})", contains),
            format!("(.bookmark.annotation | {})", contains),
            format!("(.bookmark.collection | {})", contains),
            format!("any(.bookmark.tags[] ; {})", contains),
        ]
    }).flat_map(|array: [String; 4]| 
        array.iter().map(
            |i| i.to_string()
        ).collect::<Vec<String>>()
    ).collect();
    format!("select({})", conditions.join(" or "))
}


fn build_bash_command(dir_path: &str, keywords: &Vec<String>) -> String {
    format!("cat {}/*.json | jq -s '\
            [\
                map(to_entries | \
                map({{\"index\": .key, \"bookmark\": .value}})) | \
                flatten | \
                .[] | \
                {}\
            ]'", &dir_path, &build_select(keywords))
}


pub fn validate_jsons(dir_path: &str) {
    // TODO: ensure no corrupt file in dir_path with jq
}