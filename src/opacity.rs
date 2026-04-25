use floem::style::Style;

pub trait TailwindOpacityExt {
    fn opacity_5(self) -> Self;
    fn opacity_10(self) -> Self;
    fn opacity_15(self) -> Self;
    fn opacity_20(self) -> Self;
    fn opacity_25(self) -> Self;
    fn opacity_30(self) -> Self;
    fn opacity_35(self) -> Self;
    fn opacity_40(self) -> Self;
    fn opacity_45(self) -> Self;
    fn opacity_50(self) -> Self;
    fn opacity_55(self) -> Self;
    fn opacity_60(self) -> Self;
    fn opacity_65(self) -> Self;
    fn opacity_70(self) -> Self;
    fn opacity_75(self) -> Self;
    fn opacity_80(self) -> Self;
    fn opacity_85(self) -> Self;
    fn opacity_90(self) -> Self;
    fn opacity_95(self) -> Self;
    fn opacity_100(self) -> Self;
}
impl TailwindOpacityExt for Style {
    fn opacity_5(self) -> Self { self.opacity(0.05) }
    fn opacity_10(self) -> Self { self.opacity(0.10) }
    fn opacity_15(self) -> Self { self.opacity(0.15) }
    fn opacity_20(self) -> Self { self.opacity(0.20) }
    fn opacity_25(self) -> Self { self.opacity(0.25) }
    fn opacity_30(self) -> Self { self.opacity(0.30) }
    fn opacity_35(self) -> Self { self.opacity(0.35) }
    fn opacity_40(self) -> Self { self.opacity(0.40) }
    fn opacity_45(self) -> Self { self.opacity(0.45) }
    fn opacity_50(self) -> Self { self.opacity(0.50) }
    fn opacity_55(self) -> Self { self.opacity(0.55) }
    fn opacity_60(self) -> Self { self.opacity(0.60) }
    fn opacity_65(self) -> Self { self.opacity(0.65) }
    fn opacity_70(self) -> Self { self.opacity(0.70) }
    fn opacity_75(self) -> Self { self.opacity(0.75) }
    fn opacity_80(self) -> Self { self.opacity(0.80) }
    fn opacity_85(self) -> Self { self.opacity(0.85) }
    fn opacity_90(self) -> Self { self.opacity(0.90) }
    fn opacity_95(self) -> Self { self.opacity(0.95) }
    fn opacity_100(self) -> Self { self.opacity(1.00) }
}
