//! Macro-driven integration tests for all Tailwind utility traits.
//! Each method is called with fully-qualified syntax to avoid ambiguity.

use floem::style::Style;
use floem_tailwind::{
    TailwindBorderSideExt, TailwindColorExt, TailwindFlexboxExt,
    TailwindGapExt, TailwindInsetExt, TailwindInteractivityExt,
    TailwindLayoutExt, TailwindLeadingExt, TailwindMinMaxHeightExt,
    TailwindNumberFontSizeExt, TailwindOpacityExt,
    TailwindExt,
};

macro_rules! test_trait_methods {
    ($trait:ty, $test_name:ident, $($method:ident),* $(,)?) => {
        #[test]
        fn $test_name() {
            $(
                let _ = <Style as $trait>::$method(Style::new());
            )*
        }
    };
}

test_trait_methods!(TailwindFlexboxExt, test_flexbox,
    flex_row, flex_col, flex_col_reverse, flex_wrap, flex_nowrap, flex_wrap_reverse,
    grow, grow_0, shrink, shrink_0,
    basis_0, basis_auto, basis_px, basis_0_5, basis_1, basis_1_5,
    basis_2, basis_2_5, basis_3, basis_3_5, basis_4, basis_5,
    basis_6, basis_7, basis_8, basis_9, basis_10,
    basis_11, basis_12, basis_14, basis_16, basis_20, basis_24,
    basis_28, basis_32, basis_36, basis_40, basis_44, basis_48,
    basis_52, basis_56, basis_60, basis_64, basis_72, basis_80, basis_96,
    justify_start, justify_end, justify_center, justify_between, justify_around, justify_evenly,
    items_start, items_end, items_center, items_baseline, items_stretch,
    self_auto, self_start, self_end, self_center, self_stretch,
    content_normal, content_start, content_end, content_center,
    content_between, content_around, content_evenly, content_stretch
);

test_trait_methods!(TailwindLayoutExt, test_layout,
    flex, grid, hidden, relative, absolute,
    z_0, z_10, z_20, z_30, z_40, z_50, z_auto,
    inset_x_0, inset_y_0, truncate, text_ellipsis, text_clip
);

test_trait_methods!(TailwindInteractivityExt, test_interactivity,
    cursor_default, cursor_wait, cursor_text, cursor_move, cursor_grab,
    select_none, select_text, select_all, select_auto,
    pointer_events_none, pointer_events_auto
);

test_trait_methods!(TailwindGapExt, test_gap,
    gap_x_0, gap_x_px, gap_x_0_5, gap_x_1, gap_x_1_5,
    gap_x_2, gap_x_2_5, gap_x_3, gap_x_3_5, gap_x_4,
    gap_x_5, gap_x_6, gap_x_7, gap_x_8, gap_x_9,
    gap_x_10, gap_x_11, gap_x_12, gap_x_14, gap_x_16,
    gap_x_20, gap_x_24, gap_x_28, gap_x_32, gap_x_36,
    gap_x_40, gap_x_44, gap_x_48, gap_x_52, gap_x_56,
    gap_x_60, gap_x_64, gap_x_72, gap_x_80, gap_x_96,
    gap_y_0, gap_y_px, gap_y_0_5, gap_y_1, gap_y_1_5,
    gap_y_2, gap_y_2_5, gap_y_3, gap_y_3_5, gap_y_4,
    gap_y_5, gap_y_6, gap_y_7, gap_y_8, gap_y_9,
    gap_y_10, gap_y_11, gap_y_12, gap_y_14, gap_y_16,
    gap_y_20, gap_y_24, gap_y_28, gap_y_32, gap_y_36,
    gap_y_40, gap_y_44, gap_y_48, gap_y_52, gap_y_56,
    gap_y_60, gap_y_64, gap_y_72, gap_y_80, gap_y_96
);

test_trait_methods!(TailwindBorderSideExt, test_border_side,
    border_t_0, border_t_1, border_t_2, border_t_4, border_t_8,
    border_r_0, border_r_1, border_r_2, border_r_4, border_r_8,
    border_b_0, border_b_1, border_b_2, border_b_4, border_b_8,
    border_l_0, border_l_1, border_l_2, border_l_4, border_l_8,
    border_x_0, border_x_1, border_x_2, border_x_4, border_x_8,
    border_y_0, border_y_1, border_y_2, border_y_4, border_y_8
);

