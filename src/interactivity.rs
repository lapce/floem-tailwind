use floem::style::{CursorStyle, Style};

pub trait TailwindInteractivityExt {
    fn cursor_default(self) -> Self;
    fn cursor_wait(self) -> Self;
    fn cursor_text(self) -> Self;
    fn cursor_move(self) -> Self;
    fn cursor_grab(self) -> Self;
    fn select_none(self) -> Self;
    fn select_text(self) -> Self;
    fn select_all(self) -> Self;
    fn select_auto(self) -> Self;
    fn pointer_events_none(self) -> Self;
    fn pointer_events_auto(self) -> Self;
}

impl TailwindInteractivityExt for Style {
    fn cursor_default(self) -> Self { self.cursor(CursorStyle::Default) }
    fn cursor_wait(self) -> Self { self.cursor(CursorStyle::Wait) }
    fn cursor_text(self) -> Self { self.cursor(CursorStyle::Text) }
    fn cursor_move(self) -> Self { self.cursor(CursorStyle::Move) }
    fn cursor_grab(self) -> Self { self.cursor(CursorStyle::Grab) }
    fn select_none(self) -> Self { self.selectable(false) }
    fn select_text(self) -> Self { self.selectable(true) }
    fn select_all(self) -> Self { self.selectable(true) }
    fn select_auto(self) -> Self { self.selectable(true) }
    fn pointer_events_none(self) -> Self { self }
    fn pointer_events_auto(self) -> Self { self }
}
