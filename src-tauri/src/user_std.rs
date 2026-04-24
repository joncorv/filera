use crate::process_tasks::process_tasks_on_working_files;
use crate::{AppState, FileStatus, HashMap, Mutex, Path, PathBuf, State, Task, WorkingFile};

use crate::atomics::{
    convert_file_names_to_working_files, convert_working_files_to_file_status, resolve_workingfile_duplicates, solve_duplicates,
    sort_file_names, state_update_search, state_update_sort, state_update_tasks,
};

use notify_rust::Notification;
use rfd::{AsyncMessageDialog, MessageDialogResult};
// use std::time::SystemTime;
use std::{fs::copy, fs::rename};
use tauri_plugin_notification::NotificationExt;
// use time::format_description::well_known::Iso8601;
// use time::OffsetDateTime;
use walkdir::WalkDir;

#[tauri::command]
pub fn user_open_files(file_names: Vec<String>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    solve_duplicates(file_names, &state);
    sort_file_names(&state);
    convert_file_names_to_working_files(&state);
    process_tasks_on_working_files(&state);
    resolve_workingfile_duplicates(&state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
pub fn user_open_folders(directories: Vec<String>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    let mut file_names: Vec<String> = Vec::new();

    for dir in directories {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            file_names.push(entry.path().to_string_lossy().to_string());
        }
    }

    solve_duplicates(file_names, &state);
    sort_file_names(&state);
    convert_file_names_to_working_files(&state);
    process_tasks_on_working_files(&state);
    resolve_workingfile_duplicates(&state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
pub fn user_dragdrop_files(files: Vec<String>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    let mut file_names: Vec<String> = Vec::new();

    for file in files {
        let current_file = PathBuf::from(file.clone());

        if current_file.is_file() {
            file_names.push(file);
        } else if current_file.is_dir() {
            for entry in WalkDir::new(current_file).into_iter().filter_map(|e| e.ok()) {
                file_names.push(entry.path().to_string_lossy().to_string());
            }
        }
    }

    solve_duplicates(file_names, &state);
    sort_file_names(&state);
    convert_file_names_to_working_files(&state);
    process_tasks_on_working_files(&state);
    resolve_workingfile_duplicates(&state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
pub fn user_clear_files(state: State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.file_names.clear();
    state.working_files.clear();
}

#[tauri::command]
pub fn user_update_sort(sort_choice: String, sort_ascending: bool, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    state_update_sort(sort_choice, sort_ascending, &state);
    sort_file_names(&state);
    convert_file_names_to_working_files(&state);
    process_tasks_on_working_files(&state);
    resolve_workingfile_duplicates(&state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
pub fn user_update_tasks(task_list: Vec<Task>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    state_update_tasks(task_list, &state);
    process_tasks_on_working_files(&state);
    resolve_workingfile_duplicates(&state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
pub fn user_update_search(search: String, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    state_update_search(search, &state);
    convert_working_files_to_file_status(&state)
}

#[tauri::command]
pub async fn user_rename_files(
    output_dropdown_choice: &str,
    output_directory: &str,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<Vec<FileStatus>, Vec<FileStatus>> {
    // const REPLACE: &str = "replace";
    const COPY: &str = "copy";
    const MOVE: &str = "move";

    println!("output dropdown choice= {output_dropdown_choice}");
    println!("output directory= {output_directory}");
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
    // then we will rename, copy or move the files
    else {
        match AsyncMessageDialog::new()
            .set_title("Title")
            .set_description(format!("Are you sure you want to rename these files?"))
            .set_buttons(rfd::MessageButtons::OkCancel)
            .show()
            .await
        {
            MessageDialogResult::Ok => {
                match output_dropdown_choice {
                    COPY => {
                        // here is our copy commands
                        // no longer uses fs::rename, instead uses a copy, with a new parent
                        // directory
                        let mut state = state.lock().unwrap();
                        for file in &mut state.working_files {
                            if file.active {
                                let temp_filename = file.target.file_name();

                                if let Some(t) = temp_filename {
                                    file.target = PathBuf::from(output_directory).join(t);

                                    let rename_result = copy(&file.source, &file.target);

                                    if let Err(t) = rename_result {
                                        println!("{t}");
                                    }
                                }
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
                    MOVE => {
                        // here is our move commands
                        // same rename function, but each file needs new parent directory
                        let mut state = state.lock().unwrap();
                        for file in &mut state.working_files {
                            if file.active {
                                let temp_filename = file.target.file_name();

                                if let Some(t) = temp_filename {
                                    file.target = PathBuf::from(output_directory).join(t);

                                    let rename_result = rename(&file.source, &file.target);

                                    if let Err(t) = rename_result {
                                        println!("{t}");
                                    }
                                }
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
                    _ => {
                        // here is our replace commands
                        let mut state = state.lock().unwrap();
                        for file in &mut state.working_files {
                            if file.active {
                                let rename_result = rename(&file.source, &file.target);

                                if let Err(t) = rename_result {
                                    println!("{t}");
                                }
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
                }
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
pub fn user_notification(_app: tauri::AppHandle) -> String {
    Notification::new().summary("xxx").body("xxx").icon("firefox").show().unwrap();

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
pub async fn user_dialog() -> Result<String, String> {
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
            format!("outer:{}, inner:{}. Hello, you've been greeted from Rust!", outer, inner)
        }),
        MessageDialogResult::Cancel => Ok("User clicked cancel".to_string()),
        _ => Err("Shit hit the fan".to_string()),
    }
}
