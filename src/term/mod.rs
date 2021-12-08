extern crate clipboard;

use crate::util;
// use crate::memoire;
use crate::arg_parser;

mod widget;

use std::{
    io::{stdout, Stdout},
    sync::mpsc
};

use clipboard::{ClipboardContext, ClipboardProvider};
use termion::{
    event::Key,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen
};

use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Paragraph, Wrap},
    Terminal,
};

use arg_parser::ArgParser;
// use memoire::Memoire;
use util::event::{Event, Events};
use widget::{Action, WidgetManager, ACTIONS};


pub struct Term {
    screen: Terminal<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>,
    events: Events,
    // memoire: Memoire,
    wm: WidgetManager,
    arg_parser: ArgParser
}


impl Term {
    pub fn new(memoire_history: &str) -> Term {
        Term {
            screen: Terminal::new(TermionBackend::new(AlternateScreen::from(
                stdout().into_raw_mode().unwrap(),
            )))
            .unwrap(),
            events: Events::new(),
            // memoire: Memoire::load_from(memoire_history),
            wm: WidgetManager::new(),
            arg_parser: ArgParser::new()
        }
    }

    /// Pass input into arg_parser then get the results from arg_parser and update result_table
    pub fn process_input(&mut self, input: Vec<String>) {
        self.arg_parser.matches_input(input);
        self.wm
            .update_result_table(
                self.arg_parser.get_results());
    }

