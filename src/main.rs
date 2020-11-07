mod memoire;
mod util;
mod term;
mod arg_parser;
mod tldr_parser;

use term::Term;

use std::path::Path;
use std::env;

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

    let mut term = Term::new(&history_file_path);
    let args: Vec<String> = env::args().collect();

    term.process_input(args);
    term.display();
}
