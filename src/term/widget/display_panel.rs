use tui::{
    text::Text,
    widgets::Paragraph
};

pub struct DisplayPanel {
    default: Text<'static>  // Static lifetime makes sense here right?
}

impl DisplayPanel {
    pub fn new<T>(default: T) -> DisplayPanel 
    where
        T: Into<Text<'static>>,
    {
        DisplayPanel {
            default: default.into()
        }
    }

    pub fn get_widget<'a, T>(&self, text: Option<T>) -> Paragraph<'a> 
    where
        T: Into<Text<'a>>,
    {
        Paragraph::new(
            match text {
                Some(text) => {
                    text.into()
                },
                None => self.default.clone()
            }
        )
    }
}