    pub fn display(&mut self) -> Result<(), mpsc::RecvError> {
        self.screen.hide_cursor().unwrap();
        loop {
            self.draw();

            if let Event::Input(input) = self.events.next()? {
                match input {
                    Key::Ctrl('c') => break,  // Need to match exit_key in util::event for consistent behavior
                    Key::Ctrl('a') => {
                        if self.wm.get_cur_focus() != "input_dialog" {
                            self.wm.reset_result_table_state();
                            self.wm.set_input_dialog(vec![
                                ("command".to_string(), "".to_string()),
                                ("annotation".to_string(), "".to_string()),
                                ("tags".to_string(), "".to_string()),
                            ]);
                            self.wm.set_cur_focus("input_dialog");
                        }
                    }
                    Key::Char('\n') => {
                        match self.wm.get_cur_focus() {
                            "action_list" => {
                                match self.wm.get_action_list_state_selected() {
                                    Some(action_index) => {
                                        match ACTIONS[action_index] {
                                            Action::Copy => {
                                                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                                ctx.set_contents(
                                                    self.wm
                                                        .get_selected_item_command()
                                                        .to_owned(),
                                                )
                                                .unwrap();
                                                break;
                                            }
                                            Action::Edit => {
                                                self.update_input_dialog();
                                                self.wm.set_cur_focus("input_dialog");
                                            }
                                            Action::Delete => {
                                                match self.wm.get_selected_item_id() {
                                                    Some(id) => {
                                                        // self.memoire.remove_bookmark(id);
                                                        // self.wm.update_result_table(self.memoire.all());
                                                    },
                                                    None => {}  // Add error log
                                                }
                                                self.wm.reset_action_list_state();
                                                self.wm.reset_result_table_state();
                                                self.wm.set_cur_focus("result_table");
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            }
                            "result_table" => match self.wm.get_result_table_state_selected() {
                                Some(_) => {
                                    self.wm.set_cur_focus("action_list");
                                    self.wm.key_down();
                                }
                                None => {}
                            },
                            "input_dialog" => {
                                let inputs = self.wm.get_input_dialog_inputs();
                                let mut common_args: Vec<String> = Vec::new();
                                for input in inputs.into_iter() {
                                    // TODO: research extend_from_slice
                                    common_args.append(
                                        &mut vec![
                                            format!("{}{}", "--", input.0),
                                            input.1.to_owned()
                                        ]
                                    );
                                }
                                // Split tags
                                let mut tags: Vec<String> = match common_args.pop() {
                                    Some(tags_str) => {
                                        tags_str.split(',').map(|tag| {
                                            tag.trim().to_owned()
                                        }).collect()
                                    },
                                    None => {
                                        // Log error here
                                        panic!("No inputs from input_dialog")
                                    }
                                };
                                common_args.append(&mut tags);
                                let new_args = match self.wm.get_selected_item_id() {
                                    Some(id) => {  // Edit
                                        let mut edit_args = vec![
                                            "memoire".to_owned(),
                                            "--edit".to_owned(),
                                            "--id".to_owned(),
                                            id.to_string()
                                        ];
                                        edit_args.append(&mut common_args);
                                        edit_args
                                    },
                                    None => {  // Add
                                        let mut add_args = vec![
                                            "memoire".to_owned(),
                                            "--add".to_owned()
                                        ];
                                        add_args.append(&mut common_args);
                                        add_args
                                    }
                                };
                                self.process_input(new_args);
                                self.wm.reset_action_list_state();
                                self.wm.set_cur_focus("result_table");
                            }
                            _ => {}
                        }
                    }
                    Key::Char('\t') => {
                        // Overwrite tab behavior in input mode
                        if self.wm.get_cur_focus() == "input_dialog" {
                            self.wm.update_input_dialog_input(' ');
                        }
                    }
                    Key::Char(character) => {
                        if self.wm.get_cur_focus() == "input_dialog" {
                            self.wm.update_input_dialog_input(character);
                        }
                    }
                    Key::Up => {
                        self.wm.key_up();
                    }
                    Key::Down => {
                        self.wm.key_down();
                    }
                    Key::Left => {
                        self.wm.key_left();
                    }
                    Key::Right => {
                        self.wm.key_right();
                    }
                    Key::Backspace => {
                        self.wm.key_backspace();
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self) {
        let cur_focus = self.wm.get_cur_focus();
        // For render input dialog
        let input_size = self.wm.get_input_dialog_input_size();
        let paragraphs = self.wm.get_input_dialog_widgets();
        let input_dialog_cur_input_ind = self.wm.get_input_dialog_cur_input_ind();
        let input_dialog_cursor = self.wm.get_input_dialog_cursor() as u16;
        // For render
        let result_table_widget = self.wm.get_result_table_widget();
        let result_table_state = self.wm.get_result_table_state();
        let display_panel_widget = self.wm.get_display_panel_widget();
        let action_list_widget = self.wm.get_action_list_widget();
        let action_list_state = self.wm.get_action_list_state();
        self.screen.draw(
            |f| {
                if cur_focus == "input_dialog" {
                    let mut constraints: Vec<Constraint> = Vec::new();
                    for _ in 0..input_size {
                        constraints.push(
                            Constraint::Ratio(1, input_size as u32)
                        );
                    }
                    let outer_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(2),
                                Constraint::Min(1)
                            ]
                        )
                        .split(f.size());
                    f.render_widget(
                        Paragraph::new("Press UP/DOWN to move between dialogs, LEFT/RIGHT to move cursor and ENTER to submit").wrap(Wrap { trim: true }),
                        outer_layout[0]
                    );
                    let inner_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(constraints)
                        .split(outer_layout[1]);

                    for (i, paragraph) in paragraphs.into_iter().enumerate() {
                        f.render_widget(
                            paragraph, inner_layout[i]
                        );
                        match input_dialog_cur_input_ind {
                            Some(i) => {
                                // FIXME: calc (cursor_length)/(screen_width - 2)
                                f.set_cursor(
                                    inner_layout[i].x + input_dialog_cursor + 1,
                                    inner_layout[i].y + 1
                                );
                            },
                            None => {}
                        }
                    }
                } else {
                    // The most outer top and bottom rectangles
                    let windows_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Percentage(70),
                            Constraint::Percentage(30)
                            // Constraint::Min(4)
                        ].as_ref())
                        .split(f.size());

                    let windows_layout2 = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Percentage(90),
                            Constraint::Percentage(10)
                        ].as_ref())
                        .split(windows_layout[1]);

                    f.render_stateful_widget(result_table_widget, windows_layout[0], &mut result_table_state.clone());
                    f.render_widget(display_panel_widget, windows_layout2[0]);
                    f.render_stateful_widget(action_list_widget, windows_layout2[1], &mut action_list_state.clone());
                }
            }
        ).unwrap();
    }

    /// Update input_dialog from the current chosen bookmark in result_table
    fn update_input_dialog(&mut self) {
        let inputs = self.wm.get_selected_item_as_tuple();
        self.wm.set_input_dialog(inputs);
    }
}
