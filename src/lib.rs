//! # floem-tailwind
//!
//! Tailwind-style utility methods for Floem styling.
//!
//! This crate provides a `TailwindExt` trait that extends Floem's `Style` with
//! shorthand methods following Tailwind CSS naming conventions.
//!
//! ## Spacing Scale
//!
//! The numeric scale follows Tailwind's convention where each unit = 4px (0.25rem):
//! - `w_0()` = 0px
//! - `w_1()` = 4px
//! - `w_2()` = 8px
//! - `w_4()` = 16px
//! - `w_8()` = 32px
//! - etc.
//!
//! ## Usage
//!
//! ```rust
//! use floem::style::Style;
//! use floem_tailwind::TailwindExt;
//!
//! let style = Style::new()
//!     .w_64()        // width: 256px
//!     .h_32()        // height: 128px
//!     .p_4()         // padding: 16px
//!     .gap_2()       // gap: 8px
//!     .rounded_lg(); // border-radius: 8px
//! ```
//!
//! ## Colors
//!
//! ```rust
//! use floem::style::Style;
//! use floem_tailwind::TailwindExt;
//!
//! let style = Style::new()
//!     .bg_blue_500()     // background: blue-500
//!     .text_white()      // text color: white
//!     .border_gray_300(); // border color: gray-300
//! ```

use floem::style::Style;
use floem::unit::{Pct, PxPctAuto};
use peniko::Color;

pub mod colors;

/// Tailwind-style spacing scale (in pixels)
/// Each unit = 4px (following Tailwind's 0.25rem base with 16px root)
pub mod spacing {
    pub const SPACING_0: f64 = 0.0;
    pub const SPACING_PX: f64 = 1.0;
    pub const SPACING_0_5: f64 = 2.0;
    pub const SPACING_1: f64 = 4.0;
    pub const SPACING_1_5: f64 = 6.0;
    pub const SPACING_2: f64 = 8.0;
    pub const SPACING_2_5: f64 = 10.0;
    pub const SPACING_3: f64 = 12.0;
    pub const SPACING_3_5: f64 = 14.0;
    pub const SPACING_4: f64 = 16.0;
    pub const SPACING_5: f64 = 20.0;
    pub const SPACING_6: f64 = 24.0;
    pub const SPACING_7: f64 = 28.0;
    pub const SPACING_8: f64 = 32.0;
    pub const SPACING_9: f64 = 36.0;
    pub const SPACING_10: f64 = 40.0;
    pub const SPACING_11: f64 = 44.0;
    pub const SPACING_12: f64 = 48.0;
    pub const SPACING_14: f64 = 56.0;
    pub const SPACING_16: f64 = 64.0;
    pub const SPACING_20: f64 = 80.0;
    pub const SPACING_24: f64 = 96.0;
    pub const SPACING_28: f64 = 112.0;
    pub const SPACING_32: f64 = 128.0;
    pub const SPACING_36: f64 = 144.0;
    pub const SPACING_40: f64 = 160.0;
    pub const SPACING_44: f64 = 176.0;
    pub const SPACING_48: f64 = 192.0;
    pub const SPACING_52: f64 = 208.0;
    pub const SPACING_56: f64 = 224.0;
    pub const SPACING_60: f64 = 240.0;
    pub const SPACING_64: f64 = 256.0;
    pub const SPACING_72: f64 = 288.0;
    pub const SPACING_80: f64 = 320.0;
    pub const SPACING_96: f64 = 384.0;

    // Named container sizes
    pub const SIZE_XS: f64 = 320.0;  // 20rem
    pub const SIZE_SM: f64 = 384.0;  // 24rem
    pub const SIZE_MD: f64 = 448.0;  // 28rem
    pub const SIZE_LG: f64 = 512.0;  // 32rem
    pub const SIZE_XL: f64 = 576.0;  // 36rem
    pub const SIZE_2XL: f64 = 672.0; // 42rem
    pub const SIZE_3XL: f64 = 768.0; // 48rem
    pub const SIZE_4XL: f64 = 896.0; // 56rem
    pub const SIZE_5XL: f64 = 1024.0; // 64rem
    pub const SIZE_6XL: f64 = 1152.0; // 72rem
    pub const SIZE_7XL: f64 = 1280.0; // 80rem
}

/// Border radius scale (in pixels)
pub mod radius {
    pub const ROUNDED_NONE: f64 = 0.0;
    pub const ROUNDED_SM: f64 = 2.0;
    pub const ROUNDED: f64 = 4.0;
    pub const ROUNDED_MD: f64 = 6.0;
    pub const ROUNDED_LG: f64 = 8.0;
    pub const ROUNDED_XL: f64 = 12.0;
    pub const ROUNDED_2XL: f64 = 16.0;
    pub const ROUNDED_3XL: f64 = 24.0;
    pub const ROUNDED_FULL: f64 = 9999.0;
}

/// Font size scale (in pixels)
/// Based on Tailwind's default font sizes with 16px base
pub mod font_size {
    pub const TEXT_XS: f32 = 12.0;    // 0.75rem
    pub const TEXT_SM: f32 = 14.0;    // 0.875rem
    pub const TEXT_BASE: f32 = 16.0;  // 1rem
    pub const TEXT_LG: f32 = 18.0;    // 1.125rem
    pub const TEXT_XL: f32 = 20.0;    // 1.25rem
    pub const TEXT_2XL: f32 = 24.0;   // 1.5rem
    pub const TEXT_3XL: f32 = 30.0;   // 1.875rem
    pub const TEXT_4XL: f32 = 36.0;   // 2.25rem
    pub const TEXT_5XL: f32 = 48.0;   // 3rem
    pub const TEXT_6XL: f32 = 60.0;   // 3.75rem
    pub const TEXT_7XL: f32 = 72.0;   // 4.5rem
    pub const TEXT_8XL: f32 = 96.0;   // 6rem
    pub const TEXT_9XL: f32 = 128.0;  // 8rem
}

/// Font weight values matching Tailwind CSS
pub mod font_weight {
    use floem::text::Weight;

    pub const THIN: Weight = Weight::THIN;             // 100
    pub const EXTRALIGHT: Weight = Weight::EXTRA_LIGHT; // 200
    pub const LIGHT: Weight = Weight::LIGHT;           // 300
    pub const NORMAL: Weight = Weight::NORMAL;         // 400
    pub const MEDIUM: Weight = Weight::MEDIUM;         // 500
    pub const SEMIBOLD: Weight = Weight::SEMIBOLD;     // 600
    pub const BOLD: Weight = Weight::BOLD;             // 700
    pub const EXTRABOLD: Weight = Weight::EXTRA_BOLD;  // 800
    pub const BLACK: Weight = Weight::BLACK;           // 900
}

/// Line height values matching Tailwind CSS leading-* utilities
pub mod line_height {
    pub const LEADING_NONE: f32 = 1.0;
    pub const LEADING_TIGHT: f32 = 1.25;
    pub const LEADING_SNUG: f32 = 1.375;
    pub const LEADING_NORMAL: f32 = 1.5;
    pub const LEADING_RELAXED: f32 = 1.625;
    pub const LEADING_LOOSE: f32 = 2.0;
}

/// Box shadow presets matching Tailwind CSS shadow-* utilities
/// Each shadow is defined by (h_offset, v_offset, blur, spread, opacity)
pub mod shadow {
    use floem::style::BoxShadow;
    use peniko::Color;

    /// Creates a shadow color with the given opacity (0.0 - 1.0)
    fn shadow_color(opacity: f32) -> Color {
        Color::from_rgba8(0, 0, 0, (opacity * 255.0) as u8)
    }

    /// shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05)
    pub fn shadow_sm() -> BoxShadow {
        BoxShadow::new()
            .h_offset(0.0)
            .v_offset(1.0)
            .blur_radius(2.0)
            .spread(0.0)
            .color(shadow_color(0.05))
    }

    /// shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)
    /// (simplified to single shadow)
    pub fn shadow_default() -> BoxShadow {
        BoxShadow::new()
            .h_offset(0.0)
            .v_offset(1.0)
            .blur_radius(3.0)
            .spread(0.0)
            .color(shadow_color(0.1))
    }

    /// shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1)
    pub fn shadow_md() -> BoxShadow {
        BoxShadow::new()
            .h_offset(0.0)
            .v_offset(4.0)
            .blur_radius(6.0)
            .spread(-1.0)
            .color(shadow_color(0.1))
    }

    /// shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1)
    pub fn shadow_lg() -> BoxShadow {
        BoxShadow::new()
            .h_offset(0.0)
            .v_offset(10.0)
            .blur_radius(15.0)
            .spread(-3.0)
            .color(shadow_color(0.1))
    }

    /// shadow-xl: 0 20px 25px -5px rgb(0 0 0 / 0.1)
    pub fn shadow_xl() -> BoxShadow {
        BoxShadow::new()
            .h_offset(0.0)
            .v_offset(20.0)
            .blur_radius(25.0)
            .spread(-5.0)
            .color(shadow_color(0.1))
    }

    /// shadow-2xl: 0 25px 50px -12px rgb(0 0 0 / 0.25)
    pub fn shadow_2xl() -> BoxShadow {
        BoxShadow::new()
            .h_offset(0.0)
            .v_offset(25.0)
            .blur_radius(50.0)
            .spread(-12.0)
            .color(shadow_color(0.25))
    }
}

/// Macro to generate width methods
macro_rules! width_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement width methods
macro_rules! impl_width_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.width($value)
            }
        )*
    };
}

/// Macro to generate height methods
macro_rules! height_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement height methods
macro_rules! impl_height_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.height($value)
            }
        )*
    };
}

/// Macro to generate size methods (both width and height)
macro_rules! size_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement size methods
macro_rules! impl_size_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.width($value).height($value)
            }
        )*
    };
}

/// Macro to generate min-width methods
macro_rules! min_width_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement min-width methods
macro_rules! impl_min_width_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.min_width($value)
            }
        )*
    };
}

/// Macro to generate max-width methods
macro_rules! max_width_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement max-width methods
macro_rules! impl_max_width_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.max_width($value)
            }
        )*
    };
}

/// Macro to generate padding methods
macro_rules! padding_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement padding methods
macro_rules! impl_padding_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.padding($value)
            }
        )*
    };
}

/// Macro to generate horizontal padding methods
macro_rules! padding_x_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement horizontal padding methods
macro_rules! impl_padding_x_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.padding_horiz($value)
            }
        )*
    };
}

/// Macro to generate vertical padding methods
macro_rules! padding_y_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement vertical padding methods
macro_rules! impl_padding_y_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.padding_vert($value)
            }
        )*
    };
}

/// Macro to generate margin methods
macro_rules! margin_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement margin methods
macro_rules! impl_margin_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.margin($value)
            }
        )*
    };
}

/// Macro to generate horizontal margin methods
macro_rules! margin_x_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement horizontal margin methods
macro_rules! impl_margin_x_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.margin_horiz($value)
            }
        )*
    };
}

/// Macro to generate vertical margin methods
macro_rules! margin_y_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement vertical margin methods
macro_rules! impl_margin_y_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.margin_vert($value)
            }
        )*
    };
}

/// Macro to generate gap methods
macro_rules! gap_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement gap methods
macro_rules! impl_gap_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.gap($value)
            }
        )*
    };
}

/// Macro to generate border-radius methods
macro_rules! rounded_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement border-radius methods
macro_rules! impl_rounded_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.border_radius($value)
            }
        )*
    };
}

