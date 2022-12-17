use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        h1 { "The best app to learn Rust." }
    });
}