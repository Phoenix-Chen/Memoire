use tui::{
    style::{Color,Style},
    widgets::{Block, Borders, Paragraph, Wrap}
};


pub struct InputDialog {
    inputs: Vec<(String, String)>,
    cur_input: Option<usize>,
    cursor_ind: usize
}


impl InputDialog {
    pub fn new(inputs: Vec<(String, String)>) -> InputDialog {
        InputDialog {
            inputs,
            cur_input: None,
            cursor_ind: 0
        }
    }

    pub fn set_inputs(&mut self, inputs: Vec<(String, String)>) {
        if !inputs.is_empty() {
            self.cur_input = Some(0);
            self.cursor_ind = inputs[0].1.len();
        } else {
            self.cur_input = None;
        }
        self.inputs = inputs;
    }

    pub fn get_widgets(&self) -> Vec<Paragraph<'_>> {
        let paragraphs: Vec<Paragraph> = self.inputs.clone().into_iter().enumerate().map(
            |(index, (key, val))| {
                let mut paragraph = Paragraph::new(val)
                    .block(
                        Block::default()
                            .title(key)
                            .borders(Borders::ALL)
                    )
                    .wrap(Wrap { trim: true });
                if let Some(cur_input) = self.cur_input {
                    if cur_input == index {
                        paragraph = paragraph.style(
                            Style::default().fg(Color::Yellow)
                        );
                    }
                }
                paragraph
            }
        ).collect();
        paragraphs
    }

    pub fn get_inputs_size(&self) -> usize {
        self.inputs.len()
    }

    pub fn update_input(&mut self, character: char) {
        if let Some(ind) = self.cur_input {
            let mut new_input = self.inputs[ind].1.to_owned();
            // FIXME: Check new_input size in case of overflow
            new_input.insert(self.cursor_ind, character);
            self.cursor_ind += 1;
            self.inputs[ind] = (
                self.inputs[ind].0.to_owned(),
                new_input
            )
        }
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor_ind
    }

    pub fn backspace(&mut self) {
        if let Some(ind) = self.cur_input {
            if self.cursor_ind > 0 {
                self.cursor_ind -= 1;
                let mut new_input = self.inputs[ind].1.to_owned();
                new_input.remove(self.cursor_ind);
                self.inputs[ind] = (
                    self.inputs[ind].0.to_owned(),
                    new_input
                )
            }
        }
    }

    pub fn left(&mut self) {
        if self.cur_input.is_some() && self.cursor_ind > 0 {
            self.cursor_ind -= 1;
        }
    }

    pub fn right(&mut self) {
        if let Some(ind) = self.cur_input {
            if self.cursor_ind < self.inputs[ind].1.len() {
                self.cursor_ind += 1;
            }
        }
    }

    pub fn up(&mut self) {
        if let Some(ind) = self.cur_input {
            if ind > 0 {
                self.cur_input = Some(ind - 1);
                self.cursor_ind = self.inputs[ind - 1].1.len();
            }
        }
    }

    pub fn down(&mut self) {
        if let Some(ind) = self.cur_input {
            if ind < self.inputs.len() - 1 {
                self.cur_input = Some(ind + 1);
                self.cursor_ind = self.inputs[ind + 1].1.len();
            }
        }
    }

    pub fn get_cur_input_ind(&self) -> Option<usize> {
        self.cur_input
    }

    pub fn get_inputs(&self) -> &Vec<(String, String)> {
        &self.inputs
    }
}
