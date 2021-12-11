mod term;
mod arg_parser;
mod collection;

use collection::{
    jq::validate_jsons,
    util::{DEFAULT_JSON_NAME, create_collection_dir, get_json_path, get_collection_dir_path}
};
use term::Term;
// use util::{DEFAULT_JSON_NAME, get_collection_dir_path, get_json_path};

use std::path::Path;
use std::env;


fn main() {
    // Create Collection and default.json if does not exist
    // TODO: Move this logic to collection
    if !Path::new(&get_json_path(DEFAULT_JSON_NAME)).exists() {
        create_collection_dir(&get_collection_dir_path());
        // util::write_to_json(
        //     &get_json_path(DEFAULT_JSON_NAME),
        //     None
        // );
    }

    // Validate all json format
    validate_jsons(&get_collection_dir_path());

    let mut term = Term::new();
    let args: Vec<String> = env::args().collect();

    // term.process_input(args);
    term.display();
    
    
    // let search_results: Vec<SearchResult> = jq::search(&collection_dir_path, vec!["grep"]);
    // for search_result in search_results {
    //     println!("{}", search_result.get_index());
    // }
}
