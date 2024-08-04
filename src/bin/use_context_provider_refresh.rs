use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running use context provider refresh example");
    launch(App);
}

#[derive(Clone, PartialEq, Eq)]
struct Model(i32);

#[derive(Clone, PartialEq, Eq)]
struct OtherModel(i32);

#[component]
fn App() -> Element {
    let model = use_context_provider(|| {
        tracing::info!("creating model");
        Signal::new(Model(0))
    });
    let k = model.read().0;
    tracing::info!("k: {}", k);
    let other_model = Signal::new(OtherModel(k + 2));
    rsx! {
        Child { n: 1, other_model }
        Child { n: 2, other_model }
    }
}

#[component]
fn Child(n: i32, other_model: Signal<OtherModel>) -> Element {
    let mut model = use_context::<Signal<Model>>();
    model.read();
    tracing::info!("rendering child {}", n);
    let this = other_model.read().0 + n;
    rsx! {
        button {
            onclick: move |_| model.set(Model(n)),
            "{this}"
        }
    }
}
