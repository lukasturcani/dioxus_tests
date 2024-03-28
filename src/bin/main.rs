use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { "hi" }
    }
}
