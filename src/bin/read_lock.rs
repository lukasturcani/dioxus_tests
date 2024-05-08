use std::collections::HashMap;

use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running shared state example");
    launch(App);
}

#[derive(Default)]
struct Model {
    users: HashMap<String, String>,
    content: HashMap<String, String>,
}

#[component]
pub fn App() -> Element {
    tracing::info!("rendering app");
    let model = use_context_provider(|| {
        Signal::new(Model {
            users: HashMap::from([
                ("1".to_string(), "user 1".to_string()),
                ("2".to_string(), "user 2".to_string()),
                ("3".to_string(), "user 3".to_string()),
                ("4".to_string(), "user 4".to_string()),
                ("5".to_string(), "user 5".to_string()),
                ("6".to_string(), "user 6".to_string()),
                ("7".to_string(), "user 7".to_string()),
                ("8".to_string(), "user 8".to_string()),
                ("9".to_string(), "user 9".to_string()),
                ("10".to_string(), "user 10".to_string()),
            ]),
            content: HashMap::from([
                ("user 1".to_string(), "content 1".to_string()),
                ("user 2".to_string(), "content 2".to_string()),
            ]),
        })
    });
    let read_model = model.read();
    let value_signal = use_signal(|| Some(String::from("hello")));
    let value = value_signal.read();
    rsx! {
        button {
            r#type: "button",
            "click me"
        }
        ul {
            for (id, name, content) in read_model
                .users
                .iter()
                .map(|(id, name)| (id, name, &read_model.content[name]))
            {
                li {
                    key: "{id}",
                    onclick: {
                        let id = id.clone();
                        move |_| foo(model, id.clone())
                    },
                    "{name} {content}"
                }
            }
        }
        ul {
            for (id, name) in read_model.users.iter() {
                li {
                    key: "{id}",
                    "{name}"
                }
            }
        }
        if let Some(inner) = &*value {
            div {
                "{inner}"
            }
        }
    }
}

async fn foo(model: Signal<Model>, id: String) {
    todo!()
}
