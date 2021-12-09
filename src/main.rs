mod util;
mod term;
mod arg_parser;
mod jq;
mod collection;

use term::Term;
use util::{DEFAULT_JSON_NAME, get_collection_dir_path, get_json_path};

use std::path::Path;
use std::env;


fn main() {
    // const COLLECTION_DIR_NAME: &str = ".memoire";
    // let collection_dir_path: String = util::get_path_from_home_dir(COLLECTION_DIR_NAME);
    // const DEFAULT_FILE_NAME: &str = "default.json";
    // let default_json_path: String = format!("{}/{}", &collection_dir_path, DEFAULT_FILE_NAME);

    // Create Collection and default.json if does not exist
    if !Path::new(&get_json_path(DEFAULT_JSON_NAME)).exists() {
        util::create_collection_dir(&get_collection_dir_path());
        util::write_to_json(
            &get_json_path(DEFAULT_JSON_NAME),
            None
        );
    }

    // Validate all json format
    jq::validate_jsons(&get_collection_dir_path());

    let mut term = Term::new();
    let args: Vec<String> = env::args().collect();

    term.process_input(args);
    term.display();
    
    
    // let search_results: Vec<SearchResult> = jq::search(&collection_dir_path, vec!["grep"]);
    // for search_result in search_results {
    //     println!("{}", search_result.get_index());
    // }
}
