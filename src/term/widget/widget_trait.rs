pub trait WidgetTrait {
    fn on_focus(&mut self) {}

    fn on_blur(&mut self) {}

    fn key_char(&mut self, _character: char) {}

    fn key_up(&mut self) {}

    fn key_down(&mut self) {}

    fn key_left(&mut self) {}

    fn key_right(&mut self) {}

    fn key_backspace(&mut self) {}
}