/// Macro to generate font-size methods
macro_rules! font_size_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement font-size methods
macro_rules! impl_font_size_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.font_size($value)
            }
        )*
    };
}

/// Macro to generate font-weight methods
macro_rules! font_weight_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement font-weight methods
macro_rules! impl_font_weight_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.font_weight($value)
            }
        )*
    };
}

/// Macro to generate line-height methods
macro_rules! line_height_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self;
        )*
    };
}

/// Macro to implement line-height methods
macro_rules! impl_line_height_methods {
    ($($name:ident => $value:expr),* $(,)?) => {
        $(
            fn $name(self) -> Self {
                self.line_height($value)
            }
        )*
    };
}

/// Extension trait that adds Tailwind-style utility methods to Floem's Style.
pub trait TailwindExt: Sized {
    // === Width Methods ===
    width_methods! {
        w_0 => 0.0,
        w_px => 1.0,
        w_0p5 => spacing::SPACING_0_5,
        w_1 => spacing::SPACING_1,
        w_1p5 => spacing::SPACING_1_5,
        w_2 => spacing::SPACING_2,
        w_2p5 => spacing::SPACING_2_5,
        w_3 => spacing::SPACING_3,
        w_3p5 => spacing::SPACING_3_5,
        w_4 => spacing::SPACING_4,
        w_5 => spacing::SPACING_5,
        w_6 => spacing::SPACING_6,
        w_7 => spacing::SPACING_7,
        w_8 => spacing::SPACING_8,
        w_9 => spacing::SPACING_9,
        w_10 => spacing::SPACING_10,
        w_11 => spacing::SPACING_11,
        w_12 => spacing::SPACING_12,
        w_14 => spacing::SPACING_14,
        w_16 => spacing::SPACING_16,
        w_20 => spacing::SPACING_20,
        w_24 => spacing::SPACING_24,
        w_28 => spacing::SPACING_28,
        w_32 => spacing::SPACING_32,
        w_36 => spacing::SPACING_36,
        w_40 => spacing::SPACING_40,
        w_44 => spacing::SPACING_44,
        w_48 => spacing::SPACING_48,
        w_52 => spacing::SPACING_52,
        w_56 => spacing::SPACING_56,
        w_60 => spacing::SPACING_60,
        w_64 => spacing::SPACING_64,
        w_72 => spacing::SPACING_72,
        w_80 => spacing::SPACING_80,
        w_96 => spacing::SPACING_96,
    }

    // Named width sizes
    fn w_xs(self) -> Self;
    fn w_sm(self) -> Self;
    fn w_md(self) -> Self;
    fn w_lg(self) -> Self;
    fn w_xl(self) -> Self;
    fn w_2xl(self) -> Self;
    fn w_3xl(self) -> Self;
    fn w_4xl(self) -> Self;
    fn w_5xl(self) -> Self;
    fn w_6xl(self) -> Self;
    fn w_7xl(self) -> Self;

    // Percentage widths
    fn w_full(self) -> Self;
    fn w_auto(self) -> Self;
    // Fractional widths (GPUI-style: underscore = fraction, e.g., w_1_2 = 1/2 = 50%)
    fn w_1_2(self) -> Self;   // 1/2 = 50%
    fn w_1_3(self) -> Self;   // 1/3 = 33.33%
    fn w_2_3(self) -> Self;   // 2/3 = 66.67%
    fn w_1_4(self) -> Self;   // 1/4 = 25%
    fn w_3_4(self) -> Self;   // 3/4 = 75%
    fn w_1_5(self) -> Self;   // 1/5 = 20%
    fn w_2_5(self) -> Self;   // 2/5 = 40%
    fn w_3_5(self) -> Self;   // 3/5 = 60%
    fn w_4_5(self) -> Self;   // 4/5 = 80%
    fn w_1_6(self) -> Self;   // 1/6 = 16.67%
    fn w_5_6(self) -> Self;   // 5/6 = 83.33%
    fn w_1_12(self) -> Self;  // 1/12 = 8.33%

    // === Height Methods ===
    height_methods! {
        h_0 => 0.0,
        h_px => 1.0,
        h_0p5 => spacing::SPACING_0_5,
        h_1 => spacing::SPACING_1,
        h_1p5 => spacing::SPACING_1_5,
        h_2 => spacing::SPACING_2,
        h_2p5 => spacing::SPACING_2_5,
        h_3 => spacing::SPACING_3,
        h_3p5 => spacing::SPACING_3_5,
        h_4 => spacing::SPACING_4,
        h_5 => spacing::SPACING_5,
        h_6 => spacing::SPACING_6,
        h_7 => spacing::SPACING_7,
        h_8 => spacing::SPACING_8,
        h_9 => spacing::SPACING_9,
        h_10 => spacing::SPACING_10,
        h_11 => spacing::SPACING_11,
        h_12 => spacing::SPACING_12,
        h_14 => spacing::SPACING_14,
        h_16 => spacing::SPACING_16,
        h_20 => spacing::SPACING_20,
        h_24 => spacing::SPACING_24,
        h_28 => spacing::SPACING_28,
        h_32 => spacing::SPACING_32,
        h_36 => spacing::SPACING_36,
        h_40 => spacing::SPACING_40,
        h_44 => spacing::SPACING_44,
        h_48 => spacing::SPACING_48,
        h_52 => spacing::SPACING_52,
        h_56 => spacing::SPACING_56,
        h_60 => spacing::SPACING_60,
        h_64 => spacing::SPACING_64,
        h_72 => spacing::SPACING_72,
        h_80 => spacing::SPACING_80,
        h_96 => spacing::SPACING_96,
    }

    // Named height sizes
    fn h_xs(self) -> Self;
    fn h_sm(self) -> Self;
    fn h_md(self) -> Self;
    fn h_lg(self) -> Self;
    fn h_xl(self) -> Self;
    fn h_2xl(self) -> Self;
    fn h_3xl(self) -> Self;
    fn h_4xl(self) -> Self;
    fn h_5xl(self) -> Self;
    fn h_6xl(self) -> Self;
    fn h_7xl(self) -> Self;

    // Percentage heights
    fn h_full(self) -> Self;
    fn h_auto(self) -> Self;
    // Fractional heights (GPUI-style: underscore = fraction, e.g., h_1_2 = 1/2 = 50%)
    fn h_1_2(self) -> Self;   // 1/2 = 50%
    fn h_1_3(self) -> Self;   // 1/3 = 33.33%
    fn h_2_3(self) -> Self;   // 2/3 = 66.67%
    fn h_1_4(self) -> Self;   // 1/4 = 25%
    fn h_3_4(self) -> Self;   // 3/4 = 75%
    fn h_1_5(self) -> Self;   // 1/5 = 20%
    fn h_2_5(self) -> Self;   // 2/5 = 40%
    fn h_3_5(self) -> Self;   // 3/5 = 60%
    fn h_4_5(self) -> Self;   // 4/5 = 80%
    fn h_1_6(self) -> Self;   // 1/6 = 16.67%
    fn h_5_6(self) -> Self;   // 5/6 = 83.33%
    fn h_1_12(self) -> Self;  // 1/12 = 8.33%

    // === Size Methods (width + height) ===
    size_methods! {
        size_0 => 0.0,
        size_px => 1.0,
        size_0p5 => spacing::SPACING_0_5,
        size_1 => spacing::SPACING_1,
        size_1p5 => spacing::SPACING_1_5,
        size_2 => spacing::SPACING_2,
        size_2p5 => spacing::SPACING_2_5,
        size_3 => spacing::SPACING_3,
        size_3p5 => spacing::SPACING_3_5,
        size_4 => spacing::SPACING_4,
        size_5 => spacing::SPACING_5,
        size_6 => spacing::SPACING_6,
        size_7 => spacing::SPACING_7,
        size_8 => spacing::SPACING_8,
        size_9 => spacing::SPACING_9,
        size_10 => spacing::SPACING_10,
        size_11 => spacing::SPACING_11,
        size_12 => spacing::SPACING_12,
        size_14 => spacing::SPACING_14,
        size_16 => spacing::SPACING_16,
        size_20 => spacing::SPACING_20,
        size_24 => spacing::SPACING_24,
        size_28 => spacing::SPACING_28,
        size_32 => spacing::SPACING_32,
        size_36 => spacing::SPACING_36,
        size_40 => spacing::SPACING_40,
        size_44 => spacing::SPACING_44,
        size_48 => spacing::SPACING_48,
        size_52 => spacing::SPACING_52,
        size_56 => spacing::SPACING_56,
        size_60 => spacing::SPACING_60,
        size_64 => spacing::SPACING_64,
        size_72 => spacing::SPACING_72,
        size_80 => spacing::SPACING_80,
        size_96 => spacing::SPACING_96,
    }

    // === Min-Width Methods ===
    min_width_methods! {
        min_w_0 => 0.0,
        min_w_px => 1.0,
        min_w_1 => spacing::SPACING_1,
        min_w_2 => spacing::SPACING_2,
        min_w_4 => spacing::SPACING_4,
        min_w_8 => spacing::SPACING_8,
        min_w_16 => spacing::SPACING_16,
        min_w_32 => spacing::SPACING_32,
        min_w_64 => spacing::SPACING_64,
        min_w_96 => spacing::SPACING_96,
    }
    fn min_w_full(self) -> Self;
    fn min_w_xs(self) -> Self;
    fn min_w_sm(self) -> Self;
    fn min_w_md(self) -> Self;
    fn min_w_lg(self) -> Self;
    fn min_w_xl(self) -> Self;

    // === Max-Width Methods ===
    max_width_methods! {
        max_w_0 => 0.0,
        max_w_px => 1.0,
        max_w_1 => spacing::SPACING_1,
        max_w_2 => spacing::SPACING_2,
        max_w_4 => spacing::SPACING_4,
        max_w_8 => spacing::SPACING_8,
        max_w_16 => spacing::SPACING_16,
        max_w_32 => spacing::SPACING_32,
        max_w_64 => spacing::SPACING_64,
        max_w_96 => spacing::SPACING_96,
    }
    fn max_w_full(self) -> Self;
    fn max_w_xs(self) -> Self;
    fn max_w_sm(self) -> Self;
    fn max_w_md(self) -> Self;
    fn max_w_lg(self) -> Self;
    fn max_w_xl(self) -> Self;
    fn max_w_2xl(self) -> Self;
    fn max_w_3xl(self) -> Self;
    fn max_w_4xl(self) -> Self;
    fn max_w_5xl(self) -> Self;
    fn max_w_6xl(self) -> Self;
    fn max_w_7xl(self) -> Self;

    // === Padding Methods ===
    padding_methods! {
        p_0 => 0.0,
        p_px => 1.0,
        p_0p5 => spacing::SPACING_0_5,
        p_1 => spacing::SPACING_1,
        p_1p5 => spacing::SPACING_1_5,
        p_2 => spacing::SPACING_2,
        p_2p5 => spacing::SPACING_2_5,
        p_3 => spacing::SPACING_3,
        p_3p5 => spacing::SPACING_3_5,
        p_4 => spacing::SPACING_4,
        p_5 => spacing::SPACING_5,
        p_6 => spacing::SPACING_6,
        p_7 => spacing::SPACING_7,
        p_8 => spacing::SPACING_8,
        p_9 => spacing::SPACING_9,
        p_10 => spacing::SPACING_10,
        p_11 => spacing::SPACING_11,
        p_12 => spacing::SPACING_12,
        p_14 => spacing::SPACING_14,
        p_16 => spacing::SPACING_16,
        p_20 => spacing::SPACING_20,
        p_24 => spacing::SPACING_24,
    }

