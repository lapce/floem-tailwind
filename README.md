# floem-tailwind

**Tailwind‑style utility methods for the [Floem](https://github.com/lapce/floem) GUI framework.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

`floem-tailwind` extends Floem’s `Style` type with hundreds of chainable methods that mirror the names and values of Tailwind CSS classes. Use your existing Tailwind knowledge to style Floem views with zero ceremony.

---

## ✨ Features

- **Full flexbox helpers** – direction, wrap, grow, shrink, basis, justify, align
- **Layout primitives** – display, position, z‑index, inset
- **Complete colour system** – all 22 Tailwind colour families (bg, text, border)
- **Spacing scale** – padding, margin, gap, inset (0‑96, including fractions)
- **Typography** – font size, line height, numeric font sizes
- **Borders** – width per side, colour per side
- **Effects** – opacity, box shadow
- **Interactivity** – cursor, user‑select, pointer events
- **DRY macro** for integration tests covering every method

---

## 📦 Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
floem-tailwind = { git = "https://github.com/your-username/floem-tailwind" }
```

> **Note:** Requires **Rust nightly** and the Floem dependency as specified.

---

## 🚀 Quick Start

```rust
use floem::prelude::*;
use floem_tailwind::*;  // brings all extension traits into scope

fn app_view() -> impl IntoView {
    stack::vertical((
        button("Click me")
            .style(|s| s.bg_blue_500().text_white().px_4().py_2().rounded_md()),
        label("Hello, Tailwind!").style(|s| s.text_gray_700().text_2xl()),
    ))
    .style(|s| s.gap_4().p_8().bg_gray_100())
}
```

---

## 🧰 Supported Utility Modules

| Module | Trait | Description |
|--------|--------|-------------|
| `colors` | (built‑in) | 22 colour palettes (bg/text/border) + special colours |
| `spacing` | (built‑in) | Spacing scale (p/m/gap/w/h/… ) |
| `flexbox` | `TailwindFlexboxExt` | Flex direction, wrap, grow, shrink, basis (full scale), justify, align |
| `layout` | `TailwindLayoutExt` | Display, position, z‑index, basic inset, text overflow |
| `interactivity` | `TailwindInteractivityExt` | Cursor, user‑select, pointer events |
| `gap` | `TailwindGapExt` | Column‑gap and row‑gap (0‑96 scale) |
| `border_side` | `TailwindBorderSideExt` | Per‑side border widths (0‑8) |
| `opacity` | `TailwindOpacityExt` | Opacity (0‑100 in steps of 5) |
| `special_colors` | `TailwindSpecialColorExt` | White, black, transparent (bg/text/border) |
| `leading` | `TailwindLeadingExt` | Line‑height utilities |
| `inset` | `TailwindInsetExt` | Top/right/bottom/left/inset (0‑96 scale) |
| `min_max_height` | `TailwindMinMaxHeightExt` | Min‑height and max‑height (0‑96 scale) |
| `number_font_size` | `TailwindNumberFontSizeExt` | Numeric font sizes (12‑96) |
| `colors_ext` | `TailwindColorExt` | 16 additional colour families (all shades) |
| `font_size` | (built‑in) | Named font sizes (xs‑9xl) |
| `font_weight` | (built‑in) | Font weights (thin‑black) |
| `line_height` | (built‑in) | Line heights (3‑10, none‑loose) |
| `shadow` | (built‑in) | Box shadows (sm‑2xl, none) |

### Colour families (all available as `bg_*`, `text_*`, `border_*`)

slate, gray, zinc, neutral, stone, red, orange, amber, yellow, lime, green, emerald, teal, cyan, sky, blue, indigo, violet, purple, fuchsia, pink, rose

Each family includes shades `50`‑`950`.

---

## 🧪 Testing

The integration test file `tests/utilities.rs` uses a macro to verify that every utility method compiles and can be invoked. Run:

```bash
just test
```

All 10+ test suites must pass before committing.

---

## 📝 License

MIT

---

## 🤝 Contributing

Contributions are welcome! Please open an issue or PR on the repository.
