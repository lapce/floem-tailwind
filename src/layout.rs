use floem::style::{Display, Position, Style};

pub trait TailwindLayoutExt {
    fn flex(self) -> Self;
    fn grid(self) -> Self;
    fn hidden(self) -> Self;
    fn relative(self) -> Self;
    fn absolute(self) -> Self;
    fn z_0(self) -> Self;
    fn z_10(self) -> Self;
    fn z_20(self) -> Self;
    fn z_30(self) -> Self;
    fn z_40(self) -> Self;
    fn z_50(self) -> Self;
    fn z_auto(self) -> Self;
    fn inset_x_0(self) -> Self;
    fn inset_y_0(self) -> Self;
    fn truncate(self) -> Self;
    fn text_ellipsis(self) -> Self;
    fn text_clip(self) -> Self;
}

impl TailwindLayoutExt for Style {
    fn flex(self) -> Self { self.display(Display::Flex) }
    fn grid(self) -> Self { self.display(Display::Grid) }
    fn hidden(self) -> Self { self.hide() }
    fn relative(self) -> Self { self.position(Position::Relative) }
    fn absolute(self) -> Self { self.position(Position::Absolute) }
    fn z_0(self) -> Self { self.z_index(0) }
    fn z_10(self) -> Self { self.z_index(10) }
    fn z_20(self) -> Self { self.z_index(20) }
    fn z_30(self) -> Self { self.z_index(30) }
    fn z_40(self) -> Self { self.z_index(40) }
    fn z_50(self) -> Self { self.z_index(50) }
    fn z_auto(self) -> Self { self.z_index(i32::MAX) }
    fn inset_x_0(self) -> Self { self.inset_left(0.0).inset_right(0.0) }
    fn inset_y_0(self) -> Self { self.inset_top(0.0).inset_bottom(0.0) }
    fn truncate(self) -> Self { self.text_ellipsis() }
    fn text_ellipsis(self) -> Self { self.text_ellipsis() }
    fn text_clip(self) -> Self { self.text_clip() }
}
