use std::collections::HashMap;

use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running big model example");
    launch(App);
}

#[component]
fn App() -> Element {
    let model = use_context_provider(|| Signal::new(Model::new()));
    rsx! {
        button {
            onclick: move |_| {
                update_model_task(model)
            },
            "update task"
        }
        button {
            onclick: move |_| {
                update_model_user(model)
            },
            "update user"
        }
        for &task_id in model.read().tasks.keys() {
            Task { task_id }
        }
    }
}

#[component]
fn Task(task_id: TaskId) -> Element {
    tracing::info!("rendering task {}", task_id.0);
    let model = use_context::<Signal<Model>>();
    let read_model = model.read();
    let data = &read_model.tasks[&task_id];
    rsx! {
        div {
            h1 { "{data.title}" }
            p { "{data.description}" }
            for user in data
                .users
                .iter()
                .map(|&user_id| &read_model.users[&user_id])
            {
                p {
                    "{user.name}"
                    "{user.color}"
                }
            }
        }
    }
}

struct Model {
    users: HashMap<UserId, UserData>,
    tasks: HashMap<TaskId, TaskData>,
}

impl Model {
    fn new() -> Self {
        Self {
            tasks: HashMap::from([
                (
                    TaskId(1),
                    TaskData {
                        title: "task 1".to_string(),
                        description: "task 1 description".to_string(),
                        users: vec![UserId(1), UserId(2)],
                    },
                ),
                (
                    TaskId(2),
                    TaskData {
                        title: "task 2".to_string(),
                        description: "task 2 description".to_string(),
                        users: vec![UserId(3), UserId(4)],
                    },
                ),
            ]),
            users: HashMap::from([
                (
                    UserId(1),
                    UserData {
                        name: "user 1".to_string(),
                        color: "red".to_string(),
                    },
                ),
                (
                    UserId(2),
                    UserData {
                        name: "user 2".to_string(),
                        color: "blue".to_string(),
                    },
                ),
                (
                    UserId(3),
                    UserData {
                        name: "user 3".to_string(),
                        color: "green".to_string(),
                    },
                ),
                (
                    UserId(4),
                    UserData {
                        name: "user 4".to_string(),
                        color: "yellow".to_string(),
                    },
                ),
            ]),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u32);

struct UserData {
    name: String,
    color: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct TaskId(u32);

struct TaskData {
    title: String,
    description: String,
    users: Vec<UserId>,
}

fn update_model_task(mut model: Signal<Model>) {
    let mut model = model.write();
    let task = model.tasks.get_mut(&TaskId(1)).unwrap();
    task.title = "task 1 updated".to_string();
}

fn update_model_user(mut model: Signal<Model>) {
    let mut model = model.write();
    let user = model.users.get_mut(&UserId(1)).unwrap();
    user.name = "user 1 updated".to_string();
}
