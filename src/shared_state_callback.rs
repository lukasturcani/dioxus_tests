use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ChildState {
    value: String,
}

struct Children(Vec<ChildState>);

#[component]
pub fn App() -> Element {
    log::info!("rendering app");
    let children = use_signal(|| {
        Children(
            (0..100)
                .map(|i| ChildState {
                    value: format!("child {}", i),
                })
                .collect(),
        )
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
                    onclick: move |_| {
                        spawn(edit_10_children(children));
                    },
                }
            }
        }
    }
}

#[component]
fn Child(state: ChildState, onclick: EventHandler) -> Element {
    log::info!("rendering child");
    rsx! {
        li {
            "{state.value}"
            button {
                r#type: "button",
                onclick: move |_| onclick.call(()),
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
