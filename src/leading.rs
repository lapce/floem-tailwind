use floem::style::Style;

pub trait TailwindLeadingExt {
    fn leading_3(self) -> Self;
    fn leading_4(self) -> Self;
    fn leading_5(self) -> Self;
    fn leading_6(self) -> Self;
    fn leading_7(self) -> Self;
    fn leading_8(self) -> Self;
    fn leading_9(self) -> Self;
    fn leading_10(self) -> Self;
    fn leading_none(self) -> Self;
    fn leading_tight(self) -> Self;
    fn leading_snug(self) -> Self;
    fn leading_normal(self) -> Self;
    fn leading_relaxed(self) -> Self;
    fn leading_loose(self) -> Self;
}

impl TailwindLeadingExt for Style {
    fn leading_3(self) -> Self { self.line_height(12.0) }
    fn leading_4(self) -> Self { self.line_height(16.0) }
    fn leading_5(self) -> Self { self.line_height(20.0) }
    fn leading_6(self) -> Self { self.line_height(24.0) }
    fn leading_7(self) -> Self { self.line_height(28.0) }
    fn leading_8(self) -> Self { self.line_height(32.0) }
    fn leading_9(self) -> Self { self.line_height(36.0) }
    fn leading_10(self) -> Self { self.line_height(40.0) }
    fn leading_none(self) -> Self { self.line_height(1.0) }
    fn leading_tight(self) -> Self { self.line_height(1.25) }
    fn leading_snug(self) -> Self { self.line_height(1.375) }
    fn leading_normal(self) -> Self { self.line_height(1.5) }
    fn leading_relaxed(self) -> Self { self.line_height(1.625) }
    fn leading_loose(self) -> Self { self.line_height(2.0) }
}
