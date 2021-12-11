extern crate clap;

use clap::{Arg, App, SubCommand};

use crate::{
    collection::{
        Mode,
        SearchMode,
        bookmark::Bookmark,
        jq::{SearchResult, add, delete, search},
        util::{get_collection_dir_path, get_json_path}
    }
    
    
};


pub struct ArgParser {
    input: Option<Vec<String>>,
    mode: Option<Mode>
}

impl ArgParser {
    pub fn new() -> ArgParser {
        ArgParser {
            input: None,
            mode: None
        }
    }

    pub fn reset_state(&mut self) {
        self.input = None;
        self.mode = None;
    }

    pub fn get_results(&self) -> Vec<SearchResult> {
        match &self.mode {
            Some(Mode::Delete(index, collection)) => {
                // memoire.remove_bookmark(*id);
                // memoire.search(true, true, true, "")
                delete(
                    &get_json_path(&collection),
                    *index
                );
                Vec::new()
            },
            Some(Mode::Search(search_mode)) => {
                
                // let mut search_results: Vec<HashSet<SearchResult>> = Vec::new();
                // for keyword in  {
                    
                // }
                search(&get_collection_dir_path(), search_mode.get_searches())
                // for keyword in search_mode.get_tags() {
                //     search_results.push(memoire.search(false, false, true, keyword));
                // }
                // for keyword in search_mode.get_commands() {
                //     search_results.push(memoire.search(true, false, false, keyword));
                // }
                // for keyword in search_mode.get_annotations() {
                //     search_results.push(memoire.search(false, true, false, keyword));
                // }

                // if search_mode.get_union() {
                //     return multi_union(search_results);
                // }
                // multi_intersection(search_results)
            },
            Some(Mode::Add(bookmark)) => {
                add(
                    &get_json_path(&bookmark.get_collection()),
                    &bookmark,
                    None
                );
                Vec::new()
            },
            Some(Mode::Edit(index, bookmark)) => {
                Vec::new()
                // memoire.edit_bookmark(
                //     edit_mode.get_id(),
                //     Some(edit_mode.get_command()),
                //     Some(edit_mode.get_annotation()),
                //     Some(edit_mode.get_tags())
                // );
                // memoire.search(true, false, false, "")
            },
            None => {
                Vec::new()
                // memoire.search(true, true, true, "")
            }
        }
    }

    pub fn matches_input(&mut self, inputs: Vec<String>) {
        self.reset_state();
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

            self.mode = Some(Mode::Add(Bookmark::new(
                &command,
                &annotation,
                &tags,
                Some(&collection)
            )));
            return;
        }

        // Look for delete subcommand
        if let Some(matches) = matches.subcommand_matches("--delete") {
            let index: String = matches.values_of("Index").unwrap().collect();
            let id: usize = index.parse().unwrap();
            let collection: String = matches.values_of("Collection").unwrap().collect();
            self.mode = Some(Mode::Delete(id, collection));
            return;
        }

        // Check if input contains edit command
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
            self.mode = Some(Mode::Edit(index, Bookmark::new(
                &command,
                &annotation,
                &tags,
                Some(&collection)
            )));
            return;
        }

        // Look for search args
        let args: Vec<&str> = vec!["Search", "Annotation", "Command", "Tags"];
        let mut search_mode = SearchMode::default();
        for arg in args.iter() {
            if matches.is_present(arg) {
                let vals: Vec<String> = matches.values_of(arg).unwrap().map(|s| s.to_string()).collect();
                match arg {
                    &"Tags" => {
                        search_mode.set_tags(vals);
                    },
                    &"Search" => {
                        search_mode.set_searches(vals);
                    },
                    &"Annotation" => {
                        search_mode.set_annotations(vals);
                    },
                    &"Command" => {
                        search_mode.set_commands(vals);
                    },
                    _ => {}
                }
            }
        }
        if matches.is_present("Intersection") {
            search_mode.set_union(false);
        }
        // Return all results if no args passed
        if !search_mode.has_input() {
            search_mode.set_searches(vec!["".to_owned()]);
        }
        self.mode = Some(Mode::Search(search_mode));
    }
}
