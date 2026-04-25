use floem::style::Style;

pub trait TailwindGapExt {
    fn gap_x_0(self) -> Self; fn gap_x_px(self) -> Self; fn gap_x_0_5(self) -> Self; fn gap_x_1(self) -> Self; fn gap_x_1_5(self) -> Self;
    fn gap_x_2(self) -> Self; fn gap_x_2_5(self) -> Self; fn gap_x_3(self) -> Self; fn gap_x_3_5(self) -> Self; fn gap_x_4(self) -> Self;
    fn gap_x_5(self) -> Self; fn gap_x_6(self) -> Self; fn gap_x_7(self) -> Self; fn gap_x_8(self) -> Self; fn gap_x_9(self) -> Self;
    fn gap_x_10(self) -> Self; fn gap_x_11(self) -> Self; fn gap_x_12(self) -> Self; fn gap_x_14(self) -> Self; fn gap_x_16(self) -> Self;
    fn gap_x_20(self) -> Self; fn gap_x_24(self) -> Self; fn gap_x_28(self) -> Self; fn gap_x_32(self) -> Self; fn gap_x_36(self) -> Self;
    fn gap_x_40(self) -> Self; fn gap_x_44(self) -> Self; fn gap_x_48(self) -> Self; fn gap_x_52(self) -> Self; fn gap_x_56(self) -> Self;
    fn gap_x_60(self) -> Self; fn gap_x_64(self) -> Self; fn gap_x_72(self) -> Self; fn gap_x_80(self) -> Self; fn gap_x_96(self) -> Self;
    fn gap_y_0(self) -> Self; fn gap_y_px(self) -> Self; fn gap_y_0_5(self) -> Self; fn gap_y_1(self) -> Self; fn gap_y_1_5(self) -> Self;
    fn gap_y_2(self) -> Self; fn gap_y_2_5(self) -> Self; fn gap_y_3(self) -> Self; fn gap_y_3_5(self) -> Self; fn gap_y_4(self) -> Self;
    fn gap_y_5(self) -> Self; fn gap_y_6(self) -> Self; fn gap_y_7(self) -> Self; fn gap_y_8(self) -> Self; fn gap_y_9(self) -> Self;
    fn gap_y_10(self) -> Self; fn gap_y_11(self) -> Self; fn gap_y_12(self) -> Self; fn gap_y_14(self) -> Self; fn gap_y_16(self) -> Self;
    fn gap_y_20(self) -> Self; fn gap_y_24(self) -> Self; fn gap_y_28(self) -> Self; fn gap_y_32(self) -> Self; fn gap_y_36(self) -> Self;
    fn gap_y_40(self) -> Self; fn gap_y_44(self) -> Self; fn gap_y_48(self) -> Self; fn gap_y_52(self) -> Self; fn gap_y_56(self) -> Self;
    fn gap_y_60(self) -> Self; fn gap_y_64(self) -> Self; fn gap_y_72(self) -> Self; fn gap_y_80(self) -> Self; fn gap_y_96(self) -> Self;
}

