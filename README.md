# Disclaimer
***
This branch contains modified code from [tui](https://github.com/fdehau/tui-rs) library pulled down from my personal [fork](https://github.com/Phoenix-Chen/tui-rs/tree/optional_trim_end). This branch is created for the purpose of publishing, due to [crates.io does not allow packages to be published with git dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories).

# Memoire
***
Editable, portable and sharable CLI cheatsheet.

<img src="./assets/memoire_demo.gif" alt="memoire demo">

You can search archived commands by keywords in command, annotation, tags and/or collection. The archived commands are editable and removable. Archived commands are saved in `json` format for editability and portability.

## Requirement
***
This program use [jq](https://stedolan.github.io/jq/).

## Installation
***

### With Cargo
***
Make sure you have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

Install with:
```sh
cargo install memoire
```

## Usage
***

### CLI
***
- Show all:
    ```sh
    memoire
    ```
- General search:
    ```sh
    memoire [-s|--search] <keyword_1> <keyword_2> ...
    ```

### TUI
***
- `Ctrl-c`: Quit program at any time.
- `Ctrl-a`: To add new bookmark.
- `Up` / `Down` / `Left` / `Right`: Move.
- `Enter`: To select / submit.
- `Esc`: To deselect / go back.

## Memoire Collections
***
A personal collections of commands cheatsheets in `json` format compatible with `Memoire` can be found [here](https://github.com/Phoenix-Chen/memoire_collections)

## Upgrade from below version 0.1.2
***
If you installed a `memoire` version below `0.1.2`. Run following script to move previous bookmarks into `default` collection:
```sh
cat ~/.memoir_history.json | jq -s '.[0][] |= . + {"collection": "default"} | .[0]' > ~/.memoire/default.json
```

## License
***
Memoire is under [Apache 2.0 License](LICENSE).
