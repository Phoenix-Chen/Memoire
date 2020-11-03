mod memoire;
mod util;
// mod term;
mod arg_parser;
mod tldr_parser;

// use term::Term;

use std::path::Path;
use std::env;
use futures::executor::block_on;

use memoire::Memoire;
use tldr_parser::{download_tldr, parse_page};
use util::get_path_from_home_dir;

#[tokio::main]
async fn main() {
    const HISTORY_FILE_NAME: &str = ".memoir_history.json";
    let history_file_path: String = get_path_from_home_dir(HISTORY_FILE_NAME);

    // TODO: Possibly change parameters for create_history and load_from
    // to <S: AsRef<OsStr> + ?Sized>(s: &S)
    if !Path::new(&history_file_path).exists() {
        util::create_history(&history_file_path);
    }

    // let mut term = Term::new(&history_file_path);
    let args: Vec<String> = env::args().collect();

    // term.process_input(args);
    // term.display();
    update_memoire_from_tldr("pages/osx/base64");
}

// fn update_memoire_from_tldr(memoire: &Memoire, page_path: &str) {
fn update_memoire_from_tldr(page_path: &str) {
    if let Ok(body) = block_on(download_tldr(page_path)) {
        if let Ok(tldr_page) = parse_page(&body) {
            for v in tldr_page.get_examples().into_iter() {
                println!("{:?}:{:?}", v.0, v.1);
            }
        } else {
            panic!("Failed to parse tldr page at: {:?}", page_path);
        }
    } else {
        panic!("Failed to download tldr page at: {:?}", page_path);
    }
}
