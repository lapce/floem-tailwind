use floem::style::Style;
use super::colors;

pub trait TailwindSpecialColorExt {
    fn bg_white(self) -> Self;
    fn bg_black(self) -> Self;
    fn bg_transparent(self) -> Self;
    fn text_white(self) -> Self;
    fn text_black(self) -> Self;
    fn text_transparent(self) -> Self;
    fn border_white(self) -> Self;
    fn border_black(self) -> Self;
    fn border_transparent(self) -> Self;
}

impl TailwindSpecialColorExt for Style {
    fn bg_white(self) -> Self { self.background(colors::WHITE) }
    fn bg_black(self) -> Self { self.background(colors::BLACK) }
    fn bg_transparent(self) -> Self { self.background(colors::TRANSPARENT) }
    fn text_white(self) -> Self { self.color(colors::WHITE) }
    fn text_black(self) -> Self { self.color(colors::BLACK) }
    fn text_transparent(self) -> Self { self.color(colors::TRANSPARENT) }
    fn border_white(self) -> Self { self.border_color(colors::WHITE) }
    fn border_black(self) -> Self { self.border_color(colors::BLACK) }
    fn border_transparent(self) -> Self { self.border_color(colors::TRANSPARENT) }
}
