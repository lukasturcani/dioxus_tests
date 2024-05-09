use std::collections::HashMap;

use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    tracing::info!("running split model example");
    launch(App);
}

#[component]
fn App() -> Element {
    let tasks = use_context_provider(|| Signal::new(Tasks::new()));
    let users = use_context_provider(|| Signal::new(Users::new()));
    rsx! {
        button {
            onclick: move |_| {
                update_model_task(tasks)
            },
            "update task"
        }
        button {
            onclick: move |_| {
                update_model_user(users)
            },
            "update user"
        }
        for (&task_id, task) in tasks.read().0.iter() {
            Task {
                task_id,
                task: clone(task)
            }
        }
    }
}

fn clone(task: &TaskData) -> TaskData {
    tracing::info!("cloning task");
    task.clone()
}

#[component]
fn Task(task_id: TaskId, task: TaskData) -> Element {
    tracing::info!("rendering task {}", task_id.0);
    let users = use_context::<Signal<Users>>();
    let read_users = users.read();
    rsx! {
        div {
            h1 { {task.title} }
            p { {task.description} }
            for user in task
                .users
                .iter()
                .map(|&user_id| &read_users.0[&user_id])
            {
                p {
                    "{user.name}"
                    "{user.color}"
                }
            }
        }
    }
}

struct Tasks(HashMap<TaskId, TaskData>);
impl Tasks {
    fn new() -> Self {
        Self(HashMap::from([
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
        ]))
    }
}

struct Users(HashMap<UserId, UserData>);
impl Users {
    fn new() -> Self {
        Self(HashMap::from([
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
        ]))
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

#[derive(Clone, PartialEq, Eq)]
struct TaskData {
    title: String,
    description: String,
    users: Vec<UserId>,
}

fn update_model_task(mut model: Signal<Tasks>) {
    let mut model = model.write();
    let task = model.0.get_mut(&TaskId(1)).unwrap();
    task.title = "task 1 updated".to_string();
}

fn update_model_user(mut model: Signal<Users>) {
    let mut model = model.write();
    let user = model.0.get_mut(&UserId(1)).unwrap();
    user.name = "user 1 updated".to_string();
}
