use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ChildState {
    value: String,
}

struct Children(Vec<ChildState>);

#[component]
pub fn App() -> Element {
    log::info!("rendering app");
    let children = use_context_provider(|| {
        Signal::new(Children(
            (0..100)
                .map(|i| ChildState {
                    value: format!("child {}", i),
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
        ul {
            for state in children.read().0.iter() {
                Child {
                    key: "{state.value}",
                    state: state.clone(),
                }
            }
        }
    }
}

#[component]
fn Child(state: ChildState) -> Element {
    log::info!("rendering child");
    let children = consume_context::<Signal<Children>>();
    rsx! {
        li {
            "{state.value}"
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
