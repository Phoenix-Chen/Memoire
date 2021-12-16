mod action_list;
mod input_dialog;
mod result_table;
mod widget_trait;

use std::collections::HashMap;

use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListState, Table, TableState, Paragraph, Wrap}
};

use crate::collection::bookmark::Bookmark;
use crate::collection::jq::SearchResult;
use action_list::ActionList;
use input_dialog::{Input, InputDialog};
use result_table::ResultTable;
pub use action_list::Action;
pub use action_list::ACTIONS;
use widget_trait::WidgetTrait;


enum Widget {
    ActionList(ActionList),
    ResultTable(ResultTable),
    InputDialog(InputDialog)
}

impl WidgetTrait for Widget {
    // TODO: Must be a cleaner way
    fn on_focus(&mut self) {
        match self {
            Widget::ActionList(action_list) => action_list.on_focus(),
            Widget::InputDialog(input_dialog) => input_dialog.on_focus(),
            Widget::ResultTable(result_table) => result_table.on_focus()
        }
    }

    fn on_blur(&mut self) {
        match self {
            Widget::ActionList(action_list) => action_list.on_blur(),
            Widget::InputDialog(input_dialog) => input_dialog.on_blur(),
            Widget::ResultTable(result_table) => result_table.on_blur()
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
        self.set_input_dialog(inputs);
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
    fn get_result_table(&self) -> &ResultTable {
        match self.widgets.get(RESULT_TABLE).unwrap() {
            Widget::ResultTable(result_table) => {
                result_table
            },
            _ => {
                panic!("No result_table in self.widgets!!!")
            }
        }
    }

    /// Returns a tui::widgets::Table from result_table
    pub fn get_result_table_widget(&self) -> Table {
        self.get_result_table().get_widget()
    }

    // Returns an immutable tui::widgets::TableState reference from result_table
    pub fn get_result_table_state(&self) -> &TableState {
        self.get_result_table().get_state()
    }

    pub fn get_result_table_state_selected(&self) -> Option<usize> {
        self.get_result_table_state().selected()
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

    fn get_action_list(&self) -> &ActionList {
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

    pub fn get_action_list_widget(&self) -> List {
        self.get_action_list().get_widget()
    }

    pub fn get_action_list_state(&self) -> &ListState {
        self.get_action_list().get_state()
    }

    pub fn get_action_list_state_selected(&self) -> Option<usize> {
        self.get_action_list().get_state().selected()
    }

    pub fn reset_action_list_state(&mut self) {
        self.get_mut_action_list().reset();
    }

    pub fn get_mut_input_dialog(&mut self) -> &mut InputDialog {
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

    pub fn get_input_dialog_widgets(&self) -> Vec<Paragraph> {
        self.get_input_dialog().get_widgets()
    }

    pub fn set_input_dialog(&mut self, inputs: Vec<String>) {
        self.get_mut_input_dialog().set_inputs(inputs);
    }

    pub fn get_input_dialog_input_size(&self) -> usize {
        self.get_input_dialog().get_inputs_size()
    }

    pub fn get_input_dialog_cur_input_ind(&self) -> Option<usize> {
        self.get_input_dialog().get_cur_input_ind()
    }

    pub fn get_input_dialog_inputs(&self) -> Vec<String> {
        self.get_input_dialog().get_inputs_as_strings()
    }

    pub fn get_input_dialog_cursor(&self) -> usize {
        self.get_input_dialog().get_cursor()
    }

    pub fn update_input_dialog_input(&mut self, character: char) {
        self.get_mut_input_dialog().update_input(character);
    }

    pub fn get_display_panel_widget(&self) -> Paragraph {
        let display_panel: Paragraph = match self.get_result_table_state_selected() {
            Some(result_table_state) => {
                let result_table = self.get_result_table();
                Paragraph::new(
                    bookmark_to_spans(
                        result_table.get_item(result_table_state).get_bookmark()
                    )
                )
            },
            None => {
                Paragraph::new("")
            }
        };
        display_panel.block(Block::default().borders(Borders::ALL)).wrap(Wrap { trim: true })
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

    pub fn key_up(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ActionList(action_list) => action_list.up(),
            Widget::ResultTable(result_table) => result_table.up(),
            Widget::InputDialog(input_dialog) => input_dialog.up()
        }
    }

    pub fn key_down(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ActionList(action_list) => action_list.down(),
            Widget::ResultTable(result_table) => result_table.down(),
            Widget::InputDialog(input_dialog) => input_dialog.down()
        }
    }

    pub fn key_left(&mut self) {
        if let Widget::InputDialog(input_dialog) = self.widgets.get_mut(&self.cur_focus).unwrap() {
            input_dialog.left()
        }
    }

    pub fn key_right(&mut self) {
        if let Widget::InputDialog(input_dialog) = self.widgets.get_mut(&self.cur_focus).unwrap() {
            input_dialog.right()
        }
    }

    pub fn key_backspace(&mut self) {
        match self.widgets.get_mut(&self.cur_focus).unwrap() {
            Widget::ActionList(action_list) => {
                action_list.reset();
                self.set_cur_focus(RESULT_TABLE);
            },
            Widget::InputDialog(input_dialog) => {
                input_dialog.backspace();
            }
            _ => {}
        }
    }
}


fn bookmark_to_spans(bookmark: &Bookmark) -> Vec<Spans> {
    vec![
        Spans::from(vec![
            Span::styled("Command: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(bookmark.get_command(), Style::default().fg(Color::Red))
        ]),
        Spans::from(vec![
            Span::styled("Annotation: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(bookmark.get_annotation(), Style::default().fg(Color::Green))
        ]),
        Spans::from(vec![
            Span::styled("Tags: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(bookmark.get_tags_as_string(", "), Style::default().fg(Color::Yellow))
        ]),
        Spans::from(vec![
            Span::styled("Collection: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled(bookmark.get_collection(), Style::default().fg(Color::Cyan))
        ]),
    ]
}
