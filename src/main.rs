use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            Container(class="greenBack") {
                h1 { "Learn Rust" }
                h4 { "Use the best app for learning Rust out there." }
            }
            CurvySVG {}
            Container(class="orangeBack") {
                p { "This website was made by Jeremy Boetticher with Rust via the Sycamore package." }
                p { "\"I still don't know how lifetimes work.\" -Jeremy" }
            }
            img(class="title-app-image", src="https://cdn.discordapp.com/attachments/617838827980980225/1053535559139737701/rust-app-title.png")
        }
    });
}

#[derive(Prop)]
pub struct ContainerProps<'a, G: Html> {
    children: Children<'a, G>,
    class: &'a str,
}

#[component]
fn Container<'a, G: Html>(cx: Scope<'a>, props: ContainerProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    let mut baseClass = String::from("container ");
    baseClass.push_str(props.class);

    view! { cx,
        div(class=baseClass) {
            (children)
        }
    }
}

#[component]
fn CurvySVG<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        svg(viewBox="0 0 1440 320", xmlns="http://www.w3.org/2000/svg") {
            path(
                fill="#0b7261", fill-opacity="1",
                d="M0,256L34.3,266.7C68.6,277,137,299,206,266.7C274.3,235,343,149,411,128C480,107,549,149,617,138.7C685.7,128,754,64,823,42.7C891.4,21,960,43,1029,80C1097.1,117,1166,171,1234,170.7C1302.9,171,1371,117,1406,90.7L1440,64L1440,0L1405.7,0C1371.4,0,1303,0,1234,0C1165.7,0,1097,0,1029,0C960,0,891,0,823,0C754.3,0,686,0,617,0C548.6,0,480,0,411,0C342.9,0,274,0,206,0C137.1,0,69,0,34,0L0,0Z"            )
        }
    }
}
