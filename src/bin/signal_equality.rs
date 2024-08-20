use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running signal equality example");
    launch(App);
}

#[component]
fn App() -> Element {
    tracing::info!("rendering app");
    let mut vec1 = use_signal(|| vec![0]);
    let mut vec2 = use_signal(|| vec![0]);
    rsx! {
        button {
            onclick: move |_| {
                let mut vec1 = vec1.write();
                vec1.push(1);
            },
            "vec1 {vec1:?}"
        }
        button {
            onclick: move |_| {
                let mut vec2 = vec2.write();
                vec2.push(1);
            },
            "vec2: vec2: {vec2:?}"
        }
        p { "vec1 vec1: {vec1 == vec1}" }
        p { "vec2 vec2: {vec2 == vec2}" }
        p { "vec1 vec2: {vec1 == vec2}" }
    }
}
