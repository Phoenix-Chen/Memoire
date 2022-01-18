use std::{
    fs::read_dir,
    process::{Command, Stdio, exit}
};

use serde::{Deserialize};

use super::bookmark::Bookmark;
use super::util::write_to_json;


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


// TODO: implement insert at index, currently index is unused
pub fn add(json_path: &str, bookmark: &Bookmark, _index: Option<usize>) {
    write_to_json(json_path, Some(&execute_bash(
        &format!(
            "cat {} | jq -s $'.[0] |= .+ [{}] | .[0]'",
            json_path,
            serde_json::to_string(&bookmark).expect("Unable to parse bookmarks").replace("'", "\\'")
        )
    )));
}


pub fn delete(json_path: &str, index: usize) {
    write_to_json(json_path, Some(&execute_bash(
        &format!(
            "cat {} | jq -s 'del(.[0][{}]) | .[0]'",
            json_path,
            index
        )
    )));
}


pub fn search(dir_path: &str, keywords: &[&str]) -> Vec<SearchResult> {
    json_to_search_results(
        &execute_bash(
            &build_lookup_command(dir_path, keywords)
        )
    )
}


/// Ensure no corrupt json file in dir_path with jq
pub fn validate_jsons(dir_path: &str) {
    for path in read_dir(dir_path).unwrap() {
        let path = path.unwrap().path().into_os_string().to_str().unwrap().to_string();
        if path.ends_with(".json") {
            let error_msg = format!(
                "File {} contains invalid json format.",
                &path
            );
            let status = Command::new("bash")
                         .arg("-c")
                         .arg(
                             format!(
                                 "cat {} | jq empty",
                                 &path
                             )
                         )
                         .stdout(Stdio::null())
                         .stderr(Stdio::null())
                         .status()
                         .expect(
                            &error_msg
                         );
            if !status.success() {
                println!("{}", &error_msg);
                exit(0);
            }
        }
    }
}


fn execute_bash(command: &str) -> String {
    let bash = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed bash command");
    String::from_utf8_lossy(&bash.stdout).into_owned()
}


fn json_to_search_results(json: &str) -> Vec<SearchResult> {
    match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_err) => {
            println!("Failed to parse json: {:?}", &json);
            exit(0);
        }
    }
}


fn build_select(keywords: &[&str]) -> String {
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


fn build_lookup_command(dir_path: &str, keywords: &[&str]) -> String {
    format!("cat {}/*.json | jq -s '\
            [\
                map(to_entries | \
                map({{\"index\": .key, \"bookmark\": .value}})) | \
                flatten | \
                .[] | \
                {}\
            ]'", &dir_path, &build_select(keywords))
}