test_trait_methods!(TailwindOpacityExt, test_opacity,
    opacity_5, opacity_10, opacity_15, opacity_20,
    opacity_25, opacity_30, opacity_35, opacity_40, opacity_45,
    opacity_50, opacity_55, opacity_60, opacity_65, opacity_70,
    opacity_75, opacity_80, opacity_85, opacity_90, opacity_95, opacity_100
);

test_trait_methods!(TailwindLeadingExt, test_leading,
    leading_3, leading_4, leading_5, leading_6, leading_7,
    leading_8, leading_9, leading_10,
    leading_tight, leading_snug, leading_normal, leading_relaxed, leading_loose
);

test_trait_methods!(TailwindInsetExt, test_inset,
    top_px, top_0_5, top_1, top_1_5, top_2, top_2_5, top_3, top_3_5,
    top_4, top_5, top_6, top_7, top_8, top_9, top_10, top_11, top_12,
    top_14, top_16, top_20, top_24, top_28, top_32, top_36, top_40,
    top_44, top_48, top_52, top_56, top_60, top_64, top_72, top_80, top_96,
    right_px, right_0_5, right_1, right_1_5, right_2, right_2_5, right_3, right_3_5,
    right_4, right_5, right_6, right_7, right_8, right_9, right_10, right_11, right_12,
    right_14, right_16, right_20, right_24, right_28, right_32, right_36, right_40,
    right_44, right_48, right_52, right_56, right_60, right_64, right_72, right_80, right_96,
    bottom_px, bottom_0_5, bottom_1, bottom_1_5, bottom_2, bottom_2_5, bottom_3, bottom_3_5,
    bottom_4, bottom_5, bottom_6, bottom_7, bottom_8, bottom_9, bottom_10, bottom_11, bottom_12,
    bottom_14, bottom_16, bottom_20, bottom_24, bottom_28, bottom_32, bottom_36, bottom_40,
    bottom_44, bottom_48, bottom_52, bottom_56, bottom_60, bottom_64, bottom_72, bottom_80, bottom_96,
    left_px, left_0_5, left_1, left_1_5, left_2, left_2_5, left_3, left_3_5,
    left_4, left_5, left_6, left_7, left_8, left_9, left_10, left_11, left_12,
    left_14, left_16, left_20, left_24, left_28, left_32, left_36, left_40,
    left_44, left_48, left_52, left_56, left_60, left_64, left_72, left_80, left_96,
    inset_px, inset_0_5, inset_1, inset_1_5, inset_2, inset_2_5, inset_3, inset_3_5,
    inset_4, inset_5, inset_6, inset_7, inset_8, inset_9, inset_10, inset_11, inset_12,
    inset_14, inset_16, inset_20, inset_24, inset_28, inset_32, inset_36, inset_40,
    inset_44, inset_48, inset_52, inset_56, inset_60, inset_64, inset_72, inset_80, inset_96,
    inset_x_px, inset_x_0_5, inset_x_1, inset_x_1_5, inset_x_2, inset_x_2_5, inset_x_3, inset_x_3_5,
    inset_x_4, inset_x_5, inset_x_6, inset_x_7, inset_x_8, inset_x_9, inset_x_10, inset_x_11, inset_x_12,
    inset_x_14, inset_x_16, inset_x_20, inset_x_24, inset_x_28, inset_x_32, inset_x_36, inset_x_40,
    inset_x_44, inset_x_48, inset_x_52, inset_x_56, inset_x_60, inset_x_64, inset_x_72, inset_x_80, inset_x_96,
    inset_y_px, inset_y_0_5, inset_y_1, inset_y_1_5, inset_y_2, inset_y_2_5, inset_y_3, inset_y_3_5,
    inset_y_4, inset_y_5, inset_y_6, inset_y_7, inset_y_8, inset_y_9, inset_y_10, inset_y_11, inset_y_12,
    inset_y_14, inset_y_16, inset_y_20, inset_y_24, inset_y_28, inset_y_32, inset_y_36, inset_y_40,
    inset_y_44, inset_y_48, inset_y_52, inset_y_56, inset_y_60, inset_y_64, inset_y_72, inset_y_80, inset_y_96
);