    // Horizontal padding (px-*)
    padding_x_methods! {
        px_0 => 0.0,
        px_px => 1.0,
        px_0p5 => spacing::SPACING_0_5,
        px_1 => spacing::SPACING_1,
        px_1p5 => spacing::SPACING_1_5,
        px_2 => spacing::SPACING_2,
        px_2p5 => spacing::SPACING_2_5,
        px_3 => spacing::SPACING_3,
        px_3p5 => spacing::SPACING_3_5,
        px_4 => spacing::SPACING_4,
        px_5 => spacing::SPACING_5,
        px_6 => spacing::SPACING_6,
        px_7 => spacing::SPACING_7,
        px_8 => spacing::SPACING_8,
        px_9 => spacing::SPACING_9,
        px_10 => spacing::SPACING_10,
        px_12 => spacing::SPACING_12,
        px_16 => spacing::SPACING_16,
        px_20 => spacing::SPACING_20,
        px_24 => spacing::SPACING_24,
    }

    // Vertical padding (py-*)
    padding_y_methods! {
        py_0 => 0.0,
        py_px => 1.0,
        py_0p5 => spacing::SPACING_0_5,
        py_1 => spacing::SPACING_1,
        py_1p5 => spacing::SPACING_1_5,
        py_2 => spacing::SPACING_2,
        py_2p5 => spacing::SPACING_2_5,
        py_3 => spacing::SPACING_3,
        py_3p5 => spacing::SPACING_3_5,
        py_4 => spacing::SPACING_4,
        py_5 => spacing::SPACING_5,
        py_6 => spacing::SPACING_6,
        py_7 => spacing::SPACING_7,
        py_8 => spacing::SPACING_8,
        py_9 => spacing::SPACING_9,
        py_10 => spacing::SPACING_10,
        py_12 => spacing::SPACING_12,
        py_16 => spacing::SPACING_16,
        py_20 => spacing::SPACING_20,
        py_24 => spacing::SPACING_24,
    }

    // Individual padding sides
    fn pt_0(self) -> Self;
    fn pt_1(self) -> Self;
    fn pt_2(self) -> Self;
    fn pt_3(self) -> Self;
    fn pt_4(self) -> Self;
    fn pt_5(self) -> Self;
    fn pt_6(self) -> Self;
    fn pt_8(self) -> Self;

    fn pb_0(self) -> Self;
    fn pb_1(self) -> Self;
    fn pb_2(self) -> Self;
    fn pb_3(self) -> Self;
    fn pb_4(self) -> Self;
    fn pb_5(self) -> Self;
    fn pb_6(self) -> Self;
    fn pb_8(self) -> Self;

    fn pl_0(self) -> Self;
    fn pl_1(self) -> Self;
    fn pl_2(self) -> Self;
    fn pl_3(self) -> Self;
    fn pl_4(self) -> Self;
    fn pl_5(self) -> Self;
    fn pl_6(self) -> Self;
    fn pl_8(self) -> Self;

    fn pr_0(self) -> Self;
    fn pr_1(self) -> Self;
    fn pr_2(self) -> Self;
    fn pr_3(self) -> Self;
    fn pr_4(self) -> Self;
    fn pr_5(self) -> Self;
    fn pr_6(self) -> Self;
    fn pr_8(self) -> Self;

    // === Margin Methods ===
    margin_methods! {
        m_0 => 0.0,
        m_px => 1.0,
        m_0p5 => spacing::SPACING_0_5,
        m_1 => spacing::SPACING_1,
        m_1p5 => spacing::SPACING_1_5,
        m_2 => spacing::SPACING_2,
        m_2p5 => spacing::SPACING_2_5,
        m_3 => spacing::SPACING_3,
        m_3p5 => spacing::SPACING_3_5,
        m_4 => spacing::SPACING_4,
        m_5 => spacing::SPACING_5,
        m_6 => spacing::SPACING_6,
        m_7 => spacing::SPACING_7,
        m_8 => spacing::SPACING_8,
        m_9 => spacing::SPACING_9,
        m_10 => spacing::SPACING_10,
        m_11 => spacing::SPACING_11,
        m_12 => spacing::SPACING_12,
        m_14 => spacing::SPACING_14,
        m_16 => spacing::SPACING_16,
        m_20 => spacing::SPACING_20,
        m_24 => spacing::SPACING_24,
    }

    fn m_auto(self) -> Self;

    // Horizontal margin (mx-*)
    margin_x_methods! {
        mx_0 => 0.0,
        mx_px => 1.0,
        mx_0p5 => spacing::SPACING_0_5,
        mx_1 => spacing::SPACING_1,
        mx_1p5 => spacing::SPACING_1_5,
        mx_2 => spacing::SPACING_2,
        mx_2p5 => spacing::SPACING_2_5,
        mx_3 => spacing::SPACING_3,
        mx_3p5 => spacing::SPACING_3_5,
        mx_4 => spacing::SPACING_4,
        mx_5 => spacing::SPACING_5,
        mx_6 => spacing::SPACING_6,
        mx_8 => spacing::SPACING_8,
        mx_10 => spacing::SPACING_10,
        mx_12 => spacing::SPACING_12,
        mx_16 => spacing::SPACING_16,
        mx_20 => spacing::SPACING_20,
        mx_24 => spacing::SPACING_24,
    }

    fn mx_auto(self) -> Self;

    // Vertical margin (my-*)
    margin_y_methods! {
        my_0 => 0.0,
        my_px => 1.0,
        my_0p5 => spacing::SPACING_0_5,
        my_1 => spacing::SPACING_1,
        my_1p5 => spacing::SPACING_1_5,
        my_2 => spacing::SPACING_2,
        my_2p5 => spacing::SPACING_2_5,
        my_3 => spacing::SPACING_3,
        my_3p5 => spacing::SPACING_3_5,
        my_4 => spacing::SPACING_4,
        my_5 => spacing::SPACING_5,
        my_6 => spacing::SPACING_6,
        my_8 => spacing::SPACING_8,
        my_10 => spacing::SPACING_10,
        my_12 => spacing::SPACING_12,
        my_16 => spacing::SPACING_16,
        my_20 => spacing::SPACING_20,
        my_24 => spacing::SPACING_24,
    }

    fn my_auto(self) -> Self;

    // Individual margin sides
    fn mt_0(self) -> Self;
    fn mt_1(self) -> Self;
    fn mt_2(self) -> Self;
    fn mt_3(self) -> Self;
    fn mt_4(self) -> Self;
    fn mt_5(self) -> Self;
    fn mt_6(self) -> Self;
    fn mt_8(self) -> Self;
    fn mt_auto(self) -> Self;

    fn mb_0(self) -> Self;
    fn mb_1(self) -> Self;
    fn mb_2(self) -> Self;
    fn mb_3(self) -> Self;
    fn mb_4(self) -> Self;
    fn mb_5(self) -> Self;
    fn mb_6(self) -> Self;
    fn mb_8(self) -> Self;
    fn mb_auto(self) -> Self;

    fn ml_0(self) -> Self;
    fn ml_1(self) -> Self;
    fn ml_2(self) -> Self;
    fn ml_3(self) -> Self;
    fn ml_4(self) -> Self;
    fn ml_5(self) -> Self;
    fn ml_6(self) -> Self;
    fn ml_8(self) -> Self;
    fn ml_auto(self) -> Self;

    fn mr_0(self) -> Self;
    fn mr_1(self) -> Self;
    fn mr_2(self) -> Self;
    fn mr_3(self) -> Self;
    fn mr_4(self) -> Self;
    fn mr_5(self) -> Self;
    fn mr_6(self) -> Self;
    fn mr_8(self) -> Self;
    fn mr_auto(self) -> Self;

    // === Gap Methods ===
    gap_methods! {
        gap_0 => 0.0,
        gap_px => 1.0,
        gap_0p5 => spacing::SPACING_0_5,
        gap_1 => spacing::SPACING_1,
        gap_1p5 => spacing::SPACING_1_5,
        gap_2 => spacing::SPACING_2,
        gap_2p5 => spacing::SPACING_2_5,
        gap_3 => spacing::SPACING_3,
        gap_3p5 => spacing::SPACING_3_5,
        gap_4 => spacing::SPACING_4,
        gap_5 => spacing::SPACING_5,
        gap_6 => spacing::SPACING_6,
        gap_7 => spacing::SPACING_7,
        gap_8 => spacing::SPACING_8,
        gap_9 => spacing::SPACING_9,
        gap_10 => spacing::SPACING_10,
        gap_11 => spacing::SPACING_11,
        gap_12 => spacing::SPACING_12,
        gap_14 => spacing::SPACING_14,
        gap_16 => spacing::SPACING_16,
        gap_20 => spacing::SPACING_20,
        gap_24 => spacing::SPACING_24,
    }

    // === Border Radius Methods ===
    rounded_methods! {
        rounded_none => radius::ROUNDED_NONE,
        rounded_sm => radius::ROUNDED_SM,
        rounded => radius::ROUNDED,
        rounded_md => radius::ROUNDED_MD,
        rounded_lg => radius::ROUNDED_LG,
        rounded_xl => radius::ROUNDED_XL,
        rounded_2xl => radius::ROUNDED_2XL,
        rounded_3xl => radius::ROUNDED_3XL,
        rounded_full => radius::ROUNDED_FULL,
    }

    // === Border Width Methods ===
    fn border_0(self) -> Self;
    fn border_1(self) -> Self;
    fn border_2(self) -> Self;
    fn border_4(self) -> Self;
    fn border_8(self) -> Self;

    // === Shadow Methods ===
    fn shadow_sm(self) -> Self;
    fn shadow(self) -> Self;
    fn shadow_md(self) -> Self;
    fn shadow_lg(self) -> Self;
    fn shadow_xl(self) -> Self;
    fn shadow_2xl(self) -> Self;
    fn shadow_none(self) -> Self;

