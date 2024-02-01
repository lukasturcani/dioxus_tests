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
                    state: child.clone(),
                    on_click: |_|  cx.spawn(edit_10_children(children.clone())),
                }))
        }
    })
}

#[component]
fn Child<'a>(cx: Scope, state: ChildState, on_click: EventHandler<'a>) -> Element {
    log::info!("rendering child");
    cx.render(rsx! {
        li {
            key: "{state.value}",
            state.value.clone()
            button {
                r#type: "button",
                onclick: |_| on_click.call(()),
                "edit"
            }
        }
    })
}

async fn edit_10_children(children: UseSharedState<Children>) {
    let mut children = children.write();
    for i in 0..10 {
        children.0[i].value = "edited".to_string();
    }
}
