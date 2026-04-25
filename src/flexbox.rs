use floem::style::{AlignContent, AlignItems, JustifyContent, Style};

pub trait TailwindFlexboxExt {
    fn flex_row(self) -> Self;
    fn flex_col(self) -> Self;
    fn flex_col_reverse(self) -> Self;
    fn flex_wrap(self) -> Self;
    fn flex_nowrap(self) -> Self;
    fn flex_wrap_reverse(self) -> Self;
    fn grow(self) -> Self;
    fn grow_0(self) -> Self;
    fn shrink(self) -> Self;
    fn shrink_0(self) -> Self;
    fn basis_0(self) -> Self;
    fn basis_auto(self) -> Self;
    fn basis_px(self) -> Self;
    fn basis_0_5(self) -> Self;
    fn basis_1(self) -> Self;
    fn basis_1_5(self) -> Self;
    fn basis_2(self) -> Self;
    fn basis_2_5(self) -> Self;
    fn basis_3(self) -> Self;
    fn basis_3_5(self) -> Self;
    fn basis_4(self) -> Self;
    fn basis_5(self) -> Self;
    fn basis_6(self) -> Self;
    fn basis_7(self) -> Self;
    fn basis_8(self) -> Self;
    fn basis_9(self) -> Self;
    fn basis_10(self) -> Self;
    fn basis_11(self) -> Self;
    fn basis_12(self) -> Self;
    fn basis_14(self) -> Self;
    fn basis_16(self) -> Self;
    fn basis_20(self) -> Self;
    fn basis_24(self) -> Self;
    fn basis_28(self) -> Self;
    fn basis_32(self) -> Self;
    fn basis_36(self) -> Self;
    fn basis_40(self) -> Self;
    fn basis_44(self) -> Self;
    fn basis_48(self) -> Self;
    fn basis_52(self) -> Self;
    fn basis_56(self) -> Self;
    fn basis_60(self) -> Self;
    fn basis_64(self) -> Self;
    fn basis_72(self) -> Self;
    fn basis_80(self) -> Self;
    fn basis_96(self) -> Self;
    fn justify_start(self) -> Self;
    fn justify_end(self) -> Self;
    fn justify_center(self) -> Self;
    fn justify_between(self) -> Self;
    fn justify_around(self) -> Self;
    fn justify_evenly(self) -> Self;
    fn items_start(self) -> Self;
    fn items_end(self) -> Self;
    fn items_center(self) -> Self;
    fn items_baseline(self) -> Self;
    fn items_stretch(self) -> Self;
    fn self_auto(self) -> Self;
    fn self_start(self) -> Self;
    fn self_end(self) -> Self;
    fn self_center(self) -> Self;
    fn self_stretch(self) -> Self;
    fn content_normal(self) -> Self;
    fn content_start(self) -> Self;
    fn content_end(self) -> Self;
    fn content_center(self) -> Self;
    fn content_between(self) -> Self;
    fn content_around(self) -> Self;
    fn content_evenly(self) -> Self;
    fn content_stretch(self) -> Self;
}

impl TailwindFlexboxExt for Style {
    fn flex_row(self) -> Self { self.flex_row() }
    fn flex_col(self) -> Self { self.flex_col() }
    fn flex_col_reverse(self) -> Self { self.flex_direction(floem::style::FlexDirection::ColumnReverse) }

    fn flex_wrap(self) -> Self { self.flex_wrap(floem::style::FlexWrap::Wrap) }
    fn flex_nowrap(self) -> Self { self.flex_wrap(floem::style::FlexWrap::NoWrap) }
    fn flex_wrap_reverse(self) -> Self { self.flex_wrap(floem::style::FlexWrap::WrapReverse) }

    fn grow(self) -> Self { self.flex_grow(1.0) }
    fn grow_0(self) -> Self { self.flex_grow(0.0) }
    fn shrink(self) -> Self { self.flex_shrink(1.0) }
    fn shrink_0(self) -> Self { self.flex_shrink(0.0) }

