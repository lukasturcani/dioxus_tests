use std::fmt::Display;

use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running borrowed props example");
    launch(App);
}

#[component]
fn App() -> Element {
    tracing::info!("rendering app");
    let mut counter = use_signal(|| 0);
    let mut local_counter = use_signal(|| 0);
    rsx! {
        div {
            button {
                onclick: move |_| counter.set(counter()+1),
                "parent global counter: {counter:?}"
            }
            button {
                onclick: move |_| local_counter.set(local_counter()+1),
                "parent local counter: {local_counter:?}"
            }
        }
        GoodChild { counter, value: MyString("good".into()) },
        BadChild { counter, value: MyString("bad".into()) },
    }
}

#[derive(PartialEq, Eq, Debug)]
struct MyString(String);

impl Clone for MyString {
    fn clone(&self) -> Self {
        tracing::info!("cloning {}", self.0);
        Self(self.0.clone())
    }
}

impl Display for MyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[component]
fn GoodChild(value: ReadOnlySignal<MyString>, counter: Signal<i32>) -> Element {
    tracing::info!("rendering child: {value} {counter:?}");
    let mut local_counter = use_signal(|| 0);
    rsx! {
        div {
            button {
                onclick: move |_| counter.set(counter()+1),
                "{value} global counter {counter:?}"
            }
            button {
                onclick: move |_| local_counter.set(local_counter()+1),
                "{value} local counter: {local_counter:?}"
            }
        }
    }
}

#[component]
fn BadChild(value: MyString, counter: Signal<i32>) -> Element {
    tracing::info!("rendering child: {value} {counter:?}");
    let mut local_counter = use_signal(|| 0);
    rsx! {
        div {
            button {
                onclick: move |_| counter.set(counter()+1),
                "{value} global counter {counter:?}"
            }
            button {
                onclick: move |_| local_counter.set(local_counter()+1),
                "{value} local counter: {local_counter:?}"
            }
        }
    }
}
