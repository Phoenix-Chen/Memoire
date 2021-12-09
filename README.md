# Memoire
***
Memoire is a TUI program to help you archive commands (or any text).

<img src="./assets/memoire_demo.gif" alt="memoire demo">

You can search archived commands by keywords in command, annotation and/or tags. The archived commands are editable and deletable. Archived commands are saved in json format for easy sharing.

## Installation
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
- Show all records:
    ```sh
    memoire
    ```
- General search:
    ```sh
    memoire [-s|--search] <keyword_1> <keyword_2> ...
    ```
- Search by command:
    ```sh
    memoire [-c|--command] <keyword_1> <keyword_2> ...
    ```
- Search by annotation:
    ```sh
    memoire [-a|--annotation] <keyword_1> <keyword_2> ...
    ```
- Search by tags:
    ```sh
    memoire [-t|--tags] <keyword_1> <keyword_2> ...
    ```
- You can combine arguments:
    ```sh
    memoire -c <keyword_1> -t <keyword_2> ...
    ```
- By default, search results calculate the union of search results of each keyword. For intersection add `-i`:
    ```sh
    memoire [-i|--intersection] -c <keyword_1> -t <keyword_2> <keyword_3> ...
    ```


### TUI
***
- `Ctrl-c`: Quit program at any time.
- `Up` / `Down`: To move between options or panels.
- `Enter`: To select option.
- `Ctrl-a`: To add new record.
- `Left` / `Right`: To move cursor in input_mode.


## Requirement
***
This program use [rust-clipboard](https://github.com/aweinstock314/rust-clipboard) which requires x11 on Linux.

## It's Not a Bug, It's a feature (aka Will fix)
***
1. In input mode the cursor moves out of border if text exceeds 1 line.

## What's next
***
- Copy & paste in input mode.
- Unit tests.
- A system to share, update and manage json files. Or parse [existing command collection](https://github.com/tldr-pages/tldr) to json.
- Alternative TUI libraries:
    - [cursive](https://github.com/gyscos/cursive)
    - [titik](https://github.com/ivanceras/titik)
- Look into option to switch out InputDialog with existing dialog libraries:
    - [dialoguer](https://docs.rs/dialoguer/0.7.1/dialoguer/)
    - [dialog](https://docs.rs/dialog/0.3.0/dialog/)

## Q & A
***
1. How to retrieve record id from CLI?

    ¯\\_(ツ)_/¯
2. Why are you handling errors like an amateur?

    But I am an amateur...

## License
***
Memoire is under [Apache 2.0 License](LICENSE).
