use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use super::widget_trait::WidgetTrait;


#[derive(Clone)]
pub struct Input {
    name: String,
    input: String,
    cursor_ind: Option<usize>,
    prefix: Option<Span<'static>>,
    placeholder: Option<Span<'static>>,
}


impl WidgetTrait for Input {
    fn on_focus(&mut self) {
        self.cursor_ind = Some(self.input.len() - 1)
    }

    fn on_blur(&mut self) {
        self.cursor_ind = None;
    }

    fn key_char(&mut self, character: char) {
        if character == '\t' {
            return self.key_char(' ');
        }
        if let Some(ind) = self.cursor_ind {
            self.input.insert(ind, character);
            self.cursor_ind = Some(ind + 1)
        }
    }

    fn key_left(&mut self) {
        if let Some(ind) = self.cursor_ind {
            if ind > 0 {
                self.cursor_ind = Some(ind - 1);
            }
        }
    }

    fn key_right(&mut self) {
        if let Some(ind) = self.cursor_ind {
            if ind < self.get_input().len() {
                self.cursor_ind = Some(ind + 1);
            }
        }
    }

    fn key_backspace(&mut self) {
        if let Some(ind) = self.cursor_ind {
            if ind > 0 {
                self.input.remove(ind - 1);  // Subtract 1 due to cursor index
                self.cursor_ind = Some(ind - 1);
            }
        }
    }
}


impl Input {
    pub fn new(name: &str) -> Input {
        Input {
            name: name.to_string(),
            input: " ".to_owned(), // Extra space for cursor
            cursor_ind: None,
            prefix: None,
            placeholder: None
        }
    }

    pub fn prefix(mut self, prefix: Span<'static>) -> Input {
        self.prefix = Some(prefix);
        self
    }

    pub fn placeholder(mut self, placeholder: Span<'static>) -> Input {
        self.placeholder = Some(placeholder);
        self
    }

    fn get_text(&self) -> Spans {
        let mut spans: Vec<Span> = Vec::new();
        if let Some(span) = &self.prefix {
            spans.push(span.clone());
        }
        if let Some(span) = &self.placeholder {
            if self.get_input().len() == 0 {
                spans.push(span.clone());
                return Spans::from(spans);
            }
        }
        // Note use self.input directly here for cursor highlight
        match self.cursor_ind {
            Some(index) => spans.extend_from_slice(&[
                Span::styled(
                    &self.input[0..index],
                    Style::default().fg(Color::LightYellow)
                ),
                Span::styled(
                    &self.input[index..index+1],
                    Style::default().bg(Color::White).fg(Color::Black)
                ),
                Span::styled(
                    &self.input[index+1..],
                    Style::default().fg(Color::LightYellow)
                )
            ]),
            None => spans.push(
                Span::styled(
                    self.get_input(),
                    Style::default().fg(Color::White)
                )
            )
        };
        Spans::from(spans)
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

    fn on_blur(&mut self) {
        self.cur_input = None;
        self.update_input_focus();
    }

    fn key_char(&mut self, character: char) {
        self.inputs[self.cur_input.unwrap()].key_char(character);
    }

    fn key_up(&mut self) {
        if let Some(ind) = self.cur_input {
            if ind > 0 {
                self.cur_input = Some(ind - 1);
                self.update_input_focus();
            }
        }
    }

    fn key_down(&mut self) {
        if let Some(ind) = self.cur_input {
            if ind < self.inputs.len() - 1 {
                self.cur_input = Some(ind + 1);
                self.update_input_focus();
            }
        }
    }

    fn key_left(&mut self) {
        self.inputs[self.cur_input.unwrap()].key_left();
    }

    fn key_right(&mut self) {
        self.inputs[self.cur_input.unwrap()].key_right();
    }

    fn key_backspace(&mut self) {
        self.inputs[self.cur_input.unwrap()].key_backspace();
    }
}


impl InputDialog {
    pub fn new(input_names: Vec<&str>) -> InputDialog {
        InputDialog {
            inputs: input_names.into_iter().map(Input::new).collect(),
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
        (&self.inputs).iter().map(|input| input.get_widget()).collect()        
    }

    pub fn get_inputs_size(&self) -> usize {
        self.inputs.len()
    }

    pub fn get_cur_input_ind(&self) -> Option<usize> {
        self.cur_input
    }

    pub fn get_inputs_as_strings(&self) -> Vec<String> {
        (&self.inputs).iter().map(|input| input.get_input().to_string()).collect()
    }

    pub fn get_inputs_names(&self) -> Vec<String> {
        (&self.inputs).iter().map(|input| input.get_name().to_string()).collect()
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
