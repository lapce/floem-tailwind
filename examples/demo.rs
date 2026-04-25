//! Demo example showcasing floem-tailwind utilities
//!
//! Run with: cargo run --example demo

use floem::prelude::*;
use floem_tailwind::TailwindExt;

fn main() {
    floem::launch(app_view);
}

fn app_view() -> impl IntoView {
    let mut counter = RwSignal::new(0);

    Stack::vertical((
        // Header
        Label::derived(|| "floem-tailwind Demo").style(|s| s.text_slate_900().font_size(24.0).mb_4()),
        // Counter section with Tailwind-style utilities
        Stack::horizontal((
            Button::new("Increment").action(move || counter += 1).style(|s| {
                s.bg_blue_500()
                    .text_white()
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .hover(|s| s.bg_blue_600())
            }),
            Label::derived(move || format!("Count: {counter}")).style(|s| s.text_gray_700().px_4()),
            Button::new("Decrement").action(move || counter -= 1).style(|s| {
                s.bg_red_500()
                    .text_white()
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .hover(|s| s.bg_red_600())
            }),
        ))
        .style(|s| s.gap_2().items_center()),
        // Card example
        Stack::vertical((
            Label::derived(|| "Card Title").style(|s| s.text_gray_900().font_size(18.0)),
            Label::derived(|| "This is a card styled with Tailwind-like utilities.")
                .style(|s| s.text_gray_600()),
        ))
        .style(|s| {
            s.mt_8()
                .p_4()
                .bg_white()
                .rounded_lg()
                .border_1()
                .border_gray_200()
                .w_md()
        }),
        // Color palette preview
        Stack::horizontal((
            Empty::new().style(|s| s.size_8().bg_red_500().rounded()),
            Empty::new().style(|s| s.size_8().bg_orange_500().rounded()),
            Empty::new().style(|s| s.size_8().bg_yellow_500().rounded()),
            Empty::new().style(|s| s.size_8().bg_green_500().rounded()),
            Empty::new().style(|s| s.size_8().bg_blue_500().rounded()),
            Empty::new().style(|s| s.size_8().bg_indigo_500().rounded()),
            Empty::new().style(|s| s.size_8().bg_purple_500().rounded()),
            Empty::new().style(|s| s.size_8().bg_pink_500().rounded()),
        ))
        .style(|s| s.mt_8().gap_2()),
    ))
    .style(|s| {
        s.w_full()
            .h_full()
            .p_8()
            .bg_gray_100()
            .items_center()
            .justify_center()
    })
}
