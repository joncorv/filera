// use notify_rust::Notification;
// use std::fs::{self, metadata, Metadata};
use std::sync::Mutex;
use std::{fs::rename, path::PathBuf};
use tauri::{Manager, State};
use time::OffsetDateTime;

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
        at_start: bool,
        separator: String,
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
        separator: String,
        active: bool,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
enum SortMetadata {
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
    sort: SortMetadata,
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
        .invoke_handler(tauri::generate_handler![
            user_open_files,
            user_update_tasks,
            user_clear_files
        ])
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

// #[tauri::command]
// fn user_open_folders() {}

#[tauri::command]
fn user_clear_files(state: State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.file_names.clear();
    state.working_files.clear();

    // let empty_file_status: Vec<FileStatus> = Vec::new();
    // convert_working_files_to_file_status(&state)
}

// #[tauri::command]
// fn user_sort() {}

#[tauri::command]
fn user_update_tasks(task_list: Vec<Task>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    state_update_tasks(task_list, &state);
    // convert_file_names_to_working_files_(&state);
    process_tasks_on_working_files_(&state);
    convert_working_files_to_file_status(&state)
}

// fn user_change_target_directory() {}
// fn user_rename_files() {}

#[tauri::command]
fn solve_duplicates(file_names: Vec<String>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut file_names = file_names;
    state.file_names.append(&mut file_names);
    state.file_names.sort_unstable();
    state.file_names.dedup();
}

// fn state_update_sort() {}
// fn sort_file_names() {}

#[tauri::command]
fn state_update_tasks(task_list: Vec<Task>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.tasks = task_list.clone();
}

#[tauri::command]
fn convert_file_names_to_working_files_(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut new_working_files: Vec<WorkingFile> = Vec::with_capacity(state.file_names.len());
    let file_paths = &state.file_names;

    for file_path in file_paths {
        let working_file = WorkingFile {
            source: PathBuf::from(file_path.clone()),
            target: PathBuf::new(),
            active: true,
        };
        new_working_files.push(working_file);
    }
    state.working_files = new_working_files;
}

// fn state_update_target_directory() {}

#[tauri::command]
fn process_tasks_on_working_files_(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let tasks = &state.tasks.clone();

    for (index, file) in &mut state.working_files.iter_mut().enumerate() {
        // for each WorkingFile in the array, copy from source => target
        file.target = file.source.clone();

        // iterate over all tasks in task_list
        for task in tasks {
            match task {
                Task::CustomText {
                    text,
                    at_start,
                    active,
                } => {
                    if *active {
                        let file_stem: String;
                        let file_extension: String;

                        if let Some(t) = file.target.file_stem() {
                            file_stem = t.to_string_lossy().to_string();
                        } else {
                            file_stem = "".to_string();
                        }

                        if let Some(t) = file.target.extension() {
                            file_extension = t.to_string_lossy().to_string();
                        } else {
                            file_extension = "".to_string();
                        }

                        if *at_start {
                            file.target
                                .set_file_name(format!("{}{}.{}", text, file_stem, file_extension));
                        } else {
                            file.target
                                .set_file_name(format!("{}{}.{}", file_stem, text, file_extension));
                        };
                    }
                }
                Task::FindAndReplace {
                    find_text,
                    replace_text,
                    active,
                } => {
                    if *active {
                        let new_file_name: String;

                        if let Some(t) = file.target.file_name() {
                            new_file_name = t
                                .to_string_lossy()
                                .to_string()
                                .replace(find_text, replace_text);
                        } else {
                            new_file_name = "".to_string();
                        }

                        file.target.set_file_name(new_file_name);
                    }
                }

                Task::ClearAll { active } => {
                    if *active {
                        let file_extension: String;

                        if let Some(t) = file.target.extension() {
                            file_extension = t.to_string_lossy().to_string();
                            file.target.set_file_name(format!(".{}", file_extension));
                        } else {
                            file.target.set_file_name("");
                        }
                    }
                }

                Task::ChangeCase {
                    case_choice,
                    active,
                } => {
                    if *active {
                        let new_file_name: String;

                        if let Some(t) = file.target.file_name() {
                            new_file_name = t.to_string_lossy().to_string();
                        } else {
                            new_file_name = "".to_string();
                        }

                        match *case_choice {
                            0_u8 => {
                                // UPPERCASE
                                file.target.set_file_name(new_file_name.to_uppercase());
                            }
                            1_u8 => {
                                // lowercase
                                file.target.set_file_name(new_file_name.to_lowercase());
                            }
                            2_u8..=u8::MAX => {
                                // figure out other case options later
                                // file.target.set_file_name(new_file_name);
                            }
                        }
                    }
                }
                Task::NumSequence {
                    start_num,
                    num_padding,
                    separator,
                    at_start,
                    active,
                } => {
                    if *active {
                        let file_stem: String;
                        let file_extension: String;
                        let index_u64 = u64::try_from(index).unwrap();
                        let num_padding_usize: usize = usize::try_from(*num_padding).unwrap();

                        if let Some(t) = file.target.file_stem() {
                            file_stem = t.to_string_lossy().to_string();
                        } else {
                            file_stem = "".to_string();
                        }

                        let raw_sequence_num = index_u64 + start_num;
                        let sequence_num = format!("{:01$}", raw_sequence_num, num_padding_usize);

                        if let Some(t) = file.target.extension() {
                            file_extension = t.to_string_lossy().to_string();

                            // build the file name with extension, and number at start
                            if *at_start {
                                file.target.set_file_name(format!(
                                    "{}{}{}.{}",
                                    sequence_num, separator, file_stem, file_extension
                                ));
                            } else
                            // build the file name with extension, and number at the end
                            {
                                file.target.set_file_name(format!(
                                    "{}{}{}.{}",
                                    file_stem, separator, sequence_num, file_extension
                                ));
                            }
                        } else {
                            // build the file name without extension, and number at start
                            if *at_start {
                                file.target.set_file_name(format!(
                                    "{}{}{}",
                                    sequence_num, separator, file_stem
                                ));
                            } else
                            // build the file name without an extension, and number at the end
                            {
                                file.target.set_file_name(format!(
                                    "{}{}{}",
                                    file_stem, separator, sequence_num
                                ));
                            }
                        }
                    }
                }
                Task::Date {
                    year,
                    month,
                    day,
                    year_4,
                    separator,
                    active,
                } => {
                    if *active {
                        let file_metadata = file.source.metadata();

                        if let Ok(t) = file_metadata {
                            // create SystemTime Instance from metadata
                            let systime = t.modified();

                            if let Ok(j) = systime {
                                // able to access metadata and file modified info
                                let datetime = OffsetDateTime::from(j);
                                let datetime_year = datetime.year();
                                let datetime_month = datetime.month();
                                let datetime_day = datetime.day();

                                // now that we have system time, let's build the file
                                // get file_stem and file_extension
                                let file_stem: String;
                                let file_extension: String;

                                // if there is a file stem, use it, or make it blank
                                if let Some(t) = file.target.file_stem() {
                                    file_stem = t.to_string_lossy().to_string();
                                } else {
                                    file_stem = "".to_string();
                                }

                                // if there isn't an extension, we do need to change how we approach building here
                                if let Some(t) = file.target.extension() {
                                    file_extension = t.to_string_lossy().to_string();
                                    file.target.set_file_name(format!(
                                        "{}{}{}{}{}{}{}.{}",
                                        datetime_year,
                                        separator,
                                        datetime_month,
                                        separator,
                                        datetime_day,
                                        separator,
                                        file_stem,
                                        file_extension
                                    ));
                                } else {
                                    file.target.set_file_name(format!(
                                        "{}{}{}{}{}{}{}",
                                        datetime_year,
                                        separator,
                                        datetime_month,
                                        separator,
                                        datetime_day,
                                        separator,
                                        file_stem
                                    ));
                                }
                            } else {
                                // able to access metadata but not file modified info
                                // i guess we just skip this step altogether for now
                                // should probably bubble up an error to the user
                            }
                        } else {
                            // This means we can't access metadata on file at all
                            // i guess we just skip this step altogether for now
                            // should probably bubble up an error to the user
                        }
                    }
                }
                Task::Time {
                    hour_24,
                    separator,
                    active,
                } => {
                    if *active {
                        let file_metadata = file.source.metadata();

                        if let Ok(t) = file_metadata {
                            // create SystemTime Instance from metadata
                            let systime = t.modified();

                            if let Ok(j) = systime {
                                // able to access metadata and file modified info
                                let datetime = OffsetDateTime::from(j);
                                let datetime_hour = datetime.hour();
                                let datetime_minute = datetime.minute();
                                let datetime_second = datetime.second();

                                // now that we have system time, let's build the file
                                // get file_stem and file_extension
                                let file_stem: String;
                                let file_extension: String;

                                // if there is a file stem, use it, or make it blank
                                if let Some(t) = file.target.file_stem() {
                                    file_stem = t.to_string_lossy().to_string();
                                } else {
                                    file_stem = "".to_string();
                                }

                                // if there isn't an extension, we do need to change how we approach building here
                                if let Some(t) = file.target.extension() {
                                    file_extension = t.to_string_lossy().to_string();
                                    file.target.set_file_name(format!(
                                        "{}{}{}{}{}{}{}.{}",
                                        datetime_hour,
                                        separator,
                                        datetime_minute,
                                        separator,
                                        datetime_second,
                                        separator,
                                        file_stem,
                                        file_extension
                                    ));
                                } else {
                                    file.target.set_file_name(format!(
                                        "{}{}{}{}{}{}{}",
                                        datetime_hour,
                                        separator,
                                        datetime_minute,
                                        separator,
                                        datetime_second,
                                        separator,
                                        file_stem
                                    ));
                                }
                            } else {
                                // able to access metadata but not file modified info
                                // i guess we just skip this step altogether for now
                                // should probably bubble up an error to the user
                            }
                        } else {
                            // This means we can't access metadata on file at all
                            // i guess we just skip this step altogether for now
                            // should probably bubble up an error to the user
                        }
                    }
                }
            }
        }
    }
}

// fn state_update_working_files() {}

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

// fn rename_files() {}

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
