// use notify_rust::Notification;
use rfd::{AsyncMessageDialog, MessageDialogResult};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::time::SystemTime;
use std::{fs::rename, path::PathBuf};
use tauri::{Manager, State};
use tauri_plugin_notification::NotificationExt;
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
        year: u8,
        month: bool,
        day: bool,
        at_start: bool,
        separator: String,
        active: bool,
    },
    Time {
        at_start: bool,
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
enum Output {
    #[default]
    Replace,
    Copy {
        directory: String,
    },
    Move {
        directory: String,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
struct AppState {
    file_names: Vec<String>,
    file_names_sorted: Vec<String>,
    working_files: Vec<WorkingFile>,
    tasks: Vec<Task>,
    sort_choice: String,
    search: String,
    output: Output,
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
            user_update_sort,
            user_update_tasks,
            user_update_search,
            user_clear_files,
            user_update_search,
            user_rename_files,
            user_notification,
            user_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// these are all Tauri commands
// they should all return Vec<FileStatus>
#[tauri::command]
fn user_open_files(file_names: Vec<String>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    solve_duplicates(file_names, &state);
    sort_file_names(&state);
    convert_file_names_to_working_files(&state);
    process_tasks_on_working_files(&state);
    resolve_workingfile_duplicates(&state);
    convert_working_files_to_file_status(&state)
}

// #[tauri::command]
// fn user_open_folders() {}

#[tauri::command]
fn user_clear_files(state: State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.file_names.clear();
    state.working_files.clear();
}

#[tauri::command]
fn user_update_sort(sort_choice: String, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    state_update_sort(sort_choice, &state);
    sort_file_names(&state);
    convert_file_names_to_working_files(&state);
    process_tasks_on_working_files(&state);
    resolve_workingfile_duplicates(&state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
fn user_update_tasks(task_list: Vec<Task>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    state_update_tasks(task_list, &state);
    process_tasks_on_working_files(&state);
    resolve_workingfile_duplicates(&state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
fn user_update_search(search: String, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    state_update_search(search, &state);
    convert_working_files_to_file_status(&state)
}

// fn user_change_target_directory() {}

#[tauri::command]
fn solve_duplicates(file_names: Vec<String>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut file_names = file_names;
    state.file_names.append(&mut file_names);
    state.file_names.sort_unstable();
    state
        .file_names
        .sort_by(|a, b| natord::compare(&a.to_lowercase(), &b.to_lowercase()));
    state.file_names.dedup();
}

fn sort_file_names(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let sort_choice: &str = &state.sort_choice;

    match sort_choice {
        "name" => {
            let mut valid_files: Vec<(String, String)> = state
                .file_names
                .iter()
                .filter_map(|file| {
                    let filename = Path::new(file).file_name()?.to_string_lossy().to_string();
                    Some((file.clone(), filename))
                })
                .collect();

            valid_files.sort_by_key(|(_, key)| key.clone());
            state.file_names_sorted = valid_files.into_iter().map(|(path, _)| path).collect();
        }
        "modified" => {
            let mut valid_files: Vec<(String, SystemTime)> = state
                .file_names
                .iter()
                .filter_map(|file| {
                    let meta_data = std::fs::metadata(file)
                        .ok()?
                        .modified()
                        .unwrap_or(SystemTime::UNIX_EPOCH);
                    Some((file.clone(), meta_data))
                })
                .collect();

            valid_files.sort_by_key(|(_, key)| key.clone());
            state.file_names_sorted = valid_files.into_iter().map(|(path, _)| path).collect();
        }
        "created" => {
            let mut valid_files: Vec<(String, SystemTime)> = state
                .file_names
                .iter()
                .filter_map(|file| {
                    let meta_data = std::fs::metadata(file)
                        .ok()?
                        .created()
                        .unwrap_or(SystemTime::UNIX_EPOCH);
                    Some((file.clone(), meta_data))
                })
                .collect();

            valid_files.sort_by_key(|(_, key)| key.clone());
            state.file_names_sorted = valid_files.into_iter().map(|(path, _)| path).collect();
        }
        "size" => {
            let mut valid_files: Vec<(String, _)> = state
                .file_names
                .iter()
                .filter_map(|file| {
                    let meta_data = std::fs::metadata(file).ok()?.len();
                    Some((file.clone(), meta_data))
                })
                .collect();

            valid_files.sort_by_key(|(_, key)| std::cmp::Reverse(key.clone()));
            state.file_names_sorted = valid_files.into_iter().map(|(path, _)| path).collect();
        }
        "type" => {
            let mut valid_files: Vec<(String, String)> = state
                .file_names
                .iter()
                .filter_map(|file| {
                    std::fs::metadata(file).ok()?; // File must exist

                    let extension = Path::new(file)
                        .extension()
                        .map_or("zz_no_ext".to_string(), |ext| ext.to_string_lossy().to_string());

                    Some((file.clone(), extension))
                })
                .collect();

            valid_files.sort_by_key(|(_, key)| key.clone());
            state.file_names_sorted = valid_files.into_iter().map(|(path, _)| path).collect();
        }
        _ => {
            // for now use the name sorting
            let mut valid_files: Vec<(String, String)> = state
                .file_names
                .iter()
                .filter_map(|file| {
                    let filename = Path::new(file).file_name()?.to_string_lossy().to_string();
                    Some((file.clone(), filename))
                })
                .collect();

            valid_files.sort_by_key(|(_, key)| key.clone());
            state.file_names_sorted = valid_files.into_iter().map(|(path, _)| path).collect();
        }
    }
}

fn state_update_sort(sort_choice: String, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.sort_choice = sort_choice;
}

#[tauri::command]
fn state_update_tasks(task_list: Vec<Task>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.tasks = task_list.clone();
}

#[tauri::command]
fn state_update_search(search: String, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.search = search;
}

#[tauri::command]
fn convert_file_names_to_working_files(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut new_working_files: Vec<WorkingFile> = Vec::with_capacity(state.file_names.len());
    let file_paths = &state.file_names_sorted;

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
fn process_tasks_on_working_files(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let tasks = &state.tasks.clone();

    for (index, file) in &mut state.working_files.iter_mut().enumerate() {
        // for each WorkingFile in the array, copy from source => target
        file.target = file.source.clone();

        // iterate over all tasks in task_list
        for task in tasks {
            match task {
                // TODO: Custom Text does not account for files w/o a file extension, or files that
                // start with a "."
                // TODO: account for <BLANK> edge case
                Task::CustomText { text, at_start, active } => {
                    if *active {
                        let file_stem: String;
                        let file_extension: String;
                        let file_name = file.target.file_name().unwrap().to_string_lossy();
                        let start_period: bool = file_name.starts_with(".");
                        let has_period: bool = file_name.contains(".");

                        // NOTE: if the first char is a period, the file stem will be entire file name
                        if let Some(t) = file.target.file_stem() {
                            file_stem = t.to_string_lossy().to_string();
                        } else {
                            file_stem = "".to_string();
                        }

                        // NOTE: if there are no periods, the extension is empty
                        if let Some(t) = file.target.extension() {
                            file_extension = t.to_string_lossy().to_string();
                        } else {
                            file_extension = "".to_string();
                        }

                        // BUG: This is not working properly
                        if *at_start {
                            if start_period || !has_period {
                                file.target.set_file_name(format!("{text}{file_stem}"));
                            } else {
                                file.target.set_file_name(format!("{text}{file_stem}.{file_extension}"));
                            }
                        } else {
                            if start_period || !has_period {
                                file.target.set_file_name(format!("{file_stem}{text}"));
                            } else {
                                file.target.set_file_name(format!("{file_stem}{text}.{file_extension}"));
                            }
                        };
                    }
                }
                Task::FindAndReplace {
                    find_text,
                    replace_text,
                    active,
                } => {
                    if *active {
                        let mut new_file_name: String;

                        let target_file_name = file.target.file_name();

                        // if the file name exists
                        if let Some(t) = target_file_name {
                            // store the file name into temp_val string
                            let temp_val = t.to_string_lossy().to_string();

                            // if the file name is <BLANK>
                            if temp_val == "<BLANK>".to_string() {
                                // because the string is blank, we don't operate.
                                new_file_name = "<BLANK>".to_string();
                            } else {
                                // because it's not blank, we can operate on it
                                new_file_name = temp_val.replace(find_text, replace_text);
                            }
                        } else {
                            new_file_name = "".to_string();
                        }

                        // return the <BLANK> token if new_file_name is empty
                        if new_file_name.is_empty() {
                            new_file_name = "<BLANK>".to_string();
                        }

                        file.target.set_file_name(new_file_name);
                    }
                }

                // TODO: if filename starts w/ or does not contain ".", we return a special "<BLANK>" token that needs to be accounted for as an edge case in the rest of the tasks
                Task::ClearAll { active } => {
                    if *active {
                        let file_extension: String;

                        if let Some(t) = file.target.extension() {
                            file_extension = t.to_string_lossy().to_string();
                            file.target.set_file_name(format!(".{file_extension}"));
                        } else {
                            file.target.set_file_name("<BLANK>");
                        }
                    }
                }

                // TODO: Need to build in cases for TitleCase.
                // TODO: account for <BLANK> edge case
                Task::ChangeCase { case_choice, active } => {
                    if *active {
                        let new_file_name: String;

                        if let Some(t) = file.target.file_name() {
                            new_file_name = t.to_string_lossy().to_string();
                        } else {
                            new_file_name = "".to_string();
                        }

                        match *case_choice {
                            0_u8 => {
                                // lowercase
                                file.target.set_file_name(new_file_name.to_lowercase());
                            }
                            1_u8 => {
                                // UPPERCASE
                                file.target.set_file_name(new_file_name.to_uppercase());
                            }
                            2_u8..=u8::MAX => {
                                // figure out other case options later
                                // file.target.set_file_name(new_file_name);
                            }
                        }
                    }
                }
                // TODO: account for <BLANK> edge case
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
                        let sequence_num = format!("{raw_sequence_num:0num_padding_usize$}");

                        if let Some(t) = file.target.extension() {
                            file_extension = t.to_string_lossy().to_string();

                            // build the file name with extension, and number at start
                            if *at_start {
                                file.target
                                    .set_file_name(format!("{sequence_num}{separator}{file_stem}.{file_extension}"));
                            } else
                            // build the file name with extension, and number at the end
                            {
                                file.target
                                    .set_file_name(format!("{file_stem}{separator}{sequence_num}.{file_extension}"));
                            }
                        } else {
                            // build the file name without extension, and number at start
                            if *at_start {
                                file.target
                                    .set_file_name(format!("{sequence_num}{separator}{file_stem}"));
                            } else
                            // build the file name without an extension, and number at the end
                            {
                                file.target
                                    .set_file_name(format!("{file_stem}{separator}{sequence_num}",));
                            }
                        }
                    }
                }

                // TODO: account for <BLANK> edge case
                Task::Date {
                    year,
                    month,
                    day,
                    at_start,
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
                                let mut dates_combined_vector: Vec<String> = vec![];

                                // calculate year by user input
                                let year_val = datetime.year().to_string();
                                match *year {
                                    0 => {
                                        dates_combined_vector.push(year_val);
                                    }
                                    1 => {
                                        dates_combined_vector
                                            .push(year_val.chars().take(year_val.len().saturating_sub(2)).collect());
                                    }
                                    2_u8..=std::u8::MAX => {}
                                };

                                // calculate month & day by user input
                                if *month {
                                    let month_val = datetime.month() as u8;
                                    dates_combined_vector.push(format!("{month_val:02}"));
                                }
                                if *day {
                                    let day_val = datetime.day();
                                    dates_combined_vector.push(format!("{day_val:02}"));
                                }
                                let dates_formatted_string: String = dates_combined_vector.join(separator);

                                //TODO: Edge Case: Leading "."
                                //TODO: Edge Case: Contains no "."
                                //TODO: Contains the <<SPECIAL_CLEAR_a432LKJ>> value
                                //OK let's write the logic here:
                                // if *at_start -> dates + file_stem + "." + file_extension
                                let file_stem: String;
                                let file_extension: String;
                                // let has_extension: bool;
                                // let has_stem: bool;

                                // if there is a file stem, use it, or make it blank
                                if let Some(t) = file.target.file_stem() {
                                    file_stem = t.to_string_lossy().to_string();
                                } else {
                                    file_stem = "".to_string();
                                }

                                if *at_start {
                                    if let Some(t) = file.target.extension() {
                                        file_extension = t.to_string_lossy().to_string();
                                        file.target.set_file_name(format!(
                                            "{dates_formatted_string}{separator}{file_stem}.{file_extension}"
                                        ));
                                    } else {
                                        file.target
                                            .set_file_name(format!("{dates_formatted_string}{separator}{file_stem}"));
                                    }
                                } else {
                                    if let Some(t) = file.target.extension() {
                                        file_extension = t.to_string_lossy().to_string();
                                        file.target.set_file_name(format!(
                                            "{file_stem}{separator}{dates_formatted_string}.{file_extension}"
                                        ));
                                    } else {
                                        file.target
                                            .set_file_name(format!("{file_stem}{separator}{dates_formatted_string}"));
                                    }
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

                // TODO: Update to use a 24 hour clock, and include am and pm automatically
                // TODO: account for <BLANK> edge case
                Task::Time {
                    at_start,
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
                                let datetime_hour = format!("{:02}", datetime.hour());
                                let datetime_minute = format!("{:02}", datetime.minute());
                                let datetime_second = format!("{:02}", datetime.second());

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

                                if *at_start {
                                    if let Some(t) = file.target.extension() {
                                        file_extension = t.to_string_lossy().to_string();
                                        file.target.set_file_name(format!(
                                        "{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}{separator}{file_stem}.{file_extension}"
                                    ));
                                    } else {
                                        file.target.set_file_name(format!(
                                        "{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}{separator}{file_stem}"
                                    ));
                                    }
                                } else {
                                    if let Some(t) = file.target.extension() {
                                        file_extension = t.to_string_lossy().to_string();
                                        file.target.set_file_name(format!(
                                        "{file_stem}{separator}{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}.{file_extension}"
                                    ));
                                    } else {
                                        file.target.set_file_name(format!(
                                        "{file_stem}{separator}{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}"
                                    ));
                                    }
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

fn resolve_workingfile_duplicates(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();

    state.working_files = state
        .working_files
        .drain(..)
        .scan(HashMap::<PathBuf, usize>::new(), |seen, working_file| {
            let count = seen.entry(working_file.target.clone()).or_insert(0);
            let new_target = match *count {
                0 => working_file.target.clone(),
                n => {
                    let (parent, stem, extension) = (
                        working_file.target.parent(),
                        working_file.target.file_stem().unwrap_or_default().to_string_lossy(),
                        working_file.target.extension().map(|e| e.to_string_lossy()),
                    );

                    let new_filename = match extension {
                        Some(ext) => format!("{}_{:04}.{}", stem, n, ext),
                        None => format!("{}_{:04}", stem, n),
                    };

                    match parent {
                        Some(p) => p.join(new_filename),     // Move happens here
                        None => PathBuf::from(new_filename), // Move happens here
                    }
                }
            };
            *count += 1;
            Some(WorkingFile {
                source: working_file.source,
                target: new_target,
                active: working_file.active,
            })
        })
        .collect();
}

fn convert_working_files_to_file_status(state: &State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    let state = state.lock().unwrap();
    let search_is_empty = state.search.is_empty();
    let search_term = &state.search;
    let mut file_statuses: Vec<FileStatus> = Vec::with_capacity(state.working_files.len());
    if search_is_empty {
        for working_file in &state.working_files {
            let file_status = FileStatus {
                old_file_name: working_file.source.file_name().unwrap().to_string_lossy().into_owned(),
                new_file_name: working_file.target.file_name().unwrap().to_string_lossy().into_owned(),
            };
            file_statuses.push(file_status);
        }
        file_statuses
    } else {
        for working_file in &state.working_files {
            let source = working_file
                .source
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains(search_term);
            let target = working_file
                .target
                .file_name()
                .unwrap()
                .to_string_lossy()
                .contains(search_term);

            if source || target {
                let file_status = FileStatus {
                    old_file_name: working_file.source.file_name().unwrap().to_string_lossy().into_owned(),
                    new_file_name: working_file.target.file_name().unwrap().to_string_lossy().into_owned(),
                };
                file_statuses.push(file_status);
            }
        }
        file_statuses
    }
}

#[tauri::command]
async fn user_rename_files(
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<Vec<FileStatus>, Vec<FileStatus>> {
    let existing_file_status = convert_working_files_to_file_status(&state);
    // Extract the needed data from the locked state without holding the lock across await points
    let (tasks_empty, files_empty) = {
        let state_guard = state.lock().unwrap();
        (state_guard.tasks.is_empty(), state_guard.file_names.is_empty())
    }; // MutexGuard is dropped here

    // if there are tasks but no files selected
    if !tasks_empty && files_empty {
        match AsyncMessageDialog::new()
            .set_title("Warning")
            .set_description(format!("Please add files first"))
            .set_buttons(rfd::MessageButtons::Ok)
            .show()
            .await
        {
            MessageDialogResult::Ok => Ok(existing_file_status),
            _ => Err(existing_file_status),
        }
    }
    // if there aren't tasks but there are files
    else if tasks_empty && !files_empty {
        match AsyncMessageDialog::new()
            .set_title("Warning")
            .set_description(format!("Please add file tasks first"))
            .set_buttons(rfd::MessageButtons::Ok)
            .show()
            .await
        {
            MessageDialogResult::Ok => Ok(existing_file_status),
            _ => Err(existing_file_status),
        }
    }
    // if there aren't tasks or files
    else if tasks_empty && files_empty {
        match AsyncMessageDialog::new()
            .set_title("Warning")
            .set_description(format!("Please add files and tasks first"))
            .set_buttons(rfd::MessageButtons::Ok)
            .show()
            .await
        {
            MessageDialogResult::Ok => Ok(existing_file_status),
            _ => Err(existing_file_status),
        }
    }
    // if there are tasks and files
    else {
        // create sender and receiver channels for closure

        match AsyncMessageDialog::new()
            .set_title("Title")
            .set_description(format!("Are you sure you want to rename these files?"))
            .set_buttons(rfd::MessageButtons::OkCancel)
            .show()
            .await
        {
            MessageDialogResult::Ok => {
                let mut state = state.lock().unwrap();
                for file in &mut state.working_files {
                    let rename_result = rename(&file.source, &file.target);

                    if let Err(t) = rename_result {
                        println!("{t}");
                    }
                }
                state.file_names.clear();
                state.working_files.clear();
                let blank_file_status: Vec<FileStatus> = vec![];

                app.notification()
                    .builder()
                    .title("Success")
                    .body("Files converted successfully")
                    .show()
                    .unwrap();

                Ok(blank_file_status)
            }
            MessageDialogResult::Cancel => Err(existing_file_status),
            _ => Err(existing_file_status),
        }
    }
}

// NOTE: This notification works on Arch Linux, and Win 11
// TODO: Confirm that this works on MacOS.
// TODO: If all platforms work, we need to properly do error handling.
#[tauri::command]
fn user_notification(_app: tauri::AppHandle) -> String {
    // Notification::new()
    //     .summary("xxx")
    //     .body("xxx")
    //     .icon("firefox")
    //     .show()
    //     .unwrap();

    // this is the standard tauri notification that isn't working on macos
    // app.notification()
    //     .builder()
    //     .title("Notification")
    //     .body("This is a test notification")
    //     .show()
    //     .unwrap();

    // return my_line;

    return "test return string".to_string();
}

#[tauri::command]
async fn user_dialog() -> Result<String, String> {
    let outer: String = "outergood".to_string();

    match AsyncMessageDialog::new()
        .set_title("Dialog")
        .set_description(format!("This is a test dialog."))
        .set_buttons(rfd::MessageButtons::OkCancel)
        .show()
        .await
    {
        MessageDialogResult::Ok => Ok({
            let inner = "innergood".to_string();
            format!(
                "outer:{}, inner:{}. Hello, you've been greeted from Rust!",
                outer, inner
            )
        }),
        MessageDialogResult::Cancel => Ok("User clicked cancel".to_string()),
        _ => Err("Shit hit the fan".to_string()),
    }
}
