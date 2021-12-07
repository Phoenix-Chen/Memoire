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

pub fn search(dir_path: &str, keyword: &str) -> Vec<SearchResult> {
    let bash = Command::new("bash")
        .arg("-c")
        .arg(build_bash_command(dir_path, keyword))
        .output()
        .expect("failed bash command");
    json_to_search_results(&String::from_utf8_lossy(&bash.stdout))
}


fn build_select(keyword: &str) -> String{
    let contains: String = format!("contains(\"{}\")", keyword);
    format!("select(\
            (.bookmark.command | {}) or \
            (.bookmark.annotation | {}) or \
            (.bookmark.collection | {}) or \
            (.bookmark.tags | select(.[] | {}))\
        )", &contains, &contains, &contains, &contains)
}


fn build_bash_command(dir_path: &str, keyword: &str) -> String {
    format!("cat {}/*.json | jq -s '\
            [\
                map(to_entries | \
                map({{\"index\": .key, \"bookmark\": .value}})) | \
                flatten | \
                .[] | \
                {}\
            ]'", &dir_path, &build_select(keyword))
}