    // === Background Color Methods ===
    fn bg(self, color: impl Into<Color>) -> Self;
    fn bg_transparent(self) -> Self;
    fn bg_black(self) -> Self;
    fn bg_white(self) -> Self;
    // Slate
    fn bg_slate_50(self) -> Self;
    fn bg_slate_100(self) -> Self;
    fn bg_slate_200(self) -> Self;
    fn bg_slate_300(self) -> Self;
    fn bg_slate_400(self) -> Self;
    fn bg_slate_500(self) -> Self;
    fn bg_slate_600(self) -> Self;
    fn bg_slate_700(self) -> Self;
    fn bg_slate_800(self) -> Self;
    fn bg_slate_900(self) -> Self;
    fn bg_slate_950(self) -> Self;
    // Gray
    fn bg_gray_50(self) -> Self;
    fn bg_gray_100(self) -> Self;
    fn bg_gray_200(self) -> Self;
    fn bg_gray_300(self) -> Self;
    fn bg_gray_400(self) -> Self;
    fn bg_gray_500(self) -> Self;
    fn bg_gray_600(self) -> Self;
    fn bg_gray_700(self) -> Self;
    fn bg_gray_800(self) -> Self;
    fn bg_gray_900(self) -> Self;
    fn bg_gray_950(self) -> Self;
    // Zinc
    fn bg_zinc_50(self) -> Self;
    fn bg_zinc_100(self) -> Self;
    fn bg_zinc_200(self) -> Self;
    fn bg_zinc_300(self) -> Self;
    fn bg_zinc_400(self) -> Self;
    fn bg_zinc_500(self) -> Self;
    fn bg_zinc_600(self) -> Self;
    fn bg_zinc_700(self) -> Self;
    fn bg_zinc_800(self) -> Self;
    fn bg_zinc_900(self) -> Self;
    fn bg_zinc_950(self) -> Self;
    // Red
    fn bg_red_50(self) -> Self;
    fn bg_red_100(self) -> Self;
    fn bg_red_200(self) -> Self;
    fn bg_red_300(self) -> Self;
    fn bg_red_400(self) -> Self;
    fn bg_red_500(self) -> Self;
    fn bg_red_600(self) -> Self;
    fn bg_red_700(self) -> Self;
    fn bg_red_800(self) -> Self;
    fn bg_red_900(self) -> Self;
    fn bg_red_950(self) -> Self;
    // Orange
    fn bg_orange_50(self) -> Self;
    fn bg_orange_100(self) -> Self;
    fn bg_orange_200(self) -> Self;
    fn bg_orange_300(self) -> Self;
    fn bg_orange_400(self) -> Self;
    fn bg_orange_500(self) -> Self;
    fn bg_orange_600(self) -> Self;
    fn bg_orange_700(self) -> Self;
    fn bg_orange_800(self) -> Self;
    fn bg_orange_900(self) -> Self;
    fn bg_orange_950(self) -> Self;
    // Yellow
    fn bg_yellow_50(self) -> Self;
    fn bg_yellow_100(self) -> Self;
    fn bg_yellow_200(self) -> Self;
    fn bg_yellow_300(self) -> Self;
    fn bg_yellow_400(self) -> Self;
    fn bg_yellow_500(self) -> Self;
    fn bg_yellow_600(self) -> Self;
    fn bg_yellow_700(self) -> Self;
    fn bg_yellow_800(self) -> Self;
    fn bg_yellow_900(self) -> Self;
    fn bg_yellow_950(self) -> Self;
    // Green
    fn bg_green_50(self) -> Self;
    fn bg_green_100(self) -> Self;
    fn bg_green_200(self) -> Self;
    fn bg_green_300(self) -> Self;
    fn bg_green_400(self) -> Self;
    fn bg_green_500(self) -> Self;
    fn bg_green_600(self) -> Self;
    fn bg_green_700(self) -> Self;
    fn bg_green_800(self) -> Self;
    fn bg_green_900(self) -> Self;
    fn bg_green_950(self) -> Self;
    // Blue
    fn bg_blue_50(self) -> Self;
    fn bg_blue_100(self) -> Self;
    fn bg_blue_200(self) -> Self;
    fn bg_blue_300(self) -> Self;
    fn bg_blue_400(self) -> Self;
    fn bg_blue_500(self) -> Self;
    fn bg_blue_600(self) -> Self;
    fn bg_blue_700(self) -> Self;
    fn bg_blue_800(self) -> Self;
    fn bg_blue_900(self) -> Self;
    fn bg_blue_950(self) -> Self;
    // Indigo
    fn bg_indigo_50(self) -> Self;
    fn bg_indigo_100(self) -> Self;
    fn bg_indigo_200(self) -> Self;
    fn bg_indigo_300(self) -> Self;
    fn bg_indigo_400(self) -> Self;
    fn bg_indigo_500(self) -> Self;
    fn bg_indigo_600(self) -> Self;
    fn bg_indigo_700(self) -> Self;
    fn bg_indigo_800(self) -> Self;
    fn bg_indigo_900(self) -> Self;
    fn bg_indigo_950(self) -> Self;
    // Purple
    fn bg_purple_50(self) -> Self;
    fn bg_purple_100(self) -> Self;
    fn bg_purple_200(self) -> Self;
    fn bg_purple_300(self) -> Self;
    fn bg_purple_400(self) -> Self;
    fn bg_purple_500(self) -> Self;
    fn bg_purple_600(self) -> Self;
    fn bg_purple_700(self) -> Self;
    fn bg_purple_800(self) -> Self;
    fn bg_purple_900(self) -> Self;
    fn bg_purple_950(self) -> Self;
    // Pink
    fn bg_pink_50(self) -> Self;
    fn bg_pink_100(self) -> Self;
    fn bg_pink_200(self) -> Self;
    fn bg_pink_300(self) -> Self;
    fn bg_pink_400(self) -> Self;
    fn bg_pink_500(self) -> Self;
    fn bg_pink_600(self) -> Self;
    fn bg_pink_700(self) -> Self;
    fn bg_pink_800(self) -> Self;
    fn bg_pink_900(self) -> Self;
    fn bg_pink_950(self) -> Self;

    // === Text Color Methods ===
    fn text(self, color: impl Into<Color>) -> Self;
    fn text_transparent(self) -> Self;
    fn text_black(self) -> Self;
    fn text_white(self) -> Self;
    // Slate
    fn text_slate_50(self) -> Self;
    fn text_slate_100(self) -> Self;
    fn text_slate_200(self) -> Self;
    fn text_slate_300(self) -> Self;
    fn text_slate_400(self) -> Self;
    fn text_slate_500(self) -> Self;
    fn text_slate_600(self) -> Self;
    fn text_slate_700(self) -> Self;
    fn text_slate_800(self) -> Self;
    fn text_slate_900(self) -> Self;
    fn text_slate_950(self) -> Self;
    // Gray
    fn text_gray_50(self) -> Self;
    fn text_gray_100(self) -> Self;
    fn text_gray_200(self) -> Self;
    fn text_gray_300(self) -> Self;
    fn text_gray_400(self) -> Self;
    fn text_gray_500(self) -> Self;
    fn text_gray_600(self) -> Self;
    fn text_gray_700(self) -> Self;
    fn text_gray_800(self) -> Self;
    fn text_gray_900(self) -> Self;
    fn text_gray_950(self) -> Self;
    // Red
    fn text_red_50(self) -> Self;
    fn text_red_100(self) -> Self;
    fn text_red_200(self) -> Self;
    fn text_red_300(self) -> Self;
    fn text_red_400(self) -> Self;
    fn text_red_500(self) -> Self;
    fn text_red_600(self) -> Self;
    fn text_red_700(self) -> Self;
    fn text_red_800(self) -> Self;
    fn text_red_900(self) -> Self;
    fn text_red_950(self) -> Self;
    // Green
    fn text_green_50(self) -> Self;
    fn text_green_100(self) -> Self;
    fn text_green_200(self) -> Self;
    fn text_green_300(self) -> Self;
    fn text_green_400(self) -> Self;
    fn text_green_500(self) -> Self;
    fn text_green_600(self) -> Self;
    fn text_green_700(self) -> Self;
    fn text_green_800(self) -> Self;
    fn text_green_900(self) -> Self;
    fn text_green_950(self) -> Self;
    // Blue
    fn text_blue_50(self) -> Self;
    fn text_blue_100(self) -> Self;
    fn text_blue_200(self) -> Self;
    fn text_blue_300(self) -> Self;
    fn text_blue_400(self) -> Self;
    fn text_blue_500(self) -> Self;
    fn text_blue_600(self) -> Self;
    fn text_blue_700(self) -> Self;
    fn text_blue_800(self) -> Self;
    fn text_blue_900(self) -> Self;
    fn text_blue_950(self) -> Self;

    // === Font Size Methods ===
    font_size_methods! {
        text_xs => font_size::TEXT_XS,
        text_sm => font_size::TEXT_SM,
        text_base => font_size::TEXT_BASE,
        text_lg => font_size::TEXT_LG,
        text_xl => font_size::TEXT_XL,
        text_2xl => font_size::TEXT_2XL,
        text_3xl => font_size::TEXT_3XL,
        text_4xl => font_size::TEXT_4XL,
        text_5xl => font_size::TEXT_5XL,
        text_6xl => font_size::TEXT_6XL,
        text_7xl => font_size::TEXT_7XL,
        text_8xl => font_size::TEXT_8XL,
        text_9xl => font_size::TEXT_9XL,
    }

    // === Font Weight Methods ===
    font_weight_methods! {
        font_thin => font_weight::THIN,
        font_extralight => font_weight::EXTRALIGHT,
        font_light => font_weight::LIGHT,
        font_normal => font_weight::NORMAL,
        font_medium => font_weight::MEDIUM,
        font_semibold => font_weight::SEMIBOLD,
        font_bold => font_weight::BOLD,
        font_extrabold => font_weight::EXTRABOLD,
        font_black => font_weight::BLACK,
    }

    // === Line Height Methods ===
    line_height_methods! {
        leading_none => line_height::LEADING_NONE,
        leading_tight => line_height::LEADING_TIGHT,
        leading_snug => line_height::LEADING_SNUG,
        leading_normal => line_height::LEADING_NORMAL,
        leading_relaxed => line_height::LEADING_RELAXED,
        leading_loose => line_height::LEADING_LOOSE,
    }

    // === Display Methods ===
    fn flex(self) -> Self;
    fn block(self) -> Self;
    fn grid(self) -> Self;
    fn hidden(self) -> Self;

    // === Flex Direction Methods ===
    fn flex_row(self) -> Self;
    fn flex_col(self) -> Self;
    fn flex_row_reverse(self) -> Self;
    fn flex_col_reverse(self) -> Self;

    // === Flex Wrap Methods ===
    fn wrap(self) -> Self;
    fn nowrap(self) -> Self;
    fn wrap_reverse(self) -> Self;

    // === Cursor Methods ===
    fn cursor_pointer(self) -> Self;
    fn cursor_default(self) -> Self;
    fn cursor_text(self) -> Self;
    fn cursor_move(self) -> Self;
    fn cursor_grab(self) -> Self;
    fn cursor_grabbing(self) -> Self;

    // === Border Color Methods ===
    fn border_transparent(self) -> Self;
    fn border_black(self) -> Self;
    fn border_white(self) -> Self;
    fn border_gray_200(self) -> Self;
    fn border_gray_300(self) -> Self;
    fn border_gray_400(self) -> Self;
    fn border_gray_500(self) -> Self;
    fn border_gray_600(self) -> Self;
    fn border_red_500(self) -> Self;
    fn border_blue_500(self) -> Self;
    fn border_green_500(self) -> Self;
}

impl TailwindExt for Style {
    // === Width Implementations ===
    impl_width_methods! {
        w_0 => 0.0,
        w_px => 1.0,
        w_0p5 => spacing::SPACING_0_5,
        w_1 => spacing::SPACING_1,
        w_1p5 => spacing::SPACING_1_5,
        w_2 => spacing::SPACING_2,
        w_2p5 => spacing::SPACING_2_5,
        w_3 => spacing::SPACING_3,
        w_3p5 => spacing::SPACING_3_5,
        w_4 => spacing::SPACING_4,
        w_5 => spacing::SPACING_5,
        w_6 => spacing::SPACING_6,
        w_7 => spacing::SPACING_7,
        w_8 => spacing::SPACING_8,
        w_9 => spacing::SPACING_9,
        w_10 => spacing::SPACING_10,
        w_11 => spacing::SPACING_11,
        w_12 => spacing::SPACING_12,
        w_14 => spacing::SPACING_14,
        w_16 => spacing::SPACING_16,
        w_20 => spacing::SPACING_20,
        w_24 => spacing::SPACING_24,
        w_28 => spacing::SPACING_28,
        w_32 => spacing::SPACING_32,
        w_36 => spacing::SPACING_36,
        w_40 => spacing::SPACING_40,
        w_44 => spacing::SPACING_44,
        w_48 => spacing::SPACING_48,
        w_52 => spacing::SPACING_52,
        w_56 => spacing::SPACING_56,
        w_60 => spacing::SPACING_60,
        w_64 => spacing::SPACING_64,
        w_72 => spacing::SPACING_72,
        w_80 => spacing::SPACING_80,
        w_96 => spacing::SPACING_96,
    }

