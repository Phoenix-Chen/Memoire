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
    style::{Color, Style},
    widgets::{Paragraph, Wrap, Block, Borders},
    Terminal,
};

use event::events;
use widget::{Action, WidgetManager, ACTIONS, ACTION_LIST, INPUT_DIALOG, RESULT_TABLE};
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
                Key::Ctrl('c') => break,
                Key::Ctrl('a') => {
                    if self.wm.get_cur_focus() != INPUT_DIALOG {
                        self.wm.set_input_dialog_inputs(
                            Bookmark::default().to_vec()
                        );
                        self.wm.set_cur_focus(INPUT_DIALOG);
                    }
                }
                Key::Char('\n') => {
                    match self.wm.get_cur_focus() {
                        ACTION_LIST => {
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
                                        self.wm.update_input_dialog_from_result_table();
                                        self.wm.set_cur_focus(INPUT_DIALOG);
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
                                        
                                        self.wm.set_cur_focus(RESULT_TABLE);
                                    }
                                }
                                
                            }
                        }
                        RESULT_TABLE => if self.wm.get_result_table().get_state().selected().is_some() {
                            self.wm.set_cur_focus(ACTION_LIST);
                        },
                        INPUT_DIALOG => {
                            let bookmark = dialog_inputs_to_bookmark(
                                self.wm.get_input_dialog().get_inputs_as_strings()
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
                            self.wm.set_cur_focus(RESULT_TABLE);
                        }
                        _ => {}
                    }
                }
                Key::Char(character) => {
                    self.wm.key_char(character);
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
        let num_of_inputs = self.wm.get_input_dialog().get_inputs_size();
        let input_titles = self.wm.get_input_dialog().get_inputs_names();
        let cur_focus_input = self.wm.get_input_dialog().get_cur_input_ind();
        let paragraphs = self.wm.get_input_dialog().get_widgets();
        // let input_dialog_cur_input_ind = self.wm.get_input_dialog_cur_input_ind();
        // let input_dialog_cursor = self.wm.get_input_dialog_cursor() as u16;
        // For render
        let search_bar = self.wm.get_search_bar().get_widget().block(
            Block::default().borders(Borders::ALL)
        );
        let result_table_widget = self.wm.get_result_table().get_widget();
        let result_table_state = self.wm.get_result_table().get_state();
        let display_panel_widget = self.wm.get_display_panel_widget();
        let action_list_widget = self.wm.get_action_list().get_widget();
        let action_list_state = self.wm.get_action_list().get_state();
        self.screen.draw(
            |f| {
                if cur_focus == INPUT_DIALOG {
                    let mut constraints: Vec<Constraint> = Vec::new();
                    for _ in 0..num_of_inputs {
                        constraints.push(
                            Constraint::Ratio(1, num_of_inputs as u32)
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
                            paragraph.block(
                                Block::default().title(
                                    input_titles[i].to_owned()
                                ).borders(Borders::ALL)
                            ).style(
                                // FIXME: Must be a cleaner way
                                if let Some(cur_input_index) = cur_focus_input {
                                    if i == cur_input_index {
                                        Style::default().fg(Color::LightYellow)
                                    } else {
                                        Style::default().fg(Color::White)
                                    }
                                } else {
                                    Style::default().fg(Color::White)
                                }
                            ),
                            // ).wrap(Wrap { trim: false }),  // FIXME
                            inner_layout[i]
                        );
                        // if let Some(i) = input_dialog_cur_input_ind {
                        //     // FIXME: calc (cursor_length)/(screen_width - 2)
                        //     f.set_cursor(
                        //         inner_layout[i].x + input_dialog_cursor + 1,
                        //         inner_layout[i].y + 1
                        //     );
                        // }
                    }
                } else {
                    // The most outer top and bottom rectangles
                    let windows_layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Min(3),
                            Constraint::Percentage(60),
                            Constraint::Percentage(40)
                            // Constraint::Min(4)
                        ].as_ref())
                        .split(f.size());

                    let windows_layout2 = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Percentage(90),
                            Constraint::Percentage(10)
                        ].as_ref())
                        .split(windows_layout[2]);

                    f.render_widget(search_bar, windows_layout[0]);
                    f.render_stateful_widget(result_table_widget, windows_layout[1], &mut result_table_state.clone());
                    f.render_widget(display_panel_widget, windows_layout2[0]);
                    f.render_stateful_widget(action_list_widget, windows_layout2[1], &mut action_list_state.clone());
                }
            }
        ).unwrap();
    }
}

fn dialog_inputs_to_bookmark(inputs: Vec<String>) -> Bookmark {
    Bookmark::new(
        &replace_special_chars(&inputs[0]), 
        &replace_special_chars(&inputs[1]),
        &inputs[2].split(',').map(|s| replace_special_chars(s).trim().to_owned()).collect(),
        &replace_special_chars(&inputs[3])
    )
}

fn replace_special_chars(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}