test_trait_methods!(TailwindMinMaxHeightExt, test_min_max_height,
    min_h_0, min_h_px, min_h_0_5, min_h_1, min_h_1_5, min_h_2, min_h_2_5, min_h_3, min_h_3_5,
    min_h_4, min_h_5, min_h_6, min_h_7, min_h_8, min_h_9, min_h_10, min_h_11, min_h_12,
    min_h_14, min_h_16, min_h_20, min_h_24, min_h_28, min_h_32, min_h_36, min_h_40,
    min_h_44, min_h_48, min_h_52, min_h_56, min_h_60, min_h_64, min_h_72, min_h_80, min_h_96,
    max_h_0, max_h_px, max_h_0_5, max_h_1, max_h_1_5, max_h_2, max_h_2_5, max_h_3, max_h_3_5,
    max_h_4, max_h_5, max_h_6, max_h_7, max_h_8, max_h_9, max_h_10, max_h_11, max_h_12,
    max_h_14, max_h_16, max_h_20, max_h_24, max_h_28, max_h_32, max_h_36, max_h_40,
    max_h_44, max_h_48, max_h_52, max_h_56, max_h_60, max_h_64, max_h_72, max_h_80, max_h_96
);

test_trait_methods!(TailwindNumberFontSizeExt, test_number_font_size,
    text_12, text_14, text_16, text_18, text_20, text_24, text_28, text_32,
    text_36, text_40, text_44, text_48, text_52, text_56, text_60, text_64,
    text_72, text_80, text_96
);