    // Named width sizes
    fn w_xs(self) -> Self { self.width(spacing::SIZE_XS) }
    fn w_sm(self) -> Self { self.width(spacing::SIZE_SM) }
    fn w_md(self) -> Self { self.width(spacing::SIZE_MD) }
    fn w_lg(self) -> Self { self.width(spacing::SIZE_LG) }
    fn w_xl(self) -> Self { self.width(spacing::SIZE_XL) }
    fn w_2xl(self) -> Self { self.width(spacing::SIZE_2XL) }
    fn w_3xl(self) -> Self { self.width(spacing::SIZE_3XL) }
    fn w_4xl(self) -> Self { self.width(spacing::SIZE_4XL) }
    fn w_5xl(self) -> Self { self.width(spacing::SIZE_5XL) }
    fn w_6xl(self) -> Self { self.width(spacing::SIZE_6XL) }
    fn w_7xl(self) -> Self { self.width(spacing::SIZE_7XL) }

    // Percentage widths
    fn w_full(self) -> Self { self.width(Pct(100.0)) }
    fn w_auto(self) -> Self { self.width(PxPctAuto::Auto) }
    // Fractional widths (GPUI-style)
    fn w_1_2(self) -> Self { self.width(Pct(50.0)) }
    fn w_1_3(self) -> Self { self.width(Pct(33.333333)) }
    fn w_2_3(self) -> Self { self.width(Pct(66.666667)) }
    fn w_1_4(self) -> Self { self.width(Pct(25.0)) }
    fn w_3_4(self) -> Self { self.width(Pct(75.0)) }
    fn w_1_5(self) -> Self { self.width(Pct(20.0)) }
    fn w_2_5(self) -> Self { self.width(Pct(40.0)) }
    fn w_3_5(self) -> Self { self.width(Pct(60.0)) }
    fn w_4_5(self) -> Self { self.width(Pct(80.0)) }
    fn w_1_6(self) -> Self { self.width(Pct(16.666667)) }
    fn w_5_6(self) -> Self { self.width(Pct(83.333333)) }
    fn w_1_12(self) -> Self { self.width(Pct(8.333333)) }

    // === Height Implementations ===
    impl_height_methods! {
        h_0 => 0.0,
        h_px => 1.0,
        h_0p5 => spacing::SPACING_0_5,
        h_1 => spacing::SPACING_1,
        h_1p5 => spacing::SPACING_1_5,
        h_2 => spacing::SPACING_2,
        h_2p5 => spacing::SPACING_2_5,
        h_3 => spacing::SPACING_3,
        h_3p5 => spacing::SPACING_3_5,
        h_4 => spacing::SPACING_4,
        h_5 => spacing::SPACING_5,
        h_6 => spacing::SPACING_6,
        h_7 => spacing::SPACING_7,
        h_8 => spacing::SPACING_8,
        h_9 => spacing::SPACING_9,
        h_10 => spacing::SPACING_10,
        h_11 => spacing::SPACING_11,
        h_12 => spacing::SPACING_12,
        h_14 => spacing::SPACING_14,
        h_16 => spacing::SPACING_16,
        h_20 => spacing::SPACING_20,
        h_24 => spacing::SPACING_24,
        h_28 => spacing::SPACING_28,
        h_32 => spacing::SPACING_32,
        h_36 => spacing::SPACING_36,
        h_40 => spacing::SPACING_40,
        h_44 => spacing::SPACING_44,
        h_48 => spacing::SPACING_48,
        h_52 => spacing::SPACING_52,
        h_56 => spacing::SPACING_56,
        h_60 => spacing::SPACING_60,
        h_64 => spacing::SPACING_64,
        h_72 => spacing::SPACING_72,
        h_80 => spacing::SPACING_80,
        h_96 => spacing::SPACING_96,
    }

    // Named height sizes
    fn h_xs(self) -> Self { self.height(spacing::SIZE_XS) }
    fn h_sm(self) -> Self { self.height(spacing::SIZE_SM) }
    fn h_md(self) -> Self { self.height(spacing::SIZE_MD) }
    fn h_lg(self) -> Self { self.height(spacing::SIZE_LG) }
    fn h_xl(self) -> Self { self.height(spacing::SIZE_XL) }
    fn h_2xl(self) -> Self { self.height(spacing::SIZE_2XL) }
    fn h_3xl(self) -> Self { self.height(spacing::SIZE_3XL) }
    fn h_4xl(self) -> Self { self.height(spacing::SIZE_4XL) }
    fn h_5xl(self) -> Self { self.height(spacing::SIZE_5XL) }
    fn h_6xl(self) -> Self { self.height(spacing::SIZE_6XL) }
    fn h_7xl(self) -> Self { self.height(spacing::SIZE_7XL) }

    // Percentage heights
    fn h_full(self) -> Self { self.height(Pct(100.0)) }
    fn h_auto(self) -> Self { self.height(PxPctAuto::Auto) }
    // Fractional heights (GPUI-style)
    fn h_1_2(self) -> Self { self.height(Pct(50.0)) }
    fn h_1_3(self) -> Self { self.height(Pct(33.333333)) }
    fn h_2_3(self) -> Self { self.height(Pct(66.666667)) }
    fn h_1_4(self) -> Self { self.height(Pct(25.0)) }
    fn h_3_4(self) -> Self { self.height(Pct(75.0)) }
    fn h_1_5(self) -> Self { self.height(Pct(20.0)) }
    fn h_2_5(self) -> Self { self.height(Pct(40.0)) }
    fn h_3_5(self) -> Self { self.height(Pct(60.0)) }
    fn h_4_5(self) -> Self { self.height(Pct(80.0)) }
    fn h_1_6(self) -> Self { self.height(Pct(16.666667)) }
    fn h_5_6(self) -> Self { self.height(Pct(83.333333)) }
    fn h_1_12(self) -> Self { self.height(Pct(8.333333)) }

    // === Size Implementations (width + height) ===
    impl_size_methods! {
        size_0 => 0.0,
        size_px => 1.0,
        size_0p5 => spacing::SPACING_0_5,
        size_1 => spacing::SPACING_1,
        size_1p5 => spacing::SPACING_1_5,
        size_2 => spacing::SPACING_2,
        size_2p5 => spacing::SPACING_2_5,
        size_3 => spacing::SPACING_3,
        size_3p5 => spacing::SPACING_3_5,
        size_4 => spacing::SPACING_4,
        size_5 => spacing::SPACING_5,
        size_6 => spacing::SPACING_6,
        size_7 => spacing::SPACING_7,
        size_8 => spacing::SPACING_8,
        size_9 => spacing::SPACING_9,
        size_10 => spacing::SPACING_10,
        size_11 => spacing::SPACING_11,
        size_12 => spacing::SPACING_12,
        size_14 => spacing::SPACING_14,
        size_16 => spacing::SPACING_16,
        size_20 => spacing::SPACING_20,
        size_24 => spacing::SPACING_24,
        size_28 => spacing::SPACING_28,
        size_32 => spacing::SPACING_32,
        size_36 => spacing::SPACING_36,
        size_40 => spacing::SPACING_40,
        size_44 => spacing::SPACING_44,
        size_48 => spacing::SPACING_48,
        size_52 => spacing::SPACING_52,
        size_56 => spacing::SPACING_56,
        size_60 => spacing::SPACING_60,
        size_64 => spacing::SPACING_64,
        size_72 => spacing::SPACING_72,
        size_80 => spacing::SPACING_80,
        size_96 => spacing::SPACING_96,
    }

    // === Min-Width Implementations ===
    impl_min_width_methods! {
        min_w_0 => 0.0,
        min_w_px => 1.0,
        min_w_1 => spacing::SPACING_1,
        min_w_2 => spacing::SPACING_2,
        min_w_4 => spacing::SPACING_4,
        min_w_8 => spacing::SPACING_8,
        min_w_16 => spacing::SPACING_16,
        min_w_32 => spacing::SPACING_32,
        min_w_64 => spacing::SPACING_64,
        min_w_96 => spacing::SPACING_96,
    }
    fn min_w_full(self) -> Self { self.min_width(Pct(100.0)) }
    fn min_w_xs(self) -> Self { self.min_width(spacing::SIZE_XS) }
    fn min_w_sm(self) -> Self { self.min_width(spacing::SIZE_SM) }
    fn min_w_md(self) -> Self { self.min_width(spacing::SIZE_MD) }
    fn min_w_lg(self) -> Self { self.min_width(spacing::SIZE_LG) }
    fn min_w_xl(self) -> Self { self.min_width(spacing::SIZE_XL) }

    // === Max-Width Implementations ===
    impl_max_width_methods! {
        max_w_0 => 0.0,
        max_w_px => 1.0,
        max_w_1 => spacing::SPACING_1,
        max_w_2 => spacing::SPACING_2,
        max_w_4 => spacing::SPACING_4,
        max_w_8 => spacing::SPACING_8,
        max_w_16 => spacing::SPACING_16,
        max_w_32 => spacing::SPACING_32,
        max_w_64 => spacing::SPACING_64,
        max_w_96 => spacing::SPACING_96,
    }
    fn max_w_full(self) -> Self { self.max_width(Pct(100.0)) }
    fn max_w_xs(self) -> Self { self.max_width(spacing::SIZE_XS) }
    fn max_w_sm(self) -> Self { self.max_width(spacing::SIZE_SM) }
    fn max_w_md(self) -> Self { self.max_width(spacing::SIZE_MD) }
    fn max_w_lg(self) -> Self { self.max_width(spacing::SIZE_LG) }
    fn max_w_xl(self) -> Self { self.max_width(spacing::SIZE_XL) }
    fn max_w_2xl(self) -> Self { self.max_width(spacing::SIZE_2XL) }
    fn max_w_3xl(self) -> Self { self.max_width(spacing::SIZE_3XL) }
    fn max_w_4xl(self) -> Self { self.max_width(spacing::SIZE_4XL) }
    fn max_w_5xl(self) -> Self { self.max_width(spacing::SIZE_5XL) }
    fn max_w_6xl(self) -> Self { self.max_width(spacing::SIZE_6XL) }
    fn max_w_7xl(self) -> Self { self.max_width(spacing::SIZE_7XL) }

    // === Padding Implementations ===
    impl_padding_methods! {
        p_0 => 0.0,
        p_px => 1.0,
        p_0p5 => spacing::SPACING_0_5,
        p_1 => spacing::SPACING_1,
        p_1p5 => spacing::SPACING_1_5,
        p_2 => spacing::SPACING_2,
        p_2p5 => spacing::SPACING_2_5,
        p_3 => spacing::SPACING_3,
        p_3p5 => spacing::SPACING_3_5,
        p_4 => spacing::SPACING_4,
        p_5 => spacing::SPACING_5,
        p_6 => spacing::SPACING_6,
        p_7 => spacing::SPACING_7,
        p_8 => spacing::SPACING_8,
        p_9 => spacing::SPACING_9,
        p_10 => spacing::SPACING_10,
        p_11 => spacing::SPACING_11,
        p_12 => spacing::SPACING_12,
        p_14 => spacing::SPACING_14,
        p_16 => spacing::SPACING_16,
        p_20 => spacing::SPACING_20,
        p_24 => spacing::SPACING_24,
    }

