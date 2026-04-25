use floem::style::Style;
pub trait TailwindBorderSideExt {
    fn border_t_0(self) -> Self;
    fn border_t_1(self) -> Self;
    fn border_t_2(self) -> Self;
    fn border_t_4(self) -> Self;
    fn border_t_8(self) -> Self;
    fn border_r_0(self) -> Self;
    fn border_r_1(self) -> Self;
    fn border_r_2(self) -> Self;
    fn border_r_4(self) -> Self;
    fn border_r_8(self) -> Self;
    fn border_b_0(self) -> Self;
    fn border_b_1(self) -> Self;
    fn border_b_2(self) -> Self;
    fn border_b_4(self) -> Self;
    fn border_b_8(self) -> Self;
    fn border_l_0(self) -> Self;
    fn border_l_1(self) -> Self;
    fn border_l_2(self) -> Self;
    fn border_l_4(self) -> Self;
    fn border_l_8(self) -> Self;
    fn border_x_0(self) -> Self;
    fn border_x_1(self) -> Self;
    fn border_x_2(self) -> Self;
    fn border_x_4(self) -> Self;
    fn border_x_8(self) -> Self;
    fn border_y_0(self) -> Self;
    fn border_y_1(self) -> Self;
    fn border_y_2(self) -> Self;
    fn border_y_4(self) -> Self;
    fn border_y_8(self) -> Self;
}
impl TailwindBorderSideExt for Style {
    fn border_t_0(self) -> Self { self.border_top(0.0) }
    fn border_r_0(self) -> Self { self.border_right(0.0) }
    fn border_b_0(self) -> Self { self.border_bottom(0.0) }
    fn border_l_0(self) -> Self { self.border_left(0.0) }
    fn border_x_0(self) -> Self { self.border_left(0.0).border_right(0.0) }
    fn border_y_0(self) -> Self { self.border_top(0.0).border_bottom(0.0) }
    fn border_t_1(self) -> Self { self.border_top(1.0) }
    fn border_r_1(self) -> Self { self.border_right(1.0) }
    fn border_b_1(self) -> Self { self.border_bottom(1.0) }
    fn border_l_1(self) -> Self { self.border_left(1.0) }
    fn border_x_1(self) -> Self { self.border_left(1.0).border_right(1.0) }
    fn border_y_1(self) -> Self { self.border_top(1.0).border_bottom(1.0) }
    fn border_t_2(self) -> Self { self.border_top(2.0) }
    fn border_r_2(self) -> Self { self.border_right(2.0) }
    fn border_b_2(self) -> Self { self.border_bottom(2.0) }
    fn border_l_2(self) -> Self { self.border_left(2.0) }
    fn border_x_2(self) -> Self { self.border_left(2.0).border_right(2.0) }
    fn border_y_2(self) -> Self { self.border_top(2.0).border_bottom(2.0) }
    fn border_t_4(self) -> Self { self.border_top(4.0) }
    fn border_r_4(self) -> Self { self.border_right(4.0) }
    fn border_b_4(self) -> Self { self.border_bottom(4.0) }
    fn border_l_4(self) -> Self { self.border_left(4.0) }
    fn border_x_4(self) -> Self { self.border_left(4.0).border_right(4.0) }
    fn border_y_4(self) -> Self { self.border_top(4.0).border_bottom(4.0) }
    fn border_t_8(self) -> Self { self.border_top(8.0) }
    fn border_r_8(self) -> Self { self.border_right(8.0) }
    fn border_b_8(self) -> Self { self.border_bottom(8.0) }
    fn border_l_8(self) -> Self { self.border_left(8.0) }
    fn border_x_8(self) -> Self { self.border_left(8.0).border_right(8.0) }
    fn border_y_8(self) -> Self { self.border_top(8.0).border_bottom(8.0) }
}
