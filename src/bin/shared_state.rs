use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running shared state example");
    launch(App);
}

#[derive(Clone, PartialEq)]
struct ChildState {
    value: String,
}

struct Children(Vec<ChildState>);

#[component]
pub fn App() -> Element {
    tracing::info!("rendering app");
    let children = use_context_provider(|| {
        Signal::new(Children(
            (0..100)
                .map(|x| ChildState {
                    value: x.to_string(),
                })
                .collect(),
        ))
    });
    rsx! {
        button {
            r#type: "button",
            onclick: move |_| edit_10_children(children),
            "edit 10 children"
        }
        ul {{
            children
                .read()
                .0
                .iter()
                .map(|child| rsx!(Child {
                    key: "{child.value}",
                    state: child.clone() ,
                }))
        }}
    }
}

#[component]
fn Child(state: ChildState) -> Element {
    tracing::info!("rendering child");
    let children = use_context::<Signal<Children>>();
    rsx! {
        li {
            {state.value}
            button {
                r#type: "button",
                onclick: move |_| edit_10_children(children),
                "edit"
            }
        }
    }
}

async fn edit_10_children(mut children: Signal<Children>) {
    let mut children = children.write();
    for i in 0..10 {
        children.0[i].value = format!("edited {}", children.0[i].value);
    }
}
