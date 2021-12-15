extern crate clipboard;

mod widget;
mod event;

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

use event::events;
use widget::{Action, WidgetManager, ACTIONS};
use crate::collection::{
    bookmark::Bookmark,
    jq,
    util::{get_collection_dir_path, get_json_path},
};


pub struct Term {
    screen: Terminal<TermionBackend<AlternateScreen<RawTerminal<Stdout>>>>,
    events: mpsc::Receiver<Key>,
    wm: WidgetManager,
}


impl Term {
    pub fn new() -> Term {
        Term {
            screen: Terminal::new(TermionBackend::new(AlternateScreen::from(
                stdout().into_raw_mode().unwrap(),
            )))
            .unwrap(),
            events: events(),
            wm: WidgetManager::new(),
        }
    }

    // FIXME: Dislike this approach. consider using setter or set when initialize
    pub fn get_mut_widget_manager(&mut self) -> &mut WidgetManager {
        &mut self.wm
    }

    pub fn display(&mut self) -> Result<(), mpsc::RecvError> {
        self.screen.hide_cursor().unwrap();
        loop {
            self.draw();

            match self.events.recv()? {
                Key::Ctrl('c') => break,  // Need to match exit_key in util::event for consistent behavior
                Key::Ctrl('a') => {
                    if self.wm.get_cur_focus() != "input_dialog" {
                        self.wm.reset_result_table_state();
                        self.wm.set_input_dialog(Bookmark::default("", "", &vec![]).to_tuple_vec());
                        self.wm.set_cur_focus("input_dialog");
                    }
                }
                Key::Char('\n') => {
                    match self.wm.get_cur_focus() {
                        "action_list" => {
                            if let Some(action_index) = self.wm.get_action_list_state_selected() {
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
                                        self.wm.update_input_dialog();
                                        self.wm.set_cur_focus("input_dialog");
                                    }
                                    Action::Delete => {
                                        match self.wm.get_selected_item_index() {
                                            Some(index) => {
                                                jq::delete(
                                                    &get_json_path(self.wm.get_selected_item_collection()),
                                                    index
                                                );
                                                self.wm.update_result_table(jq::search(
                                                    &get_collection_dir_path(),
                                                    &[self.wm.get_selected_item_collection()]
                                                ))
                                            },
                                            None => {}  // Add error log
                                        }
                                        self.wm.reset_action_list_state();
                                        self.wm.reset_result_table_state();
                                        
                                        self.wm.set_cur_focus("result_table");
                                    }
                                }
                                
                            }
                        }
                        "result_table" => if self.wm.get_result_table_state_selected().is_some() {
                            self.wm.set_cur_focus("action_list");
                            self.wm.key_down();
                        },
                        "input_dialog" => {
                            let bookmark = dialog_inputs_to_bookmark(
                                self.wm.get_input_dialog_inputs()
                            );
                            match self.wm.get_selected_item_index() {
                                Some(index) => {  // Edit
                                    jq::delete(
                                        &get_json_path(self.wm.get_selected_item_collection()),
                                        index
                                    );
                                    jq::add(
                                        &get_json_path(bookmark.get_collection()),
                                        &bookmark,
                                        Some(index)
                                    );
                                },
                                None => {  // Add
                                    jq::add(
                                        &get_json_path(bookmark.get_collection()),
                                        &bookmark,
                                        None
                                    );
                                }
                            };
                            self.wm.reset_action_list_state();
                            self.wm.reset_result_table_state();
                            self.wm.update_result_table(
                                // update this to search by only tag
                                jq::search(
                                    &get_collection_dir_path(),
                                    &[bookmark.get_collection()]
                                )
                            );
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
                        if let Some(i) = input_dialog_cur_input_ind {
                            // FIXME: calc (cursor_length)/(screen_width - 2)
                            f.set_cursor(
                                inner_layout[i].x + input_dialog_cursor + 1,
                                inner_layout[i].y + 1
                            );
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
}

fn dialog_inputs_to_bookmark(inputs: &Vec<(String, String)>) -> Bookmark {
    Bookmark::new(
        &replace_special_chars(&inputs[0].1), 
        &replace_special_chars(&inputs[1].1),
        &inputs[2].1.split(',').map(|s| replace_special_chars(s)).collect(),
        &replace_special_chars(&inputs[3].1)
    )
}

fn replace_special_chars(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}