    // Horizontal padding
    impl_padding_x_methods! {
        px_0 => 0.0,
        px_px => 1.0,
        px_0p5 => spacing::SPACING_0_5,
        px_1 => spacing::SPACING_1,
        px_1p5 => spacing::SPACING_1_5,
        px_2 => spacing::SPACING_2,
        px_2p5 => spacing::SPACING_2_5,
        px_3 => spacing::SPACING_3,
        px_3p5 => spacing::SPACING_3_5,
        px_4 => spacing::SPACING_4,
        px_5 => spacing::SPACING_5,
        px_6 => spacing::SPACING_6,
        px_7 => spacing::SPACING_7,
        px_8 => spacing::SPACING_8,
        px_9 => spacing::SPACING_9,
        px_10 => spacing::SPACING_10,
        px_12 => spacing::SPACING_12,
        px_16 => spacing::SPACING_16,
        px_20 => spacing::SPACING_20,
        px_24 => spacing::SPACING_24,
    }

    // Vertical padding
    impl_padding_y_methods! {
        py_0 => 0.0,
        py_px => 1.0,
        py_0p5 => spacing::SPACING_0_5,
        py_1 => spacing::SPACING_1,
        py_1p5 => spacing::SPACING_1_5,
        py_2 => spacing::SPACING_2,
        py_2p5 => spacing::SPACING_2_5,
        py_3 => spacing::SPACING_3,
        py_3p5 => spacing::SPACING_3_5,
        py_4 => spacing::SPACING_4,
        py_5 => spacing::SPACING_5,
        py_6 => spacing::SPACING_6,
        py_7 => spacing::SPACING_7,
        py_8 => spacing::SPACING_8,
        py_9 => spacing::SPACING_9,
        py_10 => spacing::SPACING_10,
        py_12 => spacing::SPACING_12,
        py_16 => spacing::SPACING_16,
        py_20 => spacing::SPACING_20,
        py_24 => spacing::SPACING_24,
    }

    // Individual padding sides
    fn pt_0(self) -> Self { self.padding_top(0.0) }
    fn pt_1(self) -> Self { self.padding_top(spacing::SPACING_1) }
    fn pt_2(self) -> Self { self.padding_top(spacing::SPACING_2) }
    fn pt_3(self) -> Self { self.padding_top(spacing::SPACING_3) }
    fn pt_4(self) -> Self { self.padding_top(spacing::SPACING_4) }
    fn pt_5(self) -> Self { self.padding_top(spacing::SPACING_5) }
    fn pt_6(self) -> Self { self.padding_top(spacing::SPACING_6) }
    fn pt_8(self) -> Self { self.padding_top(spacing::SPACING_8) }

    fn pb_0(self) -> Self { self.padding_bottom(0.0) }
    fn pb_1(self) -> Self { self.padding_bottom(spacing::SPACING_1) }
    fn pb_2(self) -> Self { self.padding_bottom(spacing::SPACING_2) }
    fn pb_3(self) -> Self { self.padding_bottom(spacing::SPACING_3) }
    fn pb_4(self) -> Self { self.padding_bottom(spacing::SPACING_4) }
    fn pb_5(self) -> Self { self.padding_bottom(spacing::SPACING_5) }
    fn pb_6(self) -> Self { self.padding_bottom(spacing::SPACING_6) }
    fn pb_8(self) -> Self { self.padding_bottom(spacing::SPACING_8) }

    fn pl_0(self) -> Self { self.padding_left(0.0) }
    fn pl_1(self) -> Self { self.padding_left(spacing::SPACING_1) }
    fn pl_2(self) -> Self { self.padding_left(spacing::SPACING_2) }
    fn pl_3(self) -> Self { self.padding_left(spacing::SPACING_3) }
    fn pl_4(self) -> Self { self.padding_left(spacing::SPACING_4) }
    fn pl_5(self) -> Self { self.padding_left(spacing::SPACING_5) }
    fn pl_6(self) -> Self { self.padding_left(spacing::SPACING_6) }
    fn pl_8(self) -> Self { self.padding_left(spacing::SPACING_8) }

    fn pr_0(self) -> Self { self.padding_right(0.0) }
    fn pr_1(self) -> Self { self.padding_right(spacing::SPACING_1) }
    fn pr_2(self) -> Self { self.padding_right(spacing::SPACING_2) }
    fn pr_3(self) -> Self { self.padding_right(spacing::SPACING_3) }
    fn pr_4(self) -> Self { self.padding_right(spacing::SPACING_4) }
    fn pr_5(self) -> Self { self.padding_right(spacing::SPACING_5) }
    fn pr_6(self) -> Self { self.padding_right(spacing::SPACING_6) }
    fn pr_8(self) -> Self { self.padding_right(spacing::SPACING_8) }

    // === Margin Implementations ===
    impl_margin_methods! {
        m_0 => 0.0,
        m_px => 1.0,
        m_0p5 => spacing::SPACING_0_5,
        m_1 => spacing::SPACING_1,
        m_1p5 => spacing::SPACING_1_5,
        m_2 => spacing::SPACING_2,
        m_2p5 => spacing::SPACING_2_5,
        m_3 => spacing::SPACING_3,
        m_3p5 => spacing::SPACING_3_5,
        m_4 => spacing::SPACING_4,
        m_5 => spacing::SPACING_5,
        m_6 => spacing::SPACING_6,
        m_7 => spacing::SPACING_7,
        m_8 => spacing::SPACING_8,
        m_9 => spacing::SPACING_9,
        m_10 => spacing::SPACING_10,
        m_11 => spacing::SPACING_11,
        m_12 => spacing::SPACING_12,
        m_14 => spacing::SPACING_14,
        m_16 => spacing::SPACING_16,
        m_20 => spacing::SPACING_20,
        m_24 => spacing::SPACING_24,
    }

    fn m_auto(self) -> Self { self.margin(PxPctAuto::Auto) }

    // Horizontal margin
    impl_margin_x_methods! {
        mx_0 => 0.0,
        mx_px => 1.0,
        mx_0p5 => spacing::SPACING_0_5,
        mx_1 => spacing::SPACING_1,
        mx_1p5 => spacing::SPACING_1_5,
        mx_2 => spacing::SPACING_2,
        mx_2p5 => spacing::SPACING_2_5,
        mx_3 => spacing::SPACING_3,
        mx_3p5 => spacing::SPACING_3_5,
        mx_4 => spacing::SPACING_4,
        mx_5 => spacing::SPACING_5,
        mx_6 => spacing::SPACING_6,
        mx_8 => spacing::SPACING_8,
        mx_10 => spacing::SPACING_10,
        mx_12 => spacing::SPACING_12,
        mx_16 => spacing::SPACING_16,
        mx_20 => spacing::SPACING_20,
        mx_24 => spacing::SPACING_24,
    }

    fn mx_auto(self) -> Self { self.margin_horiz(PxPctAuto::Auto) }

    // Vertical margin
    impl_margin_y_methods! {
        my_0 => 0.0,
        my_px => 1.0,
        my_0p5 => spacing::SPACING_0_5,
        my_1 => spacing::SPACING_1,
        my_1p5 => spacing::SPACING_1_5,
        my_2 => spacing::SPACING_2,
        my_2p5 => spacing::SPACING_2_5,
        my_3 => spacing::SPACING_3,
        my_3p5 => spacing::SPACING_3_5,
        my_4 => spacing::SPACING_4,
        my_5 => spacing::SPACING_5,
        my_6 => spacing::SPACING_6,
        my_8 => spacing::SPACING_8,
        my_10 => spacing::SPACING_10,
        my_12 => spacing::SPACING_12,
        my_16 => spacing::SPACING_16,
        my_20 => spacing::SPACING_20,
        my_24 => spacing::SPACING_24,
    }

    fn my_auto(self) -> Self { self.margin_vert(PxPctAuto::Auto) }

    // Individual margin sides
    fn mt_0(self) -> Self { self.margin_top(0.0) }
    fn mt_1(self) -> Self { self.margin_top(spacing::SPACING_1) }
    fn mt_2(self) -> Self { self.margin_top(spacing::SPACING_2) }
    fn mt_3(self) -> Self { self.margin_top(spacing::SPACING_3) }
    fn mt_4(self) -> Self { self.margin_top(spacing::SPACING_4) }
    fn mt_5(self) -> Self { self.margin_top(spacing::SPACING_5) }
    fn mt_6(self) -> Self { self.margin_top(spacing::SPACING_6) }
    fn mt_8(self) -> Self { self.margin_top(spacing::SPACING_8) }
    fn mt_auto(self) -> Self { self.margin_top(PxPctAuto::Auto) }

    fn mb_0(self) -> Self { self.margin_bottom(0.0) }
    fn mb_1(self) -> Self { self.margin_bottom(spacing::SPACING_1) }
    fn mb_2(self) -> Self { self.margin_bottom(spacing::SPACING_2) }
    fn mb_3(self) -> Self { self.margin_bottom(spacing::SPACING_3) }
    fn mb_4(self) -> Self { self.margin_bottom(spacing::SPACING_4) }
    fn mb_5(self) -> Self { self.margin_bottom(spacing::SPACING_5) }
    fn mb_6(self) -> Self { self.margin_bottom(spacing::SPACING_6) }
    fn mb_8(self) -> Self { self.margin_bottom(spacing::SPACING_8) }
    fn mb_auto(self) -> Self { self.margin_bottom(PxPctAuto::Auto) }

    fn ml_0(self) -> Self { self.margin_left(0.0) }
    fn ml_1(self) -> Self { self.margin_left(spacing::SPACING_1) }
    fn ml_2(self) -> Self { self.margin_left(spacing::SPACING_2) }
    fn ml_3(self) -> Self { self.margin_left(spacing::SPACING_3) }
    fn ml_4(self) -> Self { self.margin_left(spacing::SPACING_4) }
    fn ml_5(self) -> Self { self.margin_left(spacing::SPACING_5) }
    fn ml_6(self) -> Self { self.margin_left(spacing::SPACING_6) }
    fn ml_8(self) -> Self { self.margin_left(spacing::SPACING_8) }
    fn ml_auto(self) -> Self { self.margin_left(PxPctAuto::Auto) }

    fn mr_0(self) -> Self { self.margin_right(0.0) }
    fn mr_1(self) -> Self { self.margin_right(spacing::SPACING_1) }
    fn mr_2(self) -> Self { self.margin_right(spacing::SPACING_2) }
    fn mr_3(self) -> Self { self.margin_right(spacing::SPACING_3) }
    fn mr_4(self) -> Self { self.margin_right(spacing::SPACING_4) }
    fn mr_5(self) -> Self { self.margin_right(spacing::SPACING_5) }
    fn mr_6(self) -> Self { self.margin_right(spacing::SPACING_6) }
    fn mr_8(self) -> Self { self.margin_right(spacing::SPACING_8) }
    fn mr_auto(self) -> Self { self.margin_right(PxPctAuto::Auto) }

