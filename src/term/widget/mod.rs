mod action_list;
mod input_dialog;
mod result_table;
mod widget_trait;

use std::collections::HashMap;

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap}
};

use crate::collection::bookmark::Bookmark;
use crate::collection::jq::{SearchResult, search};
use crate::collection::util::get_collection_dir_path;
use action_list::ActionList;
use input_dialog::{Input, InputDialog};
use result_table::ResultTable;
pub use action_list::Action;
pub use action_list::ACTIONS;
pub use widget_trait::WidgetTrait;


enum Widget {
    ActionList(ActionList),
    ResultTable(ResultTable),
    SearchBar(Input),
    InputDialog(InputDialog)
}

impl WidgetTrait for Widget {
    // TODO: Must be a cleaner way
    fn on_focus(&mut self) {
        match self {
            Widget::ActionList(action_list) => action_list.on_focus(),
            Widget::InputDialog(input_dialog) => input_dialog.on_focus(),
            Widget::ResultTable(result_table) => result_table.on_focus(),
            Widget::SearchBar(input) => input.on_focus()
        }
    }

    fn on_blur(&mut self) {
        match self {
            Widget::ActionList(action_list) => action_list.on_blur(),
            Widget::InputDialog(input_dialog) => input_dialog.on_blur(),
            Widget::ResultTable(result_table) => result_table.on_blur(),
            Widget::SearchBar(input) => input.on_blur()
        }
    }
}


pub struct WidgetManager {
    widgets: HashMap<String, Widget>,
    cur_focus: String  // current on focus widget
}


pub const ACTION_LIST: &str = "action_list";
pub const INPUT_DIALOG: &str = "input_dialog";
pub const RESULT_TABLE: &str = "result_table";
pub const SEARCH_BAR: &str = "search_bar";


impl WidgetTrait for WidgetManager {
    fn key_char(&mut self, character: char) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ResultTable(_) => {
                self.set_cur_focus(SEARCH_BAR);
                self.key_char(character);
            },
            Widget::InputDialog(input_dialog) => input_dialog.key_char(character),
            Widget::SearchBar(input) => {
                input.key_char(character);
                self.update_result_table_from_search_bar();
            },
            _ => {}
        }
    }

    fn key_up(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ActionList(action_list) => action_list.key_up(),
            Widget::ResultTable(result_table) => result_table.key_up(),
            Widget::InputDialog(input_dialog) => input_dialog.key_up(),
            Widget::SearchBar(_) => {
                self.set_cur_focus(RESULT_TABLE);
                self.key_up();
            }
        }
    }

    fn key_down(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ActionList(action_list) => action_list.key_down(),
            Widget::ResultTable(result_table) => result_table.key_down(),
            Widget::InputDialog(input_dialog) => input_dialog.key_down(),
            Widget::SearchBar(_) => {
                self.set_cur_focus(RESULT_TABLE);
                self.key_down();
            }
        }
    }

    fn key_left(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ResultTable(_) => {
                self.set_cur_focus(SEARCH_BAR);
                self.key_left();
            },
            Widget::InputDialog(input_dialog) => input_dialog.key_left(),
            Widget::SearchBar(input) => input.key_left(),
            _ => {}
        }

    }

    fn key_right(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ResultTable(_) => {
                self.set_cur_focus(SEARCH_BAR);
                self.key_right();
            },
            Widget::InputDialog(input_dialog) => input_dialog.key_right(),
            Widget::SearchBar(input) => input.key_right(),
            _ => {}
        }
    }

    fn key_backspace(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::InputDialog(input_dialog) => {
                input_dialog.key_backspace();
            },
            Widget::SearchBar(input) => {
                input.key_backspace();
                self.update_result_table_from_search_bar();
            },
            Widget::ResultTable(_) => {
                self.set_cur_focus(SEARCH_BAR);
                self.key_backspace();
            },
            _ => {}
        }
    }

    fn key_esc(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ActionList(action_list) => {
                action_list.reset();
                self.set_cur_focus(RESULT_TABLE);
            },
            Widget::InputDialog(_) => {
                self.set_cur_focus(SEARCH_BAR);
            },
            Widget::ResultTable(_) => {
                self.set_cur_focus(SEARCH_BAR);
            }
            _ => {}
        }
    }
}


