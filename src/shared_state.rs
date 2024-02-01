use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct ChildState {
    value: String,
}

struct Children(Vec<ChildState>);

#[component]
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Children(Vec::new()));
    let children = use_shared_state::<Children>(cx).unwrap();
    use_future(cx, (), |_| {
        let children = children.clone();
        async move {
            let mut children = children.write();
            children.0.extend((0..100).map(|x| ChildState {
                value: x.to_string(),
            }));
        }
    });
    cx.render(rsx! {
        button {
            r#type: "button",
            onclick: |_| edit_10_children(children.clone()),
            "edit 10 children"
        }
        ul {
            children
                .read()
                .0
                .iter()
                .map(|child| rsx!(Child {
                    key: "{child.value}",
                    state: child.clone() ,
                }))
        }
    })
}

#[component]
fn Child(cx: Scope, state: ChildState) -> Element {
    log::info!("rendering child");
    let children = use_shared_state::<Children>(cx).unwrap();
    cx.render(rsx! {
        li {
            state.value.clone()
            button {
                r#type: "button",
                onclick: |_| edit_10_children(children.clone()),
                "edit"
            }
        }
    })
}

async fn edit_10_children(children: UseSharedState<Children>) {
    let mut children = children.write();
    for i in 0..10 {
        children.0[i].value = format!("edited {}", children.0[i].value);
    }
}
