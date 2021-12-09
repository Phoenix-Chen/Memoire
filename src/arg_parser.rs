extern crate clap;

use clap::{Arg, App, SubCommand};

use crate::{
    jq::{SearchResult, search},
    util::{AddMode, EditMode, Mode, SearchMode, get_collection_dir_path}
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
            Some(Mode::Add(add_mode)) => {
                // memoire.add_bookmark(&add_mode.command, &add_mode.annotation, &add_mode.tags);
                // memoire.search(true, true, true, "")
                Vec::new()
            },
            Some(Mode::Edit(edit_mode)) => {
                Vec::new()
                // memoire.edit_bookmark(
                //     edit_mode.get_id(),
                //     Some(edit_mode.get_command()),
                //     Some(edit_mode.get_annotation()),
                //     Some(edit_mode.get_tags())
                // );
                // memoire.search(true, false, false, "")
            },
            Some(Mode::Parse(tldr_page_path)) => {
                Vec::new()
                // update_memoire_from_tldr(memoire, tldr_page_path);
                // memoire.search(true, true, true, "")
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
            .arg(Arg::with_name("TLDR")
                .long("tldr")
                .takes_value(true)
                .help("Load from tldr-pages. (ex. pages/osx/base64)")
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
                .arg(Arg::with_name("ID")
                    .short("i")
                    .takes_value(true)
                    .long("id")
                    .required(true)
                    .help("ID of the bookmark")
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
            self.mode = Some(Mode::Add(AddMode {
                command,
                annotation,
                tags
            }));
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
            let id: String = matches.values_of("ID").unwrap().collect();
            let id: usize = id.parse().unwrap();
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
            self.mode = Some(Mode::Edit(EditMode::new(id, command, annotation, tags)));
            return;
        }

        // Look for tldr args
        if matches.is_present("TLDR") {
            let tldr_page_path = matches.value_of("TLDR").unwrap();
            self.mode = Some(Mode::Parse(tldr_page_path.to_owned()));
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


// fn update_memoire_from_tldr(memoire: &mut Memoire, page_path: &str) {
//     if let Ok(body) = block_on(download_tldr(page_path)) {
//         if let Ok(tldr_page) = parse_page(&body) {
//             let tags = vec![tldr_page.get_command_name().to_string()];
//             for v in tldr_page.get_examples().into_iter() {
//                 memoire.add_bookmark(&v.1, &v.0, &tags)
//             }
//         } else {
//             panic!("Failed to parse tldr page at: {:?}", page_path);
//         }
//     } else {
//         panic!("Failed to download tldr page at: {:?}", page_path);
//     }
// }
