use std::collections::HashSet;

use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState}
};

use crate::collection::jq::SearchResult;


pub struct ResultTable {
    pub state: TableState,
    items: Vec<SearchResult>,
}


impl ResultTable {
    pub fn new(results: HashSet<SearchResult>) -> ResultTable {
        ResultTable {
            state: TableState::default(),
            items: hashset_to_vec(&results)
        }
    }

    pub fn default() -> ResultTable {
        ResultTable::new(HashSet::new())
    }

    pub fn update_results(&mut self, results: Vec<SearchResult>) {
        self.items = results;
    }

    pub fn get_widget(&self) -> Table {
        // Define selected style for table row
        let selected_row_style: Style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

        // Set table
        let header: Row = Row::new(
            ["Command", "Annotation", "Tags", "Collection"]
        ).style(
            Style::default().fg(Color::Green)
                            .add_modifier(Modifier::BOLD)
        );
        let body_rows = self.items.iter().map(
            |i| Row::new(
                i.get_bookmark().to_vec().into_iter()
            ).style(
                Style::default().fg(Color::White)
            )
        );
        let t = Table::new(body_rows)
            .block(Block::default().borders(Borders::ALL).title("Results"))
            .highlight_style(selected_row_style)
            // .highlight_symbol(">> ")
            .header(header)
            .column_spacing(1)
            .widths(&[
                Constraint::Percentage(35),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
                Constraint::Percentage(15),
            ]);
        t
    }

    pub fn up(&mut self) {
        match self.state.selected() {
            Some(ind) => {
                if ind > 0 {
                    self.state.select(Some(ind - 1));
                }
            },
            None => {}
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
                if self.items.len() > 0 {
                    self.state.select(Some(0));
                }
            }
        }
    }

    pub fn reset_state(&mut self) {
        self.state = TableState::default();
    }

    pub fn get_state(&self) -> &TableState {
        &self.state
    }

    pub fn get_item(&self, ind: usize) -> &SearchResult {
        return &self.items[ind]
    }
}

fn hashset_to_vec(results: &HashSet<SearchResult>) -> Vec<SearchResult> {
    let mut v = Vec::new();
    for r in results.iter() {
        v.push(r.to_owned());
    }
    v
}
