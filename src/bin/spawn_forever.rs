use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running spawn forever example");
    launch(App);
}

#[component]
fn App() -> Element {
    let child1 = use_signal(|| true);
    let child2 = use_signal(|| true);
    let child3 = use_signal(|| true);
    rsx! {
        if child1() {
            Child { exists: child1 }
        }
        if child2() {
            Child { exists: child2 }
        }
        if child3() {
            Child { exists: child3 }
        }
    }
}

#[component]
fn Child(exists: Signal<bool>) -> Element {
    rsx! {
        button {
            onclick: move |_| {
                spawn_forever(print_stuff());
                exists.set(false)
            },
            "remove me"
        }
    }
}

async fn print_stuff() {
    tracing::info!("printing stuff");
    for i in 0..10 {
        tracing::info!("i: {}", i);
    }
}
