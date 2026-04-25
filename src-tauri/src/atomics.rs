use crate::{AppState, FileStatus, HashMap, Mutex, Path, PathBuf, State, Task, WorkingFile};
use std::time::SystemTime;
// use notify_rust::Notification;
// use rfd::{AsyncMessageDialog, MessageDialogResult};
// use std::{fs::copy, fs::rename};
// use tauri_plugin_notification::NotificationExt;
// use time::format_description::well_known::Iso8601;
// use time::OffsetDateTime;
// use walkdir::WalkDir;

#[tauri::command]
pub fn solve_duplicates(file_names: Vec<String>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut file_names = file_names;
    state.file_names.append(&mut file_names);
    state.file_names.sort_unstable();
    state
        .file_names
        .sort_by(|a, b| natord::compare(&a.to_lowercase(), &b.to_lowercase()));
    state.file_names.dedup();
}

pub fn sort_file_names(state: &State<'_, Mutex<AppState>>) {
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
                    let meta_data = std::fs::metadata(file).ok()?.modified().unwrap_or(SystemTime::UNIX_EPOCH);
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
                    let meta_data = std::fs::metadata(file).ok()?.created().unwrap_or(SystemTime::UNIX_EPOCH);
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

    if !state.sort_ascending {
        state.file_names_sorted.reverse();
    }
}

pub fn state_update_sort(sort_choice: String, sort_ascending: bool, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.sort_choice = sort_choice;
    state.sort_ascending = sort_ascending;
}

#[tauri::command]
pub fn state_update_tasks(task_list: Vec<Task>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.tasks = task_list.clone();
}

#[tauri::command]
pub fn state_update_search(search: String, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.search = search;
}

#[tauri::command]
pub fn convert_file_names_to_working_files(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut new_working_files: Vec<WorkingFile> = Vec::with_capacity(state.file_names.len());
    let file_paths = &state.file_names_sorted;

    for file_path in file_paths {
        let working_file = WorkingFile {
            source: PathBuf::from(file_path.clone()),
            target: PathBuf::new(),
            active: true,
        };

        let is_file = working_file.source.is_file();

        let is_legal_file;

        if file_path.contains(".DS_Store") {
            is_legal_file = false;
        } else {
            is_legal_file = true;
        }

        if is_file && is_legal_file {
            new_working_files.push(working_file);
        }
    }
    state.working_files = new_working_files;
}

// fn state_update_target_directory() {}

pub fn resolve_workingfile_duplicates(state: &State<'_, Mutex<AppState>>) {
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

pub fn convert_working_files_to_file_status(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let search_is_empty = state.search.is_empty();
    let search_term = &state.search;
    let mut file_statuses: Vec<FileStatus> = Vec::with_capacity(state.working_files.len());
    // let selected_filestatuses = state.selected_filestatuses;
    if search_is_empty {
        for working_file in &state.working_files {
            let file_status = FileStatus {
                old_file_name: working_file.source.file_name().unwrap().to_string_lossy().into_owned(),
                new_file_name: working_file.target.file_name().unwrap().to_string_lossy().into_owned(),
                active: working_file.active,
                selected: false,
            };
            file_statuses.push(file_status);
        }
        state.file_statuses = file_statuses;
    } else {
        for working_file in &state.working_files {
            let source = working_file.source.file_name().unwrap().to_string_lossy().contains(search_term);
            let target = working_file.target.file_name().unwrap().to_string_lossy().contains(search_term);

            if source || target {
                let file_status = FileStatus {
                    old_file_name: working_file.source.file_name().unwrap().to_string_lossy().into_owned(),
                    new_file_name: working_file.target.file_name().unwrap().to_string_lossy().into_owned(),
                    active: working_file.active,
                    selected: false,
                };
                file_statuses.push(file_status);
            }
        }
        state.file_statuses = file_statuses;
    }
}

pub fn apply_selections_to_filestatuses(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let selected = state.selected_filestatuses.clone();
    let file_statuses = &mut state.file_statuses;

    if let Some(selected) = selected {
        selected.iter().for_each(|selected_index| {
            if let Some(filestatus) = file_statuses.get_mut(*selected_index) {
                filestatus.selected = true;
            }
        })
    }
}

pub fn apply_search_to_filestatuses(state: &State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    let state = state.lock().unwrap();
    let search_term = state.search.clone();
    let search_is_empty = search_term.is_empty();

    if search_is_empty {
        return state.file_statuses.clone();
    } else {
        let file_statuses_filtered = state.file_statuses.clone();

        let _ = file_statuses_filtered.iter().filter(|file_status| {
            let old = file_status.old_file_name.contains(&search_term);
            let new = file_status.new_file_name.contains(&search_term);

            return old && new;
        });

        return file_statuses_filtered;
    }
}
