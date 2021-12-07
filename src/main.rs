// mod memoire;
mod util;
// mod term;
// mod arg_parser;
mod jq;
mod bookmark;
// mod tldr_parser;

// use term::Term;
// extern crate memoire;
// use memoire::util;
use util::get_path_from_home_dir;
use jq::SearchResult;

use std::path::Path;
use std::env;



#[tokio::main]
async fn main() {
    const COLLECTION_DIR_NAME: &str = ".memoire";
    const DEFAULT_FILE_NAME: &str = "default.json";
    let collection_dir_path: String = get_path_from_home_dir(COLLECTION_DIR_NAME);

    // // TODO: Possibly change parameters for create_history and load_from
    // // to <S: AsRef<OsStr> + ?Sized>(s: &S)
    // if !Path::new(&collection_dir_path).exists() {
    //     util::create_collection_dir(&collection_dir_path);
    //     util::create_collection_json(
    //         &format!("{}/{}", &collection_dir_path, DEFAULT_FILE_NAME)
    //     );
    // }

    // let mut term = Term::new(&collection_dir_path);
    // let args: Vec<String> = env::args().collect();

    // term.process_input(args);
    // term.display();
    let search_results: Vec<SearchResult> = jq::search(&collection_dir_path, "grep");
    for search_result in search_results {
        println!("{}", search_result.get_index());
    }
}
