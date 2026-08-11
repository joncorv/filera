use std::println;
use std::process::Command;

const CARGO_DIR: &str = env!("CARGO_MANIFEST_DIR");
use filera_lib::atomics::*;
use filera_lib::process_tasks::*;
use filera_lib::user_std::*;
use filera_lib::*;

fn main() {
    // create dummy data before we do the benchmarks.
    println!("Now we're doing benchmarks!");

    // let mut list_dir = Command::new("ls");
    // list_dir.current_dir("./benches/typical_files");
    // list_dir.status().expect("process failed to execute");
    //
    // let mut state = AppState::default();
    // let mut files: Vec<String> = Vec::new();
    //
    // let test_directory = CARGO_DIR.to_string() + "/benches/typical_files";
    //
    // files.push(test_directory);

    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn my_bench(bencher: divan::Bencher) {
    use tauri::test::mock_builder;
    use tauri::test::*;
    use tauri::Manager;

    bencher
        .with_inputs(|| {
            // untimed: build whatever odd type you need

            // let app = tauri::test::mock_app();

            let builder = mock_builder().setup(|app| {
                app.manage(Mutex::new(AppState::default()));
                Ok(())
            });

            // create app state
            // tauri::Builder::default()
            //     .setup(|app| {
            //         app.manage(Mutex::new(AppState::default()));
            //         Ok(())
            //     })
            //     .expect("error while running tauri application");

            // let mut state = Mutex::new(AppState::default());
            // let mut state_2 = tauri::State::from(Mutex::new(AppState::default()));

            let mut files: Vec<String> = Vec::new();

            let test_directory = CARGO_DIR.to_string() + "/benches/typical_files";

            files.push(test_directory);
            (files, state)
        })
        .bench_values(|(files, state)| {
            // timed: only this closure is measured
            user_dragdrop_files(files, state)
        });
}

// Register a `fibonacci` function and benchmark it over multiple cases.
// #[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}
//
// #[divan::bench(args = [10, 20, 40, 80, 160, 320])]
// fn simple_test(n: usize) {
//     let my_struct = 0..n;
//
//     for num in my_struct {
//         let nothing = num.clone();
//         if nothing % 2 == 0 {
//             //
//         }
//     }
// }
//

// // #[divan::bench(args = [files, state])]
// pub fn user_dragdrop_files(files: Vec<String>, state: State<'_, Mutex<AppState>>) -> FileStatusResponse {
//     let mut file_names: Vec<String> = Vec::new();
//     for file in files {
//         let current_file = PathBuf::from(file.clone());
//         if current_file.is_file() {
//             file_names.push(file);
//         } else if current_file.is_dir() {
//             for entry in WalkDir::new(current_file).into_iter().filter_map(|e| e.ok()) {
//                 file_names.push(entry.path().to_string_lossy().to_string());
//             }
//         }
//     }
//     solve_duplicates(file_names, &state);
//     sort_file_names(&state);
//     convert_file_names_to_working_files(&state);
//     process_tasks_on_working_files(&state);
//     resolve_workingfile_duplicates(&state);
//     state_clear_selected_filestatuses(&state);
//     convert_working_files_to_file_status(&state);
//     apply_selections_to_filestatuses(&state);
//     apply_search(&state);
//     build_response(&state)
// }
