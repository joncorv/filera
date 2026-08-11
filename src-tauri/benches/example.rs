use std::sync::Mutex;

use divan::Bencher;
use tauri::test::{mock_app, MockRuntime};
use tauri::{App, Manager, State};

use filera_lib::atomics::state_update_tasks;
use filera_lib::process_tasks::process_tasks_on_working_files;
use filera_lib::user_std::user_dragdrop_files;
use filera_lib::{AppState, Task};

const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    divan::main();
}

fn managed_state() -> State<'static, Mutex<AppState>> {
    let app = mock_app();
    app.manage(Mutex::new(AppState::default()));

    let app: &'static App<MockRuntime> = Box::leak(Box::new(app));
    app.state::<Mutex<AppState>>()
}

fn test_directory() -> String {
    CARGO_DIR.to_string() + "/benches/typical_files"
}

fn basic_tasks() -> Vec<Task> {
    vec![
        Task::FindAndReplace {
            find_text: "_".to_string(),
            replace_text: "-".to_string(),
            active: true,
        },
        Task::ChangeCase {
            case_choice: 0, // 0 = lowercase, 1 = uppercase
            active: true,
        },
        Task::CustomText {
            text: "archive-".to_string(),
            at_start: true,
            active: true,
        },
        Task::NumSequence {
            start_num: 1,
            num_padding: 3,
            at_start: false,
            separator: "-".to_string(),
            active: true,
        },
        Task::FilterName {
            inclusive: true,
            name: "draft".to_string(),
        },
    ]
}

#[divan::bench]
fn dragdrop_no_tasks(bencher: Bencher) {
    let state = managed_state();
    let directory = test_directory();

    bencher
        .with_inputs(|| {
            *state.lock().unwrap() = AppState::default();
            (vec![directory.clone()], state.clone())
        })
        .bench_values(|(files, state)| {
            // timed: only this closure is measured
            user_dragdrop_files(files, state)
        });
}

#[divan::bench]
fn dragdrop_basic_tasks(bencher: Bencher) {
    let state = managed_state();
    let directory = test_directory();

    bencher
        .with_inputs(|| {
            *state.lock().unwrap() = AppState::default();
            state_update_tasks(basic_tasks(), &state);

            (vec![directory.clone()], state.clone())
        })
        .bench_values(|(files, state)| user_dragdrop_files(files, state));
}

fn populated_state(tasks: Vec<Task>) -> State<'static, Mutex<AppState>> {
    let state = managed_state();

    state_update_tasks(tasks, &state);
    user_dragdrop_files(vec![test_directory()], state.clone());

    state
}

#[divan::bench]
fn process_tasks_basic_tasks(bencher: Bencher) {
    let state = populated_state(basic_tasks());

    bencher.bench(|| process_tasks_on_working_files(&state));
}
