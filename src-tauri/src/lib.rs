use notify_rust::Notification;
use std::any::Any;
use std::io::Cursor;
use std::sync::Mutex;
use std::{fs::rename, path::PathBuf};
use tauri::utils::config::parse::EXTENSIONS_SUPPORTED;
use tauri::{Manager, State};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct WorkingFile {
    source: PathBuf,
    target: PathBuf,
    active: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct FileStatus {
    old_file_name: String,
    new_file_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum Task {
    CustomText {
        text: String,
        at_start: bool,
        active: bool,
    },
    FindAndReplace {
        find_text: String,
        replace_text: String,
        active: bool,
    },
    ClearAll {
        active: bool,
    },
    ChangeCase {
        case_choice: u8,
        active: bool,
    },
    NumSequence {
        start_num: u64,
        num_padding: u64,
        active: bool,
    },
    Date {
        year: bool,
        month: bool,
        day: bool,
        year_4: bool,
        separator: String,
        active: bool,
    },
    Time {
        hour_24: bool,
        ampm: bool,
        separator: String,
        active: bool,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
enum Metadata {
    #[default]
    Name,
    DateCreated,
    DateModified,
    Type,
    Size,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct AppState {
    file_names: Vec<String>,
    working_files: Vec<WorkingFile>,
    tasks: Vec<Task>,
    sort: Metadata,
}

// App Entry Point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![user_open_files, user_update_tasks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// these are all Tauri commands
// they should all return Vec<FileStatus>
#[tauri::command]
fn user_open_files(file_names: Vec<String>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    solve_duplicates(file_names, &state);
    // sort_file_names();
    convert_file_names_to_working_files_(&state);
    process_tasks_on_working_files_(&state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
fn user_open_folders() {}

#[tauri::command]
fn user_sort() {}

#[tauri::command]
fn user_update_tasks(task_list: Vec<Task>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    println!("1st user_update_tasks: {:?}", &task_list);
    state_update_tasks(task_list, &state);
    convert_file_names_to_working_files_(&state);
    process_tasks_on_working_files_(&state);
    convert_working_files_to_file_status(&state)
}

fn user_change_target_directory() {}
fn user_rename_files() {}

#[tauri::command]
fn solve_duplicates(file_names: Vec<String>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut file_names = file_names;
    state.file_names.append(&mut file_names);
    state.file_names.sort_unstable();
    state.file_names.dedup();
}

fn state_update_sort() {}
fn sort_file_names() {}

#[tauri::command]
fn state_update_tasks(task_list: Vec<Task>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.tasks = task_list.clone();
    println!("2nd state_update_tasks: {:?}", state.tasks);
}

#[tauri::command]
fn convert_file_names_to_working_files_(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut new_working_files: Vec<WorkingFile> = Vec::with_capacity(state.file_names.len());
    let file_paths = &state.file_names;

    for file_path in file_paths {
        let working_file = WorkingFile {
            source: PathBuf::from(file_path.clone()),
            target: PathBuf::from(file_path.clone()),
            active: true,
        };
        new_working_files.push(working_file);
    }
    state.working_files = new_working_files;
}

fn state_update_target_directory() {}

#[tauri::command]
fn process_tasks_on_working_files_(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let tasks = &state.tasks;
    // println!("3rd state_process_tasks: {:?}", &tasks);
    println!("3rd working_files_begin: {:?}", &state.working_files);

    // if tasks.first() == Task::CustomText {};

    // if tasks.type == TaskType::CustomText
    let mut my_type = "fuck".to_string();
    for task in tasks {
        if let Task::CustomText {
            text,
            at_start,
            active,
        } = task
        {
            my_type = text.clone();
        }
    }

    println!("{}", my_type);

    for file in &mut state.working_files {
        let file_stem = file.target.file_stem().unwrap().to_str().unwrap();
        let file_extension = file.target.extension().unwrap().to_str().unwrap();

        file.target
            .set_file_name(format!("{}{}.{}", my_type, file_stem, file_extension))
    }

    // for file in &mut state.working_files {
    //     for task in &tasks {
    //         match task {
    //             Task::CustomText {
    //                 text,
    //                 at_start,
    //                 active,
    //             } => {
    //                 if *active {
    //                     // let parent = file.target.parent().unwrap();
    //                     let file_stem = file.target.file_stem().unwrap().to_str().unwrap();
    //                     let file_extension = file.target.extension().unwrap().to_str().unwrap();

    //                     if *at_start {
    //                         file.target
    //                             .set_file_name(format!("{}{}.{}", text, file_stem, file_extension))
    //                     } else {
    //                         file.target
    //                             .set_file_name(format!("{}{}.{}", file_stem, text, file_extension))
    //                     };
    //                 }
    //             }
    //             Task::FindAndReplace {
    //                 find_text,
    //                 replace_text,
    //                 active,
    //             } => {
    //                 // if *active {
    //                 //     file.new_file_name = file.new_file_name.replace(find_text, replace_text);
    //                 // }
    //             }
    //             Task::ClearAll { active } => {}
    //             Task::ChangeCase {
    //                 case_choice,
    //                 active,
    //             } => {}
    //             Task::NumSequence {
    //                 start_num,
    //                 num_padding,
    //                 active,
    //             } => {}
    //             Task::Date {
    //                 year,
    //                 month,
    //                 day,
    //                 year_4,
    //                 separator,
    //                 active,
    //             } => {}
    //             Task::Time {
    //                 hour_24,
    //                 ampm,
    //                 separator,
    //                 active,
    //             } => {}
    //         }
    //     }
    // }
    println!("3rd working_files_end: {:?}", &state.working_files);
}

fn state_update_working_files() {}
fn convert_working_files_to_file_status(state: &State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    let state = state.lock().unwrap();
    let mut file_statuses: Vec<FileStatus> = Vec::with_capacity(state.working_files.len());
    for working_file in &state.working_files {
        let file_status = FileStatus {
            old_file_name: working_file
                .source
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            new_file_name: working_file
                .target
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        };
        file_statuses.push(file_status);
    }
    file_statuses
}
fn rename_files() {}

// #[tauri::command]
// fn open_files(file_names: Vec<String>, task_list: Vec<Task>) -> Vec<WorkingFile> {
//     // println!("{:?}", &task_list);

//     let mut all_working_files = file_names_to_working_files(file_names);
//     all_working_files = process_tasks_on_working_files(all_working_files, task_list);
//     return all_working_files;
// }

// // Process files from the front end from paths to WorkingFiles
// fn file_names_to_working_files(file_names: Vec<String>) -> Vec<WorkingFile> {
//     // Instantiate a new Vector of Strings to work on
//     let mut all_paths: Vec<String> = file_names;

//     // Sort and De-Duplicate all paths
//     all_paths.sort_unstable();
//     all_paths.dedup();

//     // Instantiate a empty vector of WorkingFile on the heap
//     let mut all_working_files: Vec<WorkingFile> = Vec::new();

//     // convert all paths into WorkingFiles
//     for file in all_paths {
//         let path_object = std::path::Path::new(&file)
//             .file_name()
//             .unwrap()
//             .to_str()
//             .unwrap();

//         let working_workingfile = WorkingFile {
//             path: file.to_string(),
//             old_file_name: path_object.to_string(),
//             new_file_name: path_object.to_string(),
//             active: true,
//         };
//         all_working_files.push(working_workingfile);
//     }

//     all_working_files
// }

// fn update_tasks(task_list: Vec<Task>) -> Vec<Task> {
//     // Update the tasks in the task list based on user input.
// }

// fn process_tasks_on_working_files(
//     mut all_working_files: Vec<WorkingFile>,
//     task_list: Vec<Task>,
// ) -> Vec<WorkingFile> {
//     // Process all WorkingFiles and apply the effects sent from the front end.

//     for file in &mut all_working_files {
//         for task in &task_list {
//             match task {
//                 Task::CustomText {
//                     text,
//                     at_start,
//                     active,
//                 } => {
//                     if *active {
//                         file.new_file_name = format!("{}{}", text, file.new_file_name);
//                     };
//                 }
//                 Task::FindAndReplace {
//                     find_text,
//                     replace_text,
//                     active,
//                 } => {
//                     if *active {
//                         file.new_file_name = file.new_file_name.replace(find_text, replace_text);
//                     }
//                 }
//                 Task::ClearAll { active } => {}
//                 Task::ChangeCase {
//                     case_choice,
//                     active,
//                 } => {}
//                 Task::NumSequence {
//                     start_num,
//                     num_padding,
//                     active,
//                 } => {}
//                 Task::Date {
//                     year,
//                     month,
//                     day,
//                     year_4,
//                     separator,
//                     active,
//                 } => {}
//                 Task::Time {
//                     hour_24,
//                     ampm,
//                     separator,
//                     active,
//                 } => {}
//             }
//         }
//     }

//     return all_working_files;
// }

// #[tauri::command]
// fn rename_files(file_names: Vec<String>, task_list: Vec<Task>) -> String {
//     // convert the file_names data to a Vec of WorkingFiles
//     let mut all_working_files = file_names_to_working_files(file_names);

//     // process tasks on working files
//     all_working_files = process_tasks_on_working_files(all_working_files, task_list);

//     let mut rename_errors: Vec<String> = Vec::new();

//     // construct files and write to disk
//     for file in &all_working_files {
//         let path = std::path::Path::new(&file.path);

//         let parent: String;
//         let parent_calc = path.parent();
//         match parent_calc {
//             Some(t) => parent = t.to_str().unwrap().to_string(),
//             None => parent = "".to_string(),
//         };

//         let destination = format!("{}/{}", parent, &file.new_file_name);

//         let rename_result = rename(&file.path, destination);
//         match rename_result {
//             // If successfully, we do nothing.
//             Ok(_) => {}

//             // Log errors to an Array of errors.
//             Err(e) => rename_errors.push(format!("Rename Error: {} \n", e)),
//         }
//     }

//     return "Files successfully converted.".to_string();
// }

// #[tauri::command]
// fn test_working_file(working_file: WorkingFile) -> WorkingFile {
//     // let incoming_info = format!("well this is what we got: {:?}", &working_file);
//     working_file
// }

// // This should print to the fucking console but it doesn't.
// // This seems to be a linux only issue.
// #[tauri::command]
// fn print_something_to_console(console_title: &str, console_text: &str) -> String {
//     let my_line: String = format!("{}\n{}", console_title, console_text);
//     println!("{}", &my_line);

//     Notification::new()
//         .summary("This is the summary")
//         .body("This is the body")
//         .icon("firefox")
//         .show()
//         .unwrap();
//     return my_line;
// }
