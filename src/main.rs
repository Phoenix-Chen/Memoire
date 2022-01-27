mod term;
mod collection;

use std::{env, path::Path, process::{Command, Stdio, exit}};
use clap::{Arg, App, SubCommand};

use collection::{
    bookmark::Bookmark,
    jq::{SearchResult, add, delete, search, validate_jsons},
    util::{
        DEFAULT_JSON_NAME, create_collection_dir,
        get_json_path, get_collection_dir_path
    }
};
use term::Term;


fn main() {
    // Create Collection path if does not exist
    if !Path::new(&get_json_path(DEFAULT_JSON_NAME)).exists() {
        create_collection_dir(&get_collection_dir_path());
    }

    // Check if jq is installed
    let status = Command::new("bash")
                         .arg("-c")
                         .arg("jq --version")
                         .stdout(Stdio::null())
                         .stderr(Stdio::null())
                         .status()
                         .expect("jq command not found");
    if !status.success() {
        println!("Unable to execute `jq`");
        println!("Please make sure `jq` is correctly installed.");
        println!("For more Details see: https://stedolan.github.io/jq/");
        exit(1);
    }

    // Validate all json format
    validate_jsons(&get_collection_dir_path());

    let mut term = Term::new();
    term.get_mut_widget_manager().update_result_table(parse_input(env::args().collect()));

    term.display().unwrap();
}


