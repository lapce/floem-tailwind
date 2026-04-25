use floem::style::Style;
pub trait TailwindNumberFontSizeExt {
    fn text_12(self) -> Self;
    fn text_14(self) -> Self;
    fn text_16(self) -> Self;
    fn text_18(self) -> Self;
    fn text_20(self) -> Self;
    fn text_24(self) -> Self;
    fn text_28(self) -> Self;
    fn text_32(self) -> Self;
    fn text_36(self) -> Self;
    fn text_40(self) -> Self;
    fn text_44(self) -> Self;
    fn text_48(self) -> Self;
    fn text_52(self) -> Self;
    fn text_56(self) -> Self;
    fn text_60(self) -> Self;
    fn text_64(self) -> Self;
    fn text_72(self) -> Self;
    fn text_80(self) -> Self;
    fn text_96(self) -> Self;
}
impl TailwindNumberFontSizeExt for Style {
    fn text_12(self) -> Self { self.font_size(12.0) }
    fn text_14(self) -> Self { self.font_size(14.0) }
    fn text_16(self) -> Self { self.font_size(16.0) }
    fn text_18(self) -> Self { self.font_size(18.0) }
    fn text_20(self) -> Self { self.font_size(20.0) }
    fn text_24(self) -> Self { self.font_size(24.0) }
    fn text_28(self) -> Self { self.font_size(28.0) }
    fn text_32(self) -> Self { self.font_size(32.0) }
    fn text_36(self) -> Self { self.font_size(36.0) }
    fn text_40(self) -> Self { self.font_size(40.0) }
    fn text_44(self) -> Self { self.font_size(44.0) }
    fn text_48(self) -> Self { self.font_size(48.0) }
    fn text_52(self) -> Self { self.font_size(52.0) }
    fn text_56(self) -> Self { self.font_size(56.0) }
    fn text_60(self) -> Self { self.font_size(60.0) }
    fn text_64(self) -> Self { self.font_size(64.0) }
    fn text_72(self) -> Self { self.font_size(72.0) }
    fn text_80(self) -> Self { self.font_size(80.0) }
    fn text_96(self) -> Self { self.font_size(96.0) }
}