impl WidgetManager {
    pub fn new() -> WidgetManager {
        let mut widgets: HashMap<String, Widget> = HashMap::new();
        widgets.insert(
            ACTION_LIST.to_string(),
            Widget::ActionList(
                ActionList::new(ACTIONS.to_vec())
            )
        );
        widgets.insert(
            SEARCH_BAR.to_string(),
            Widget::SearchBar(
                Input::new("Search").prefix(
                    Span::styled(
                        " > ",
                        Style::default().fg(Color::LightYellow)
                    )
                ).placeholder(
                    Span::styled(
                        "Type to search | Arrow to move | Enter to select | Esc to go back",
                        Style::default().fg(Color::Gray)
                    )
                )
            )
        );
        widgets.insert(
            INPUT_DIALOG.to_string(),
            Widget::InputDialog(
                InputDialog::new(vec!["Command", "Annotation", "Tags", "Collection"])
            )
        );
        widgets.insert(RESULT_TABLE.to_string(), Widget::ResultTable(ResultTable::default()));
        WidgetManager {
            widgets,
            cur_focus: RESULT_TABLE.to_string()
        }
    }

    /// Update result_table with passed input
    pub fn update_result_table(&mut self, results: Vec<SearchResult>) {
        self.get_mut_result_table().update_results(results);
    }

    /// Reset the state of result_table
    pub fn reset_result_table_state(&mut self) {
        self.get_mut_result_table().reset_state();
    }

    pub fn update_input_dialog_from_result_table(&mut self) {
        let result_table = self.get_result_table();
        let inputs = result_table.get_item(result_table.get_state().selected().unwrap()).get_bookmark().to_vec();
        self.get_mut_input_dialog().set_inputs(inputs);
        // Note: Do not reset result_table state here
        // Input_dialog will use result_table state to determine add/edit
    }

    /// Returns a mutable reference to the result_table
    fn get_mut_result_table(&mut self) -> &mut ResultTable {
        match self.widgets.get_mut(RESULT_TABLE).unwrap() {
            Widget::ResultTable(result_table) => {
                result_table
            },
            _ => {
                panic!("No result_table in self.widgets!!!")
            }
        }
    }

    /// Returns an immutable reference to result_table
    pub fn get_result_table(&self) -> &ResultTable {
        match self.widgets.get(RESULT_TABLE).unwrap() {
            Widget::ResultTable(result_table) => {
                result_table
            },
            _ => {
                panic!("No result_table in self.widgets!!!")
            }
        }
    }

    pub fn get_selected_item_index(&self) -> Option<usize> {
        let result_table = self.get_result_table();
        result_table.get_state()
                    .selected()
                    .map(
                        |state| result_table.get_item(state)
                                            .get_index()
                    )
    }

    pub fn get_selected_item_collection(&self) -> &str {
        let result_table = self.get_result_table();
        result_table.get_item(result_table.get_state().selected().unwrap()).get_bookmark().get_collection()
    }

    pub fn get_selected_item_command(&self) -> &str {
        let result_table = self.get_result_table();
        result_table.get_item(result_table.get_state().selected().unwrap()).get_bookmark().get_command()
    }

    pub fn get_action_list(&self) -> &ActionList {
        match self.widgets.get(ACTION_LIST).unwrap() {
            Widget::ActionList(action_list) => {
                action_list
            },
            _ => {
                panic!("No action_list in self.widgets!!!")
            }
        }
    }

    fn get_mut_action_list(&mut self) -> &mut ActionList {
        match self.widgets.get_mut(ACTION_LIST).unwrap() {
            Widget::ActionList(action_list) => {
                action_list
            },
            _ => {
                panic!("No action_list in self.widgets!!!")
            }
        }
    }