fn parse_input(inputs: Vec<String>) -> Vec<SearchResult> {
    let app = App::new("Memoire")
        .version("0.1")
        .author("Phoenix Chen")
        .about("A CLI for bookmarking commands")
        .arg(Arg::with_name("Search")
            .short("s")
            .long("search")
            .takes_value(true)
            .multiple(true)
            .help("General search")
        )
        .arg(Arg::with_name("Tags")
            .short("t")
            .long("tags")
            .takes_value(true)
            .multiple(true)
            .help("Search with tags")
        )
        .arg(Arg::with_name("Annotation")
            .short("a")
            .long("annotation")
            .takes_value(true)
            .multiple(true)
            .help("Search with annotation")
        )
        .arg(Arg::with_name("Command")
            .short("c")
            .long("command")
            .takes_value(true)
            .multiple(true)
            .help("Search with command")
        )
        .arg(Arg::with_name("Collection")
            // .short("c")
            .long("collection")
            .takes_value(true)
            .multiple(true)
            .help("Search by collection")
        )
        .arg(Arg::with_name("Intersection")
            .short("i")
            .long("intersection")
            .takes_value(false)
            .help("Return search results with intersection (default union)")
        )
        .subcommand(SubCommand::with_name("--add")
            .about("Add bookmark")
            .arg(Arg::with_name("Tags")
                .short("t")
                .long("tags")
                .takes_value(true)
                .multiple(true)
                .help("Search with tags")
            )
            .arg(Arg::with_name("Annotation")
                .short("a")
                .long("annotation")
                .takes_value(true)
                .multiple(true)
                .help("Search with annotation")
            )
            .arg(Arg::with_name("Command")
                .short("c")
                .long("command")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .help("Search with command")
            )
            .arg(Arg::with_name("Collection")
                // .short("c")
                .long("collection")
                .takes_value(true)
                .multiple(true)
                .help("Search with command")
            )
        )
        .subcommand(SubCommand::with_name("--delete")
            .about("Delete bookmark")
            .arg(Arg::with_name("Index")
                .short("i")
                .takes_value(true)
                .long("index")
                .required(true)
                .help("Index of the bookmark")
            )
            .arg(Arg::with_name("Collection")
                .short("c")
                .takes_value(true)
                .long("collection")
                .required(true)
                .help("Collection of the bookmark")
            )
        )
        .subcommand(SubCommand::with_name("--edit")
            .about("Edit exisiting bookmark")
            .arg(Arg::with_name("Index")
                .short("i")
                .takes_value(true)
                .long("index")
                .required(true)
                .help("Index of the bookmark")
            )
            .arg(Arg::with_name("Tags")
                .short("t")
                .long("tags")
                .takes_value(true)
                .multiple(true)
                .help("New tags")
            )
            .arg(Arg::with_name("Annotation")
                .short("a")
                .long("annotation")
                .takes_value(true)
                .multiple(true)
                .help("New annotation")
            )
            .arg(Arg::with_name("Command")
                .short("c")
                .long("command")
                .takes_value(true)
                .multiple(true)
                .help("New command")
            )
            .arg(Arg::with_name("Collection")
                // .short("c")
                .long("collection")
                .takes_value(true)
                .multiple(true)
                .help("New collection")
            )
        );
    let matches = app.get_matches_from(inputs);

    // Check all non-search conditions first?
    // Check if input contains add sub command
    if let Some(matches) = matches.subcommand_matches("--add") {
        let mut command: String = "".to_string();
        if matches.is_present("Command") {
            let vals: Vec<&str> = matches.values_of("Command").unwrap().collect();
            command = vals.join(" ");
        }
        let mut annotation: String = "".to_string();
        if matches.is_present("Annotation") {
            let vals: Vec<&str> = matches.values_of("Annotation").unwrap().collect();
            annotation = vals.join(" ");
        }
        let mut tags: Vec<String> = Vec::new();
        if matches.is_present("Tags") {
            tags = matches.values_of("Tags").unwrap().map(|s| s.to_string()).collect();
        }
        let mut collection: String = "".to_string();
        if matches.is_present("Collection") {
            let vals: Vec<&str> = matches.values_of("Collection").unwrap().collect();
            collection = vals.join(" ");
        }

        add(
            &get_json_path(&collection),
            &Bookmark::new(
                &command,
                &annotation,
                &tags,
                &collection
            ),
            None
        );
        // TODO: Fix this to only search keyword by collection
        return search(
            &get_collection_dir_path(),
            &[&collection]
        );
    }

    // Look for delete subcommand
    if let Some(matches) = matches.subcommand_matches("--delete") {
        let index: String = matches.values_of("Index").unwrap().collect();
        let index: usize = index.parse().unwrap();
        let collection: String = matches.values_of("Collection").unwrap().collect();
        delete(&get_json_path(&collection), index);
        // TODO: Fix this to only search keyword by collection
        return search(
            &get_collection_dir_path(),
            &[&collection]
        );
    }

    // Check if input contains edit command
    // Currently doesn't support change collection
    if let Some(matches) = matches.subcommand_matches("--edit") {
        let index: String = matches.values_of("Index").unwrap().collect();
        let index: usize = index.parse().unwrap();
        let mut command: String = "".to_string();
        if matches.is_present("Command") {
            let vals: Vec<&str> = matches.values_of("Command").unwrap().collect();
            command = vals.join(" ");
        }
        let mut annotation: String = "".to_string();
        if matches.is_present("Annotation") {
            let vals: Vec<&str> = matches.values_of("Annotation").unwrap().collect();
            annotation = vals.join(" ");
        }
        let mut tags: Vec<String> = Vec::new();
        if matches.is_present("Tags") {
            tags = matches.values_of("Tags").unwrap().map(|s| s.to_string()).collect();
        }
        let mut collection: String = "".to_string();
        if matches.is_present("Collection") {
            let vals: Vec<&str> = matches.values_of("Collection").unwrap().collect();
            collection = vals.join(" ");
        }
        delete(&get_json_path(&collection), index);
        add(
            &get_json_path(&collection),
            &Bookmark::new(
                &command,
                &annotation,
                &tags,
                &collection
            ),
            Some(index)
        );
        // TODO: Fix this to only search keyword by collection
        return search(
            &get_collection_dir_path(),
            &[&collection]
        );
    }

    // Look for search args
    let args: Vec<&str> = vec!["Search", "Annotation", "Command", "Tags", "Collection"];
    let mut keywords: Vec<&str> = Vec::new();
    for arg in args.iter() {
        if matches.is_present(arg) {
            keywords.append(
                &mut matches.values_of(arg).unwrap().collect()
            );
        }
    }
    if matches.is_present("Intersection") {
        // search_mode.set_union(false);
    }
    // Return all results if no args passed
    if keywords.is_empty() {
        keywords.push("");
    }
    search(
        &get_collection_dir_path(),
        &keywords
    )
}