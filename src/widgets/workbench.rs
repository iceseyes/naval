use crate::state::NavalBattleState;
use ratatui::{
    prelude::{Buffer, Line, Rect, Stylize, Widget},
    symbols::border,
    widgets::Block,
};

/// The main window of the application.
///
/// The main window takes a *content* which is a boxed widget that will be rendered within the workbench.
pub struct Workbench<'state>(pub &'state NavalBattleState);

impl<'state> Widget for &Workbench<'state> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Naval Battle ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        block.clone().render(area, buf);
        self.0.render(block.inner(area), buf);
    }
}