impl TailwindGapExt for Style {
    fn gap_x_0(self) -> Self { self.col_gap(0.0) } fn gap_x_px(self) -> Self { self.col_gap(1.0) }
    fn gap_x_0_5(self) -> Self { self.col_gap(2.0) } fn gap_x_1(self) -> Self { self.col_gap(4.0) }
    fn gap_x_1_5(self) -> Self { self.col_gap(6.0) } fn gap_x_2(self) -> Self { self.col_gap(8.0) }
    fn gap_x_2_5(self) -> Self { self.col_gap(10.0) } fn gap_x_3(self) -> Self { self.col_gap(12.0) }
    fn gap_x_3_5(self) -> Self { self.col_gap(14.0) } fn gap_x_4(self) -> Self { self.col_gap(16.0) }
    fn gap_x_5(self) -> Self { self.col_gap(20.0) } fn gap_x_6(self) -> Self { self.col_gap(24.0) }
    fn gap_x_7(self) -> Self { self.col_gap(28.0) } fn gap_x_8(self) -> Self { self.col_gap(32.0) }
    fn gap_x_9(self) -> Self { self.col_gap(36.0) } fn gap_x_10(self) -> Self { self.col_gap(40.0) }
    fn gap_x_11(self) -> Self { self.col_gap(44.0) } fn gap_x_12(self) -> Self { self.col_gap(48.0) }
    fn gap_x_14(self) -> Self { self.col_gap(56.0) } fn gap_x_16(self) -> Self { self.col_gap(64.0) }
    fn gap_x_20(self) -> Self { self.col_gap(80.0) } fn gap_x_24(self) -> Self { self.col_gap(96.0) }
    fn gap_x_28(self) -> Self { self.col_gap(112.0) } fn gap_x_32(self) -> Self { self.col_gap(128.0) }
    fn gap_x_36(self) -> Self { self.col_gap(144.0) } fn gap_x_40(self) -> Self { self.col_gap(160.0) }
    fn gap_x_44(self) -> Self { self.col_gap(176.0) } fn gap_x_48(self) -> Self { self.col_gap(192.0) }
    fn gap_x_52(self) -> Self { self.col_gap(208.0) } fn gap_x_56(self) -> Self { self.col_gap(224.0) }
    fn gap_x_60(self) -> Self { self.col_gap(240.0) } fn gap_x_64(self) -> Self { self.col_gap(256.0) }
    fn gap_x_72(self) -> Self { self.col_gap(288.0) } fn gap_x_80(self) -> Self { self.col_gap(320.0) }
    fn gap_x_96(self) -> Self { self.col_gap(384.0) }

    fn gap_y_0(self) -> Self { self.row_gap(0.0) } fn gap_y_px(self) -> Self { self.row_gap(1.0) }
    fn gap_y_0_5(self) -> Self { self.row_gap(2.0) } fn gap_y_1(self) -> Self { self.row_gap(4.0) }
    fn gap_y_1_5(self) -> Self { self.row_gap(6.0) } fn gap_y_2(self) -> Self { self.row_gap(8.0) }
    fn gap_y_2_5(self) -> Self { self.row_gap(10.0) } fn gap_y_3(self) -> Self { self.row_gap(12.0) }
    fn gap_y_3_5(self) -> Self { self.row_gap(14.0) } fn gap_y_4(self) -> Self { self.row_gap(16.0) }
    fn gap_y_5(self) -> Self { self.row_gap(20.0) } fn gap_y_6(self) -> Self { self.row_gap(24.0) }
    fn gap_y_7(self) -> Self { self.row_gap(28.0) } fn gap_y_8(self) -> Self { self.row_gap(32.0) }
    fn gap_y_9(self) -> Self { self.row_gap(36.0) } fn gap_y_10(self) -> Self { self.row_gap(40.0) }
    fn gap_y_11(self) -> Self { self.row_gap(44.0) } fn gap_y_12(self) -> Self { self.row_gap(48.0) }
    fn gap_y_14(self) -> Self { self.row_gap(56.0) } fn gap_y_16(self) -> Self { self.row_gap(64.0) }
    fn gap_y_20(self) -> Self { self.row_gap(80.0) } fn gap_y_24(self) -> Self { self.row_gap(96.0) }
    fn gap_y_28(self) -> Self { self.row_gap(112.0) } fn gap_y_32(self) -> Self { self.row_gap(128.0) }
    fn gap_y_36(self) -> Self { self.row_gap(144.0) } fn gap_y_40(self) -> Self { self.row_gap(160.0) }
    fn gap_y_44(self) -> Self { self.row_gap(176.0) } fn gap_y_48(self) -> Self { self.row_gap(192.0) }
    fn gap_y_52(self) -> Self { self.row_gap(208.0) } fn gap_y_56(self) -> Self { self.row_gap(224.0) }
    fn gap_y_60(self) -> Self { self.row_gap(240.0) } fn gap_y_64(self) -> Self { self.row_gap(256.0) }
    fn gap_y_72(self) -> Self { self.row_gap(288.0) } fn gap_y_80(self) -> Self { self.row_gap(320.0) }
    fn gap_y_96(self) -> Self { self.row_gap(384.0) }
}
