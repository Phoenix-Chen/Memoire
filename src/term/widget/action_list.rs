use std::fmt::{Display, Formatter, Result, Debug};

use tui::{
    layout::Corner,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState}
};

use super::widget_trait::WidgetTrait;


#[derive(Clone, Debug)]
pub enum Action {
    Copy,
    Edit,
    Delete
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> Result {
        Debug::fmt(self, f)
    }
}

pub const ACTIONS: [Action; 3] = [Action::Copy, Action::Edit, Action::Delete];

pub struct ActionList {
    state: ListState,
    items: Vec<String>
}


impl WidgetTrait for ActionList {
    fn on_focus(&mut self) {
        self.state.select(Some(0))
    }

    fn on_blur(&mut self) {
        self.state.select(None);
    }
}


impl ActionList {
    pub fn new(actions: Vec<Action>) -> ActionList {
        ActionList {
            state: ListState::default(),
            items: actions.iter().map(
                |i| i.to_string()
            ).collect()
        }
    }

    pub fn get_widget(&self) -> List {
        // Define selected action style
        let selected_action_style = Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD);

        // Set list
        let action_options: Vec<ListItem> = self.items.iter().map(
                // Note here style is applied on ListItem instead of Span
                |i| ListItem::new(Span::raw(i)).style(Style::default().fg(Color::White))
            ).collect();
        List::new(action_options)
            .block(Block::default().borders(Borders::ALL).title("Actions"))
            .highlight_style(selected_action_style)
            .start_corner(Corner::TopLeft)
    }

    pub fn reset(&mut self) {
        self.state.select(None);
    }

    pub fn up(&mut self) {
        if let Some(ind) = self.state.selected() {
            if ind > 0 {
                self.state.select(Some(ind - 1));
            }
        }
    }

    pub fn down(&mut self) {
        match self.state.selected() {
            Some(ind) => {
                if ind < self.items.len() - 1 {
                    self.state.select(Some(ind + 1));
                }
            },
            None => {
                if !self.items.is_empty() {
                    self.state.select(Some(0));
                }
            }
        }
    }

    pub fn get_state(&self) -> &ListState {
        &self.state
    }
}
