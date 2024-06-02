use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running borrowed props example");
    launch(App);
}

#[component]
fn App() -> Element {
    tracing::info!("rendering app");
    let mut counter = use_signal(|| 0);
    rsx! {
        button {
            onclick: move |_| counter.set(counter()+1),
            "count"
        }
        p { "counter: {counter()}" }
        Child { counter, value: "hello" },
        Child { counter, value: "world" },
    }
}

#[component]
fn Child(value: &'static str, counter: Signal<i32>) -> Element {
    tracing::info!("rendering child");
    rsx! {

        p {
            onclick: move |_| counter.set(counter()+1),
            "{value}"
        }
    }
}