    pub fn get_action_list_state_selected(&self) -> Option<usize> {
        self.get_action_list().get_state().selected()
    }

    pub fn reset_action_list_state(&mut self) {
        self.get_mut_action_list().reset();
    }

    fn get_mut_input_dialog(&mut self) -> &mut InputDialog {
        match self.widgets.get_mut(INPUT_DIALOG).unwrap() {
            Widget::InputDialog(input_dialog) => {
                input_dialog
            },
            _ => {
                panic!("No input_dialog in self.widgets!!!")
            }
        }
    }

    pub fn get_input_dialog(&self) -> &InputDialog {
        match self.widgets.get(INPUT_DIALOG).unwrap() {
            Widget::InputDialog(input_dialog) => {
                input_dialog
            },
            _ => {
                panic!("No input_dialog in self.widgets!!!")
            }
        }
    }

    pub fn set_input_dialog_inputs(&mut self, inputs: Vec<String>) {
        self.get_mut_input_dialog().set_inputs(inputs);
    }

    pub fn get_display_panel_widget(&self) -> Paragraph {
        let display_panel: Paragraph = match self.get_result_table().get_state().selected() {
            Some(result_table_state) => {
                let result_table = self.get_result_table();
                Paragraph::new(
                    bookmark_to_spans(
                        result_table.get_item(result_table_state).get_bookmark()
                    )
                )
            },
            None => {
                Paragraph::new(
                    vec![
                        Spans::from(vec![Span::styled("Hints*", Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))]),
                        Spans::from(vec![
                            Span::styled("Ctrl-c", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
                            Span::styled(" to quit anytime", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD))
                        ]),
                        Spans::from(vec![
                            Span::styled("Ctrl-a", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)),
                            Span::styled(" to add", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD))
                        ])
                    ]
                )
            }
        };
        display_panel.block(Block::default().borders(Borders::ALL)).wrap(Wrap { trim: true, break_word: false })
    }

    pub fn get_search_bar(&self) -> &Input {
        match self.widgets.get(SEARCH_BAR).unwrap() {
            Widget::SearchBar(input) => {
                input
            },
            _ => {
                panic!("No search_bar in self.widgets!!!")
            }
        }
    }

    // Set the current on focus widget to the the passed string slices
    pub fn set_cur_focus(&mut self, new_focus: &str) {
        if self.cur_focus != new_focus {
            self.widgets.get_mut(&self.cur_focus).unwrap().on_blur();
            self.widgets.get_mut(new_focus).unwrap().on_focus();
            self.cur_focus = new_focus.to_owned();
        }
    }

    // Returns a string slices of current on focus widget
    pub fn get_cur_focus(&self) -> &str {
        &self.cur_focus
    }

    fn update_result_table_from_search_bar(&mut self) {
        self.get_mut_result_table().reset_state();
        let keywords = self.get_search_bar().get_input().to_string();
        self.get_mut_result_table().update_results(
            search(
                &get_collection_dir_path(),
                &keywords.trim().split(' ').collect::<Vec<&str>>()
            )
        );
    }
}


fn bookmark_to_spans(bookmark: &Bookmark) -> Vec<Spans> {
    vec![
        Spans::from(vec![
            Span::styled("Command: ", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::styled(bookmark.get_command(), Style::default().fg(Color::LightRed))
        ]),
        Spans::from(vec![
            Span::styled("Annotation: ", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
            Span::styled(bookmark.get_annotation(), Style::default().fg(Color::LightGreen))
        ]),
        Spans::from(vec![
            Span::styled("Tags: ", Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD)),
            Span::styled(bookmark.get_tags_as_string(", "), Style::default().fg(Color::LightYellow))
        ]),
        Spans::from(vec![
            Span::styled("Collection: ", Style::default().fg(Color::LightMagenta).add_modifier(Modifier::BOLD)),
            Span::styled(bookmark.get_collection(), Style::default().fg(Color::LightMagenta))
        ]),
    ]
}