test_trait_methods!(TailwindColorExt, test_colors_ext,
    bg_neutral_50, bg_neutral_100, bg_neutral_200, bg_neutral_300, bg_neutral_400, bg_neutral_500, bg_neutral_600, bg_neutral_700, bg_neutral_800, bg_neutral_900, bg_neutral_950,
    bg_stone_50, bg_stone_100, bg_stone_200, bg_stone_300, bg_stone_400, bg_stone_500, bg_stone_600, bg_stone_700, bg_stone_800, bg_stone_900, bg_stone_950,
    bg_amber_50, bg_amber_100, bg_amber_200, bg_amber_300, bg_amber_400, bg_amber_500, bg_amber_600, bg_amber_700, bg_amber_800, bg_amber_900, bg_amber_950,
    bg_lime_50, bg_lime_100, bg_lime_200, bg_lime_300, bg_lime_400, bg_lime_500, bg_lime_600, bg_lime_700, bg_lime_800, bg_lime_900, bg_lime_950,
    bg_green_50, bg_green_100, bg_green_200, bg_green_300, bg_green_400, bg_green_500, bg_green_600, bg_green_700, bg_green_800, bg_green_900, bg_green_950,
    bg_emerald_50, bg_emerald_100, bg_emerald_200, bg_emerald_300, bg_emerald_400, bg_emerald_500, bg_emerald_600, bg_emerald_700, bg_emerald_800, bg_emerald_900, bg_emerald_950,
    bg_teal_50, bg_teal_100, bg_teal_200, bg_teal_300, bg_teal_400, bg_teal_500, bg_teal_600, bg_teal_700, bg_teal_800, bg_teal_900, bg_teal_950,
    bg_cyan_50, bg_cyan_100, bg_cyan_200, bg_cyan_300, bg_cyan_400, bg_cyan_500, bg_cyan_600, bg_cyan_700, bg_cyan_800, bg_cyan_900, bg_cyan_950,
    bg_sky_50, bg_sky_100, bg_sky_200, bg_sky_300, bg_sky_400, bg_sky_500, bg_sky_600, bg_sky_700, bg_sky_800, bg_sky_900, bg_sky_950,
    bg_blue_50, bg_blue_100, bg_blue_200, bg_blue_300, bg_blue_400, bg_blue_500, bg_blue_600, bg_blue_700, bg_blue_800, bg_blue_900, bg_blue_950,
    bg_indigo_50, bg_indigo_100, bg_indigo_200, bg_indigo_300, bg_indigo_400, bg_indigo_500, bg_indigo_600, bg_indigo_700, bg_indigo_800, bg_indigo_900, bg_indigo_950,
    bg_violet_50, bg_violet_100, bg_violet_200, bg_violet_300, bg_violet_400, bg_violet_500, bg_violet_600, bg_violet_700, bg_violet_800, bg_violet_900, bg_violet_950,
    bg_purple_50, bg_purple_100, bg_purple_200, bg_purple_300, bg_purple_400, bg_purple_500, bg_purple_600, bg_purple_700, bg_purple_800, bg_purple_900, bg_purple_950,
    bg_fuchsia_50, bg_fuchsia_100, bg_fuchsia_200, bg_fuchsia_300, bg_fuchsia_400, bg_fuchsia_500, bg_fuchsia_600, bg_fuchsia_700, bg_fuchsia_800, bg_fuchsia_900, bg_fuchsia_950,
    bg_pink_50, bg_pink_100, bg_pink_200, bg_pink_300, bg_pink_400, bg_pink_500, bg_pink_600, bg_pink_700, bg_pink_800, bg_pink_900, bg_pink_950,
    bg_rose_50, bg_rose_100, bg_rose_200, bg_rose_300, bg_rose_400, bg_rose_500, bg_rose_600, bg_rose_700, bg_rose_800, bg_rose_900, bg_rose_950,
    text_neutral_50, text_neutral_100, text_neutral_200, text_neutral_300, text_neutral_400, text_neutral_500, text_neutral_600, text_neutral_700, text_neutral_800, text_neutral_900, text_neutral_950,
    text_stone_50, text_stone_100, text_stone_200, text_stone_300, text_stone_400, text_stone_500, text_stone_600, text_stone_700, text_stone_800, text_stone_900, text_stone_950,
    text_amber_50, text_amber_100, text_amber_200, text_amber_300, text_amber_400, text_amber_500, text_amber_600, text_amber_700, text_amber_800, text_amber_900, text_amber_950,
    text_lime_50, text_lime_100, text_lime_200, text_lime_300, text_lime_400, text_lime_500, text_lime_600, text_lime_700, text_lime_800, text_lime_900, text_lime_950,
    text_green_50, text_green_100, text_green_200, text_green_300, text_green_400, text_green_500, text_green_600, text_green_700, text_green_800, text_green_900, text_green_950,
    text_emerald_50, text_emerald_100, text_emerald_200, text_emerald_300, text_emerald_400, text_emerald_500, text_emerald_600, text_emerald_700, text_emerald_800, text_emerald_900, text_emerald_950,
    text_teal_50, text_teal_100, text_teal_200, text_teal_300, text_teal_400, text_teal_500, text_teal_600, text_teal_700, text_teal_800, text_teal_900, text_teal_950,
    text_cyan_50, text_cyan_100, text_cyan_200, text_cyan_300, text_cyan_400, text_cyan_500, text_cyan_600, text_cyan_700, text_cyan_800, text_cyan_900, text_cyan_950,
    text_sky_50, text_sky_100, text_sky_200, text_sky_300, text_sky_400, text_sky_500, text_sky_600, text_sky_700, text_sky_800, text_sky_900, text_sky_950,
    text_blue_50, text_blue_100, text_blue_200, text_blue_300, text_blue_400, text_blue_500, text_blue_600, text_blue_700, text_blue_800, text_blue_900, text_blue_950,
    text_indigo_50, text_indigo_100, text_indigo_200, text_indigo_300, text_indigo_400, text_indigo_500, text_indigo_600, text_indigo_700, text_indigo_800, text_indigo_900, text_indigo_950,
    text_violet_50, text_violet_100, text_violet_200, text_violet_300, text_violet_400, text_violet_500, text_violet_600, text_violet_700, text_violet_800, text_violet_900, text_violet_950,
    text_purple_50, text_purple_100, text_purple_200, text_purple_300, text_purple_400, text_purple_500, text_purple_600, text_purple_700, text_purple_800, text_purple_900, text_purple_950,
    text_fuchsia_50, text_fuchsia_100, text_fuchsia_200, text_fuchsia_300, text_fuchsia_400, text_fuchsia_500, text_fuchsia_600, text_fuchsia_700, text_fuchsia_800, text_fuchsia_900, text_fuchsia_950,
    text_pink_50, text_pink_100, text_pink_200, text_pink_300, text_pink_400, text_pink_500, text_pink_600, text_pink_700, text_pink_800, text_pink_900, text_pink_950,
    text_rose_50, text_rose_100, text_rose_200, text_rose_300, text_rose_400, text_rose_500, text_rose_600, text_rose_700, text_rose_800, text_rose_900, text_rose_950,
    border_neutral_50, border_neutral_100, border_neutral_200, border_neutral_300, border_neutral_400, border_neutral_500, border_neutral_600, border_neutral_700, border_neutral_800, border_neutral_900, border_neutral_950,
    border_stone_50, border_stone_100, border_stone_200, border_stone_300, border_stone_400, border_stone_500, border_stone_600, border_stone_700, border_stone_800, border_stone_900, border_stone_950,
    border_amber_50, border_amber_100, border_amber_200, border_amber_300, border_amber_400, border_amber_500, border_amber_600, border_amber_700, border_amber_800, border_amber_900, border_amber_950,
    border_lime_50, border_lime_100, border_lime_200, border_lime_300, border_lime_400, border_lime_500, border_lime_600, border_lime_700, border_lime_800, border_lime_900, border_lime_950,
    border_green_50, border_green_100, border_green_200, border_green_300, border_green_400, border_green_500, border_green_600, border_green_700, border_green_800, border_green_900, border_green_950,
    border_emerald_50, border_emerald_100, border_emerald_200, border_emerald_300, border_emerald_400, border_emerald_500, border_emerald_600, border_emerald_700, border_emerald_800, border_emerald_900, border_emerald_950,
    border_teal_50, border_teal_100, border_teal_200, border_teal_300, border_teal_400, border_teal_500, border_teal_600, border_teal_700, border_teal_800, border_teal_900, border_teal_950,
    border_cyan_50, border_cyan_100, border_cyan_200, border_cyan_300, border_cyan_400, border_cyan_500, border_cyan_600, border_cyan_700, border_cyan_800, border_cyan_900, border_cyan_950,
    border_sky_50, border_sky_100, border_sky_200, border_sky_300, border_sky_400, border_sky_500, border_sky_600, border_sky_700, border_sky_800, border_sky_900, border_sky_950,
    border_blue_50, border_blue_100, border_blue_200, border_blue_300, border_blue_400, border_blue_500, border_blue_600, border_blue_700, border_blue_800, border_blue_900, border_blue_950,
    border_indigo_50, border_indigo_100, border_indigo_200, border_indigo_300, border_indigo_400, border_indigo_500, border_indigo_600, border_indigo_700, border_indigo_800, border_indigo_900, border_indigo_950,
    border_violet_50, border_violet_100, border_violet_200, border_violet_300, border_violet_400, border_violet_500, border_violet_600, border_violet_700, border_violet_800, border_violet_900, border_violet_950,
    border_purple_50, border_purple_100, border_purple_200, border_purple_300, border_purple_400, border_purple_500, border_purple_600, border_purple_700, border_purple_800, border_purple_900, border_purple_950,
    border_fuchsia_50, border_fuchsia_100, border_fuchsia_200, border_fuchsia_300, border_fuchsia_400, border_fuchsia_500, border_fuchsia_600, border_fuchsia_700, border_fuchsia_800, border_fuchsia_900, border_fuchsia_950,
    border_pink_50, border_pink_100, border_pink_200, border_pink_300, border_pink_400, border_pink_500, border_pink_600, border_pink_700, border_pink_800, border_pink_900, border_pink_950,
    border_rose_50, border_rose_100, border_rose_200, border_rose_300, border_rose_400, border_rose_500, border_rose_600, border_rose_700, border_rose_800, border_rose_900, border_rose_950
);

test_trait_methods!(TailwindExt, test_original_ext,
    p_4, m_2, rounded_md, text_slate_900, w_full
);
