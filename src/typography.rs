use floem::style::{FontFamily, FontStyle, FontWeight, Style};

pub trait TailwindTypographyExt {
    fn font_sans(self) -> Self;
    fn font_serif(self) -> Self;
    fn font_mono(self) -> Self;
    fn italic(self) -> Self;
    fn not_italic(self) -> Self;
    fn font_weight_thin(self) -> Self;
    fn font_weight_extralight(self) -> Self;
    fn font_weight_light(self) -> Self;
    fn font_weight_normal(self) -> Self;
    fn font_weight_medium(self) -> Self;
    fn font_weight_semibold(self) -> Self;
    fn font_weight_bold(self) -> Self;
    fn font_weight_extrabold(self) -> Self;
    fn font_weight_black(self) -> Self;
    fn truncate(self) -> Self;
    fn text_ellipsis(self) -> Self;
    fn text_clip(self) -> Self;
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

impl TailwindTypographyExt for Style {
    fn font_sans(mut self) -> Self { self.font_family(FontFamily::SansSerif); self }
    fn font_serif(mut self) -> Self { self.font_family(FontFamily::Serif); self }
    fn font_mono(mut self) -> Self { self.font_family(FontFamily::Monospace); self }

    fn italic(mut self) -> Self { self.font_style(FontStyle::Italic); self }
    fn not_italic(mut self) -> Self { self.font_style(FontStyle::Normal); self }

    fn font_weight_thin(mut self) -> Self { self.font_weight(FontWeight::THIN); self }
    fn font_weight_extralight(mut self) -> Self { self.font_weight(FontWeight::EXTRA_LIGHT); self }
    fn font_weight_light(mut self) -> Self { self.font_weight(FontWeight::LIGHT); self }
    fn font_weight_normal(mut self) -> Self { self.font_weight(FontWeight::NORMAL); self }
    fn font_weight_medium(mut self) -> Self { self.font_weight(FontWeight::MEDIUM); self }
    fn font_weight_semibold(mut self) -> Self { self.font_weight(FontWeight::SEMI_BOLD); self }
    fn font_weight_bold(mut self) -> Self { self.font_weight(FontWeight::BOLD); self }
    fn font_weight_extrabold(mut self) -> Self { self.font_weight(FontWeight::EXTRA_BOLD); self }
    fn font_weight_black(mut self) -> Self { self.font_weight(FontWeight::BLACK); self }

    fn truncate(mut self) -> Self { self.text_ellipsis(); self }
    fn text_ellipsis(mut self) -> Self { self.text_ellipsis(); self }
    fn text_clip(mut self) -> Self { self.text_clip(); self }

    fn leading_3(mut self) -> Self { self.line_height(12.0); self }
    fn leading_4(mut self) -> Self { self.line_height(16.0); self }
    fn leading_5(mut self) -> Self { self.line_height(20.0); self }
    fn leading_6(mut self) -> Self { self.line_height(24.0); self }
    fn leading_7(mut self) -> Self { self.line_height(28.0); self }
    fn leading_8(mut self) -> Self { self.line_height(32.0); self }
    fn leading_9(mut self) -> Self { self.line_height(36.0); self }
    fn leading_10(mut self) -> Self { self.line_height(40.0); self }
    fn leading_none(mut self) -> Self { self.line_height(1.0); self }
    fn leading_tight(mut self) -> Self { self.line_height(1.25); self }
    fn leading_snug(mut self) -> Self { self.line_height(1.375); self }
    fn leading_normal(mut self) -> Self { self.line_height(1.5); self }
    fn leading_relaxed(mut self) -> Self { self.line_height(1.625); self }
    fn leading_loose(mut self) -> Self { self.line_height(2.0); self }
}
