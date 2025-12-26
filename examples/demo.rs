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

    v_stack((
        // Header
        label(|| "floem-tailwind Demo").style(|s| s.text_slate_900().font_size(24.0).mb_4()),
        // Counter section with Tailwind-style utilities
        h_stack((
            button("Increment").action(move || counter += 1).style(|s| {
                s.bg_blue_500()
                    .text_white()
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .hover(|s| s.bg_blue_600())
            }),
            label(move || format!("Count: {counter}")).style(|s| s.text_gray_700().px_4()),
            button("Decrement").action(move || counter -= 1).style(|s| {
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
        v_stack((
            label(|| "Card Title").style(|s| s.text_gray_900().font_size(18.0)),
            label(|| "This is a card styled with Tailwind-like utilities.")
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
        h_stack((
            empty().style(|s| s.size_8().bg_red_500().rounded()),
            empty().style(|s| s.size_8().bg_orange_500().rounded()),
            empty().style(|s| s.size_8().bg_yellow_500().rounded()),
            empty().style(|s| s.size_8().bg_green_500().rounded()),
            empty().style(|s| s.size_8().bg_blue_500().rounded()),
            empty().style(|s| s.size_8().bg_indigo_500().rounded()),
            empty().style(|s| s.size_8().bg_purple_500().rounded()),
            empty().style(|s| s.size_8().bg_pink_500().rounded()),
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
