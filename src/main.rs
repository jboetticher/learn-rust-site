use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        h1 { "Learn Rust" }
        h4 { "Use the best app for learning Rust out there." }
        p { "This website was made with Rust via the Sycamore package." }
    });
}