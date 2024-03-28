use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("running shared state callback example");
    launch(App);
}

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
                .map(|x| ChildState {
                    value: x.to_string(),
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
        ul {{
            children
                .read()
                .0
                .iter()
                .map(|child| rsx!(Child {
                    key: "{child.value}",
                    state: child.clone(),
                    onclick: move |_|  {
                        spawn(edit_10_children(children));
                    },
                }))
        }}
    }
}

#[component]
fn Child(state: ChildState, onclick: EventHandler) -> Element {
    log::info!("rendering child");
    rsx! {
        li {
            {state.value}
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