    fn basis_0(self) -> Self   { self.flex_basis(0.0) }
    fn basis_auto(self) -> Self { self.flex_basis(f32::NAN) }
    fn basis_px(self) -> Self  { self.flex_basis(1.0) }
    fn basis_0_5(self) -> Self { self.flex_basis(2.0) }
    fn basis_1(self) -> Self   { self.flex_basis(4.0) }
    fn basis_1_5(self) -> Self { self.flex_basis(6.0) }
    fn basis_2(self) -> Self   { self.flex_basis(8.0) }
    fn basis_2_5(self) -> Self { self.flex_basis(10.0) }
    fn basis_3(self) -> Self   { self.flex_basis(12.0) }
    fn basis_3_5(self) -> Self { self.flex_basis(14.0) }
    fn basis_4(self) -> Self   { self.flex_basis(16.0) }
    fn basis_5(self) -> Self   { self.flex_basis(20.0) }
    fn basis_6(self) -> Self   { self.flex_basis(24.0) }
    fn basis_7(self) -> Self   { self.flex_basis(28.0) }
    fn basis_8(self) -> Self   { self.flex_basis(32.0) }
    fn basis_9(self) -> Self   { self.flex_basis(36.0) }
    fn basis_10(self) -> Self  { self.flex_basis(40.0) }
    fn basis_11(self) -> Self  { self.flex_basis(44.0) }
    fn basis_12(self) -> Self  { self.flex_basis(48.0) }
    fn basis_14(self) -> Self  { self.flex_basis(56.0) }
    fn basis_16(self) -> Self  { self.flex_basis(64.0) }
    fn basis_20(self) -> Self  { self.flex_basis(80.0) }
    fn basis_24(self) -> Self  { self.flex_basis(96.0) }
    fn basis_28(self) -> Self  { self.flex_basis(112.0) }
    fn basis_32(self) -> Self  { self.flex_basis(128.0) }
    fn basis_36(self) -> Self  { self.flex_basis(144.0) }
    fn basis_40(self) -> Self  { self.flex_basis(160.0) }
    fn basis_44(self) -> Self  { self.flex_basis(176.0) }
    fn basis_48(self) -> Self  { self.flex_basis(192.0) }
    fn basis_52(self) -> Self  { self.flex_basis(208.0) }
    fn basis_56(self) -> Self  { self.flex_basis(224.0) }
    fn basis_60(self) -> Self  { self.flex_basis(240.0) }
    fn basis_64(self) -> Self  { self.flex_basis(256.0) }
    fn basis_72(self) -> Self  { self.flex_basis(288.0) }
    fn basis_80(self) -> Self  { self.flex_basis(320.0) }
    fn basis_96(self) -> Self  { self.flex_basis(384.0) }

    fn justify_start(self) -> Self { self.justify_content(JustifyContent::FlexStart) }
    fn justify_end(self) -> Self { self.justify_content(JustifyContent::FlexEnd) }
    fn justify_center(self) -> Self { self.justify_content(JustifyContent::Center) }
    fn justify_between(self) -> Self { self.justify_content(JustifyContent::SpaceBetween) }
    fn justify_around(self) -> Self { self.justify_content(JustifyContent::SpaceAround) }
    fn justify_evenly(self) -> Self { self.justify_content(JustifyContent::SpaceEvenly) }

    fn items_start(self) -> Self { self.align_items(AlignItems::FlexStart) }
    fn items_end(self) -> Self { self.align_items(AlignItems::FlexEnd) }
    fn items_center(self) -> Self { self.align_items(AlignItems::Center) }
    fn items_baseline(self) -> Self { self.align_items(AlignItems::Baseline) }
    fn items_stretch(self) -> Self { self.align_items(AlignItems::Stretch) }

    fn self_auto(self) -> Self { self.align_self(AlignItems::FlexStart) }
    fn self_start(self) -> Self { self.align_self(AlignItems::FlexStart) }
    fn self_end(self) -> Self { self.align_self(AlignItems::FlexEnd) }
    fn self_center(self) -> Self { self.align_self(AlignItems::Center) }
    fn self_stretch(self) -> Self { self.align_self(AlignItems::Stretch) }

    fn content_normal(self) -> Self { self.align_content(AlignContent::Stretch) }
    fn content_start(self) -> Self { self.align_content(AlignContent::FlexStart) }
    fn content_end(self) -> Self { self.align_content(AlignContent::FlexEnd) }
    fn content_center(self) -> Self { self.align_content(AlignContent::Center) }
    fn content_between(self) -> Self { self.align_content(AlignContent::SpaceBetween) }
    fn content_around(self) -> Self { self.align_content(AlignContent::SpaceAround) }
    fn content_evenly(self) -> Self { self.align_content(AlignContent::SpaceEvenly) }
    fn content_stretch(self) -> Self { self.align_content(AlignContent::Stretch) }
}