    // === Gap Implementations ===
    impl_gap_methods! {
        gap_0 => 0.0,
        gap_px => 1.0,
        gap_0p5 => spacing::SPACING_0_5,
        gap_1 => spacing::SPACING_1,
        gap_1p5 => spacing::SPACING_1_5,
        gap_2 => spacing::SPACING_2,
        gap_2p5 => spacing::SPACING_2_5,
        gap_3 => spacing::SPACING_3,
        gap_3p5 => spacing::SPACING_3_5,
        gap_4 => spacing::SPACING_4,
        gap_5 => spacing::SPACING_5,
        gap_6 => spacing::SPACING_6,
        gap_7 => spacing::SPACING_7,
        gap_8 => spacing::SPACING_8,
        gap_9 => spacing::SPACING_9,
        gap_10 => spacing::SPACING_10,
        gap_11 => spacing::SPACING_11,
        gap_12 => spacing::SPACING_12,
        gap_14 => spacing::SPACING_14,
        gap_16 => spacing::SPACING_16,
        gap_20 => spacing::SPACING_20,
        gap_24 => spacing::SPACING_24,
    }

    // === Border Radius Implementations ===
    impl_rounded_methods! {
        rounded_none => radius::ROUNDED_NONE,
        rounded_sm => radius::ROUNDED_SM,
        rounded => radius::ROUNDED,
        rounded_md => radius::ROUNDED_MD,
        rounded_lg => radius::ROUNDED_LG,
        rounded_xl => radius::ROUNDED_XL,
        rounded_2xl => radius::ROUNDED_2XL,
        rounded_3xl => radius::ROUNDED_3XL,
        rounded_full => radius::ROUNDED_FULL,
    }

    // === Border Width Implementations ===
    fn border_0(self) -> Self { self.border(0.0) }
    fn border_1(self) -> Self { self.border(1.0) }
    fn border_2(self) -> Self { self.border(2.0) }
    fn border_4(self) -> Self { self.border(4.0) }
    fn border_8(self) -> Self { self.border(8.0) }

    // === Shadow Implementations ===
    fn shadow_sm(self) -> Self { self.apply_box_shadows(vec![shadow::shadow_sm()]) }
    fn shadow(self) -> Self { self.apply_box_shadows(vec![shadow::shadow_default()]) }
    fn shadow_md(self) -> Self { self.apply_box_shadows(vec![shadow::shadow_md()]) }
    fn shadow_lg(self) -> Self { self.apply_box_shadows(vec![shadow::shadow_lg()]) }
    fn shadow_xl(self) -> Self { self.apply_box_shadows(vec![shadow::shadow_xl()]) }
    fn shadow_2xl(self) -> Self { self.apply_box_shadows(vec![shadow::shadow_2xl()]) }
    fn shadow_none(self) -> Self { self.apply_box_shadows(vec![]) }

    // === Background Color Implementations ===
    fn bg(self, color: impl Into<Color>) -> Self { self.background(color.into()) }
    fn bg_transparent(self) -> Self { self.background(colors::TRANSPARENT) }
    fn bg_black(self) -> Self { self.background(colors::BLACK) }
    fn bg_white(self) -> Self { self.background(colors::WHITE) }
    // Slate
    fn bg_slate_50(self) -> Self { self.background(colors::slate::C50) }
    fn bg_slate_100(self) -> Self { self.background(colors::slate::C100) }
    fn bg_slate_200(self) -> Self { self.background(colors::slate::C200) }
    fn bg_slate_300(self) -> Self { self.background(colors::slate::C300) }
    fn bg_slate_400(self) -> Self { self.background(colors::slate::C400) }
    fn bg_slate_500(self) -> Self { self.background(colors::slate::C500) }
    fn bg_slate_600(self) -> Self { self.background(colors::slate::C600) }
    fn bg_slate_700(self) -> Self { self.background(colors::slate::C700) }
    fn bg_slate_800(self) -> Self { self.background(colors::slate::C800) }
    fn bg_slate_900(self) -> Self { self.background(colors::slate::C900) }
    fn bg_slate_950(self) -> Self { self.background(colors::slate::C950) }
    // Gray
    fn bg_gray_50(self) -> Self { self.background(colors::gray::C50) }
    fn bg_gray_100(self) -> Self { self.background(colors::gray::C100) }
    fn bg_gray_200(self) -> Self { self.background(colors::gray::C200) }
    fn bg_gray_300(self) -> Self { self.background(colors::gray::C300) }
    fn bg_gray_400(self) -> Self { self.background(colors::gray::C400) }
    fn bg_gray_500(self) -> Self { self.background(colors::gray::C500) }
    fn bg_gray_600(self) -> Self { self.background(colors::gray::C600) }
    fn bg_gray_700(self) -> Self { self.background(colors::gray::C700) }
    fn bg_gray_800(self) -> Self { self.background(colors::gray::C800) }
    fn bg_gray_900(self) -> Self { self.background(colors::gray::C900) }
    fn bg_gray_950(self) -> Self { self.background(colors::gray::C950) }
    // Zinc
    fn bg_zinc_50(self) -> Self { self.background(colors::zinc::C50) }
    fn bg_zinc_100(self) -> Self { self.background(colors::zinc::C100) }
    fn bg_zinc_200(self) -> Self { self.background(colors::zinc::C200) }
    fn bg_zinc_300(self) -> Self { self.background(colors::zinc::C300) }
    fn bg_zinc_400(self) -> Self { self.background(colors::zinc::C400) }
    fn bg_zinc_500(self) -> Self { self.background(colors::zinc::C500) }
    fn bg_zinc_600(self) -> Self { self.background(colors::zinc::C600) }
    fn bg_zinc_700(self) -> Self { self.background(colors::zinc::C700) }
    fn bg_zinc_800(self) -> Self { self.background(colors::zinc::C800) }
    fn bg_zinc_900(self) -> Self { self.background(colors::zinc::C900) }
    fn bg_zinc_950(self) -> Self { self.background(colors::zinc::C950) }
    // Red
    fn bg_red_50(self) -> Self { self.background(colors::red::C50) }
    fn bg_red_100(self) -> Self { self.background(colors::red::C100) }
    fn bg_red_200(self) -> Self { self.background(colors::red::C200) }
    fn bg_red_300(self) -> Self { self.background(colors::red::C300) }
    fn bg_red_400(self) -> Self { self.background(colors::red::C400) }
    fn bg_red_500(self) -> Self { self.background(colors::red::C500) }
    fn bg_red_600(self) -> Self { self.background(colors::red::C600) }
    fn bg_red_700(self) -> Self { self.background(colors::red::C700) }
    fn bg_red_800(self) -> Self { self.background(colors::red::C800) }
    fn bg_red_900(self) -> Self { self.background(colors::red::C900) }
    fn bg_red_950(self) -> Self { self.background(colors::red::C950) }
    // Orange
    fn bg_orange_50(self) -> Self { self.background(colors::orange::C50) }
    fn bg_orange_100(self) -> Self { self.background(colors::orange::C100) }
    fn bg_orange_200(self) -> Self { self.background(colors::orange::C200) }
    fn bg_orange_300(self) -> Self { self.background(colors::orange::C300) }
    fn bg_orange_400(self) -> Self { self.background(colors::orange::C400) }
    fn bg_orange_500(self) -> Self { self.background(colors::orange::C500) }
    fn bg_orange_600(self) -> Self { self.background(colors::orange::C600) }
    fn bg_orange_700(self) -> Self { self.background(colors::orange::C700) }
    fn bg_orange_800(self) -> Self { self.background(colors::orange::C800) }
    fn bg_orange_900(self) -> Self { self.background(colors::orange::C900) }
    fn bg_orange_950(self) -> Self { self.background(colors::orange::C950) }
    // Yellow
    fn bg_yellow_50(self) -> Self { self.background(colors::yellow::C50) }
    fn bg_yellow_100(self) -> Self { self.background(colors::yellow::C100) }
    fn bg_yellow_200(self) -> Self { self.background(colors::yellow::C200) }
    fn bg_yellow_300(self) -> Self { self.background(colors::yellow::C300) }
    fn bg_yellow_400(self) -> Self { self.background(colors::yellow::C400) }
    fn bg_yellow_500(self) -> Self { self.background(colors::yellow::C500) }
    fn bg_yellow_600(self) -> Self { self.background(colors::yellow::C600) }
    fn bg_yellow_700(self) -> Self { self.background(colors::yellow::C700) }
    fn bg_yellow_800(self) -> Self { self.background(colors::yellow::C800) }
    fn bg_yellow_900(self) -> Self { self.background(colors::yellow::C900) }
    fn bg_yellow_950(self) -> Self { self.background(colors::yellow::C950) }
    // Green
    fn bg_green_50(self) -> Self { self.background(colors::green::C50) }
    fn bg_green_100(self) -> Self { self.background(colors::green::C100) }
    fn bg_green_200(self) -> Self { self.background(colors::green::C200) }
    fn bg_green_300(self) -> Self { self.background(colors::green::C300) }
    fn bg_green_400(self) -> Self { self.background(colors::green::C400) }
    fn bg_green_500(self) -> Self { self.background(colors::green::C500) }
    fn bg_green_600(self) -> Self { self.background(colors::green::C600) }
    fn bg_green_700(self) -> Self { self.background(colors::green::C700) }
    fn bg_green_800(self) -> Self { self.background(colors::green::C800) }
    fn bg_green_900(self) -> Self { self.background(colors::green::C900) }
    fn bg_green_950(self) -> Self { self.background(colors::green::C950) }
    // Blue
    fn bg_blue_50(self) -> Self { self.background(colors::blue::C50) }
    fn bg_blue_100(self) -> Self { self.background(colors::blue::C100) }
    fn bg_blue_200(self) -> Self { self.background(colors::blue::C200) }
    fn bg_blue_300(self) -> Self { self.background(colors::blue::C300) }
    fn bg_blue_400(self) -> Self { self.background(colors::blue::C400) }
    fn bg_blue_500(self) -> Self { self.background(colors::blue::C500) }
    fn bg_blue_600(self) -> Self { self.background(colors::blue::C600) }
    fn bg_blue_700(self) -> Self { self.background(colors::blue::C700) }
    fn bg_blue_800(self) -> Self { self.background(colors::blue::C800) }
    fn bg_blue_900(self) -> Self { self.background(colors::blue::C900) }
    fn bg_blue_950(self) -> Self { self.background(colors::blue::C950) }
    // Indigo
    fn bg_indigo_50(self) -> Self { self.background(colors::indigo::C50) }
    fn bg_indigo_100(self) -> Self { self.background(colors::indigo::C100) }
    fn bg_indigo_200(self) -> Self { self.background(colors::indigo::C200) }
    fn bg_indigo_300(self) -> Self { self.background(colors::indigo::C300) }
    fn bg_indigo_400(self) -> Self { self.background(colors::indigo::C400) }
    fn bg_indigo_500(self) -> Self { self.background(colors::indigo::C500) }
    fn bg_indigo_600(self) -> Self { self.background(colors::indigo::C600) }
    fn bg_indigo_700(self) -> Self { self.background(colors::indigo::C700) }
    fn bg_indigo_800(self) -> Self { self.background(colors::indigo::C800) }
    fn bg_indigo_900(self) -> Self { self.background(colors::indigo::C900) }
    fn bg_indigo_950(self) -> Self { self.background(colors::indigo::C950) }
    // Purple
    fn bg_purple_50(self) -> Self { self.background(colors::purple::C50) }
    fn bg_purple_100(self) -> Self { self.background(colors::purple::C100) }
    fn bg_purple_200(self) -> Self { self.background(colors::purple::C200) }
    fn bg_purple_300(self) -> Self { self.background(colors::purple::C300) }
    fn bg_purple_400(self) -> Self { self.background(colors::purple::C400) }
    fn bg_purple_500(self) -> Self { self.background(colors::purple::C500) }
    fn bg_purple_600(self) -> Self { self.background(colors::purple::C600) }
    fn bg_purple_700(self) -> Self { self.background(colors::purple::C700) }
    fn bg_purple_800(self) -> Self { self.background(colors::purple::C800) }
    fn bg_purple_900(self) -> Self { self.background(colors::purple::C900) }
    fn bg_purple_950(self) -> Self { self.background(colors::purple::C950) }
    // Pink
    fn bg_pink_50(self) -> Self { self.background(colors::pink::C50) }
    fn bg_pink_100(self) -> Self { self.background(colors::pink::C100) }
    fn bg_pink_200(self) -> Self { self.background(colors::pink::C200) }
    fn bg_pink_300(self) -> Self { self.background(colors::pink::C300) }
    fn bg_pink_400(self) -> Self { self.background(colors::pink::C400) }
    fn bg_pink_500(self) -> Self { self.background(colors::pink::C500) }
    fn bg_pink_600(self) -> Self { self.background(colors::pink::C600) }
    fn bg_pink_700(self) -> Self { self.background(colors::pink::C700) }
    fn bg_pink_800(self) -> Self { self.background(colors::pink::C800) }
    fn bg_pink_900(self) -> Self { self.background(colors::pink::C900) }
    fn bg_pink_950(self) -> Self { self.background(colors::pink::C950) }

