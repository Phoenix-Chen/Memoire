use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap}
};

use super::widget_trait::WidgetTrait;


#[derive(Clone)]
pub struct Input {
    name: String,
    input: String,
    cursor_ind: Option<usize>
}

impl WidgetTrait for Input {
    fn on_focus (&mut self) {
        self.cursor_ind = Some(self.input.len() - 1)
    }

    fn on_blur (&mut self) {
        self.cursor_ind = None;
    }
}

impl Input {
    pub fn new(name: &str) -> Input {
        Input {
            name: name.to_string(),
            input: " ".to_owned(), // Extra space for cursor
            cursor_ind: None
        }
    }

    pub fn update_input(&mut self, character: char) {
        if let Some(ind) = self.cursor_ind {
            self.input.insert(ind, character);
            self.cursor_ind = Some(ind + 1)
        }
    }

    pub fn backspace(&mut self) {
        if let Some(ind) = self.cursor_ind {
            if ind > 0 {
                self.input.remove(ind - 1);  // Subtract 1 due to cursor index
                self.cursor_ind = Some(ind - 1);
            }
        }
    }

    pub fn left(&mut self) {
        if let Some(ind) = self.cursor_ind {
            if ind > 0 {
                self.cursor_ind = Some(ind - 1);
            }
        }
    }

    pub fn right(&mut self) {
        if let Some(ind) = self.cursor_ind {
            if ind < self.get_input().len() {
                self.cursor_ind = Some(ind + 1);
            }
        }
    }

    fn get_text(&self) -> Spans {
        // Note use self.input directly here for cursor highlight
        return match self.cursor_ind {
            Some(index) => Spans::from(vec![
                Span::styled(
                    &self.input[0..index],
                    Style::default().fg(Color::LightYellow)
                ),
                Span::styled(
                    &self.input[index..index+1],
                    Style::default().bg(Color::White)
                ),
                Span::styled(
                    &self.input[index+1..],
                    Style::default().fg(Color::LightYellow)
                )
            ]),
            None => Spans::from(vec![Span::styled(
                self.get_input(),
                Style::default().fg(Color::White)
            )])
        }
    }

    pub fn get_widget(&self) -> Paragraph {
        Paragraph::new(
            self.get_text()
        )
    }

    pub fn get_input(&self) -> &str {
        &self.input[0..self.input.len() - 1]  // Exclude the extra space
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_input(&mut self, input: &str) {
        self.input = input.to_string();
        self.input.push(' ');  // Extra space for cursor
    }
}


// Rename to InputGroup
pub struct InputDialog {
    inputs: Vec<Input>,
    cur_input: Option<usize>
}


impl WidgetTrait for InputDialog {
    fn on_focus(&mut self) {
        self.cur_input = Some(0);
        self.update_input_focus();
    }

    fn on_blur (&mut self) {
        self.cur_input = None;
        self.update_input_focus();
    }
}


impl InputDialog {
    pub fn new(input_names: Vec<&str>) -> InputDialog {
        InputDialog {
            inputs: input_names.into_iter().map(|name| Input::new(name)).collect(),
            cur_input: None
        }
    }

    pub fn set_inputs(&mut self, inputs: Vec<String>) {
        if self.inputs.len() != inputs.len() {
            panic!("Inputs length not equal.")  // TODO: Better handling
        }
        for index in 0..self.inputs.len() {
            self.inputs[index].set_input(&inputs[index]);
        }
    }

    pub fn get_widgets(&self) -> Vec<Paragraph> {
        // let paragraphs: Vec<Paragraph> = self.inputs.clone().into_iter().enumerate().map(
        //     |(index, (key, val))| {
        //         let mut paragraph = Paragraph::new(val)
        //             .block(
        //                 Block::default()
        //                     .title(key)
        //                     .borders(Borders::ALL)
        //             )
        //             .wrap(Wrap { trim: true });
        //         if let Some(cur_input) = self.cur_input {
        //             if cur_input == index {
        //                 paragraph = paragraph.style(
        //                     Style::default().fg(Color::Yellow)
        //                 );
        //             }
        //         }
        //         paragraph
        //     }
        // ).collect();
        // paragraphs
        (&self.inputs).into_iter().map(|input| input.get_widget()).collect()        
    }

    pub fn get_inputs_size(&self) -> usize {
        self.inputs.len()
    }

    pub fn update_input(&mut self, character: char) {
        self.inputs[self.cur_input.unwrap()].update_input(character);
    }

    pub fn backspace(&mut self) {
        self.inputs[self.cur_input.unwrap()].backspace();
    }

    pub fn left(&mut self) {
        self.inputs[self.cur_input.unwrap()].left();
    }

    pub fn right(&mut self) {
        self.inputs[self.cur_input.unwrap()].right();
    }

    pub fn up(&mut self) {
        if let Some(ind) = self.cur_input {
            if ind > 0 {
                self.cur_input = Some(ind - 1);
                self.update_input_focus();
            }
        }
    }

    pub fn down(&mut self) {
        if let Some(ind) = self.cur_input {
            if ind < self.inputs.len() - 1 {
                self.cur_input = Some(ind + 1);
                self.update_input_focus();
            }
        }
    }

    pub fn get_cur_input_ind(&self) -> Option<usize> {
        self.cur_input
    }

    pub fn get_inputs_as_strings(&self) -> Vec<String> {
        (&self.inputs).into_iter().map(|input| input.get_input().to_string()).collect()
    }

    pub fn get_inputs_names(&self) -> Vec<String> {
        (&self.inputs).into_iter().map(|input| input.get_name().to_string()).collect()
    }

    fn update_input_focus(&mut self) {
        for index in 0..self.inputs.len() {
            self.inputs[index].on_blur();
            if let Some(cur_index) = self.cur_input {
                if cur_index == index {
                    self.inputs[self.cur_input.unwrap()].on_focus();
                }
            }
        }
    }
}