    // === Text Color Implementations ===
    fn text(self, color: impl Into<Color>) -> Self { self.color(color.into()) }
    fn text_transparent(self) -> Self { self.color(colors::TRANSPARENT) }
    fn text_black(self) -> Self { self.color(colors::BLACK) }
    fn text_white(self) -> Self { self.color(colors::WHITE) }
    // Slate
    fn text_slate_50(self) -> Self { self.color(colors::slate::C50) }
    fn text_slate_100(self) -> Self { self.color(colors::slate::C100) }
    fn text_slate_200(self) -> Self { self.color(colors::slate::C200) }
    fn text_slate_300(self) -> Self { self.color(colors::slate::C300) }
    fn text_slate_400(self) -> Self { self.color(colors::slate::C400) }
    fn text_slate_500(self) -> Self { self.color(colors::slate::C500) }
    fn text_slate_600(self) -> Self { self.color(colors::slate::C600) }
    fn text_slate_700(self) -> Self { self.color(colors::slate::C700) }
    fn text_slate_800(self) -> Self { self.color(colors::slate::C800) }
    fn text_slate_900(self) -> Self { self.color(colors::slate::C900) }
    fn text_slate_950(self) -> Self { self.color(colors::slate::C950) }
    // Gray
    fn text_gray_50(self) -> Self { self.color(colors::gray::C50) }
    fn text_gray_100(self) -> Self { self.color(colors::gray::C100) }
    fn text_gray_200(self) -> Self { self.color(colors::gray::C200) }
    fn text_gray_300(self) -> Self { self.color(colors::gray::C300) }
    fn text_gray_400(self) -> Self { self.color(colors::gray::C400) }
    fn text_gray_500(self) -> Self { self.color(colors::gray::C500) }
    fn text_gray_600(self) -> Self { self.color(colors::gray::C600) }
    fn text_gray_700(self) -> Self { self.color(colors::gray::C700) }
    fn text_gray_800(self) -> Self { self.color(colors::gray::C800) }
    fn text_gray_900(self) -> Self { self.color(colors::gray::C900) }
    fn text_gray_950(self) -> Self { self.color(colors::gray::C950) }
    // Red
    fn text_red_50(self) -> Self { self.color(colors::red::C50) }
    fn text_red_100(self) -> Self { self.color(colors::red::C100) }
    fn text_red_200(self) -> Self { self.color(colors::red::C200) }
    fn text_red_300(self) -> Self { self.color(colors::red::C300) }
    fn text_red_400(self) -> Self { self.color(colors::red::C400) }
    fn text_red_500(self) -> Self { self.color(colors::red::C500) }
    fn text_red_600(self) -> Self { self.color(colors::red::C600) }
    fn text_red_700(self) -> Self { self.color(colors::red::C700) }
    fn text_red_800(self) -> Self { self.color(colors::red::C800) }
    fn text_red_900(self) -> Self { self.color(colors::red::C900) }
    fn text_red_950(self) -> Self { self.color(colors::red::C950) }
    // Green
    fn text_green_50(self) -> Self { self.color(colors::green::C50) }
    fn text_green_100(self) -> Self { self.color(colors::green::C100) }
    fn text_green_200(self) -> Self { self.color(colors::green::C200) }
    fn text_green_300(self) -> Self { self.color(colors::green::C300) }
    fn text_green_400(self) -> Self { self.color(colors::green::C400) }
    fn text_green_500(self) -> Self { self.color(colors::green::C500) }
    fn text_green_600(self) -> Self { self.color(colors::green::C600) }
    fn text_green_700(self) -> Self { self.color(colors::green::C700) }
    fn text_green_800(self) -> Self { self.color(colors::green::C800) }
    fn text_green_900(self) -> Self { self.color(colors::green::C900) }
    fn text_green_950(self) -> Self { self.color(colors::green::C950) }
    // Blue
    fn text_blue_50(self) -> Self { self.color(colors::blue::C50) }
    fn text_blue_100(self) -> Self { self.color(colors::blue::C100) }
    fn text_blue_200(self) -> Self { self.color(colors::blue::C200) }
    fn text_blue_300(self) -> Self { self.color(colors::blue::C300) }
    fn text_blue_400(self) -> Self { self.color(colors::blue::C400) }
    fn text_blue_500(self) -> Self { self.color(colors::blue::C500) }
    fn text_blue_600(self) -> Self { self.color(colors::blue::C600) }
    fn text_blue_700(self) -> Self { self.color(colors::blue::C700) }
    fn text_blue_800(self) -> Self { self.color(colors::blue::C800) }
    fn text_blue_900(self) -> Self { self.color(colors::blue::C900) }
    fn text_blue_950(self) -> Self { self.color(colors::blue::C950) }

    // === Font Size Implementations ===
    impl_font_size_methods! {
        text_xs => font_size::TEXT_XS,
        text_sm => font_size::TEXT_SM,
        text_base => font_size::TEXT_BASE,
        text_lg => font_size::TEXT_LG,
        text_xl => font_size::TEXT_XL,
        text_2xl => font_size::TEXT_2XL,
        text_3xl => font_size::TEXT_3XL,
        text_4xl => font_size::TEXT_4XL,
        text_5xl => font_size::TEXT_5XL,
        text_6xl => font_size::TEXT_6XL,
        text_7xl => font_size::TEXT_7XL,
        text_8xl => font_size::TEXT_8XL,
        text_9xl => font_size::TEXT_9XL,
    }

    // === Font Weight Implementations ===
    impl_font_weight_methods! {
        font_thin => font_weight::THIN,
        font_extralight => font_weight::EXTRALIGHT,
        font_light => font_weight::LIGHT,
        font_normal => font_weight::NORMAL,
        font_medium => font_weight::MEDIUM,
        font_semibold => font_weight::SEMIBOLD,
        font_bold => font_weight::BOLD,
        font_extrabold => font_weight::EXTRABOLD,
        font_black => font_weight::BLACK,
    }

    // === Line Height Implementations ===
    impl_line_height_methods! {
        leading_none => line_height::LEADING_NONE,
        leading_tight => line_height::LEADING_TIGHT,
        leading_snug => line_height::LEADING_SNUG,
        leading_normal => line_height::LEADING_NORMAL,
        leading_relaxed => line_height::LEADING_RELAXED,
        leading_loose => line_height::LEADING_LOOSE,
    }

    // === Display Implementations ===
    fn flex(self) -> Self { self.display(floem::style::Display::Flex) }
    fn block(self) -> Self { self.display(floem::style::Display::Block) }
    fn grid(self) -> Self { self.display(floem::style::Display::Grid) }
    fn hidden(self) -> Self { self.display(floem::style::Display::None) }

    // === Flex Direction Implementations ===
    fn flex_row(self) -> Self { self.flex_direction(floem::style::FlexDirection::Row) }
    fn flex_col(self) -> Self { self.flex_direction(floem::style::FlexDirection::Column) }
    fn flex_row_reverse(self) -> Self { self.flex_direction(floem::style::FlexDirection::RowReverse) }
    fn flex_col_reverse(self) -> Self { self.flex_direction(floem::style::FlexDirection::ColumnReverse) }

    // === Flex Wrap Implementations ===
    fn wrap(self) -> Self { self.flex_wrap(floem::style::FlexWrap::Wrap) }
    fn nowrap(self) -> Self { self.flex_wrap(floem::style::FlexWrap::NoWrap) }
    fn wrap_reverse(self) -> Self { self.flex_wrap(floem::style::FlexWrap::WrapReverse) }

    // === Cursor Implementations ===
    fn cursor_pointer(self) -> Self { self.cursor(floem::style::CursorStyle::Pointer) }
    fn cursor_default(self) -> Self { self.cursor(floem::style::CursorStyle::Default) }
    fn cursor_text(self) -> Self { self.cursor(floem::style::CursorStyle::Text) }
    fn cursor_move(self) -> Self { self.cursor(floem::style::CursorStyle::Move) }
    fn cursor_grab(self) -> Self { self.cursor(floem::style::CursorStyle::Grab) }
    fn cursor_grabbing(self) -> Self { self.cursor(floem::style::CursorStyle::Grabbing) }

    // === Border Color Implementations ===
    fn border_transparent(self) -> Self { self.border_color(colors::TRANSPARENT) }
    fn border_black(self) -> Self { self.border_color(colors::BLACK) }
    fn border_white(self) -> Self { self.border_color(colors::WHITE) }
    fn border_gray_200(self) -> Self { self.border_color(colors::gray::C200) }
    fn border_gray_300(self) -> Self { self.border_color(colors::gray::C300) }
    fn border_gray_400(self) -> Self { self.border_color(colors::gray::C400) }
    fn border_gray_500(self) -> Self { self.border_color(colors::gray::C500) }
    fn border_gray_600(self) -> Self { self.border_color(colors::gray::C600) }
    fn border_red_500(self) -> Self { self.border_color(colors::red::C500) }
    fn border_blue_500(self) -> Self { self.border_color(colors::blue::C500) }
    fn border_green_500(self) -> Self { self.border_color(colors::green::C500) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_values() {
        assert_eq!(spacing::SPACING_1, 4.0);
        assert_eq!(spacing::SPACING_2, 8.0);
        assert_eq!(spacing::SPACING_4, 16.0);
        assert_eq!(spacing::SPACING_8, 32.0);
        assert_eq!(spacing::SPACING_16, 64.0);
    }

    #[test]
    fn test_size_values() {
        assert_eq!(spacing::SIZE_SM, 384.0);
        assert_eq!(spacing::SIZE_MD, 448.0);
        assert_eq!(spacing::SIZE_LG, 512.0);
    }

    #[test]
    fn test_radius_values() {
        assert_eq!(radius::ROUNDED_SM, 2.0);
        assert_eq!(radius::ROUNDED_MD, 6.0);
        assert_eq!(radius::ROUNDED_LG, 8.0);
    }
}
