use crate::{AppState, FileStatus, FileStatusResponse, FileStatusStats, HashMap, Mutex, Path, PathBuf, State, Task, WorkingFile};
pub use std::time::SystemTime;
// use anyhow::Error;
// use anyhow::Result;
// use anyhow::anyhow;

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
            state
                .file_names
                .sort_by(|a, b| natord::compare(&a.to_lowercase(), &b.to_lowercase()));
        }
        "modified" => {
            state.file_names.sort_by(|a, b| {
                let a_meta = if let Ok(ok_a_metadata) = std::fs::metadata(a) {
                    ok_a_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)
                } else {
                    SystemTime::UNIX_EPOCH
                };
                let b_meta = if let Ok(ok_b_metadata) = std::fs::metadata(b) {
                    ok_b_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)
                } else {
                    SystemTime::UNIX_EPOCH
                };
                a_meta.cmp(&b_meta)
            });
        }
        "created" => {
            state.file_names.sort_by(|a, b| {
                let a_meta = if let Ok(ok_a_metadata) = std::fs::metadata(a) {
                    ok_a_metadata.created().unwrap_or(SystemTime::UNIX_EPOCH)
                } else {
                    SystemTime::UNIX_EPOCH
                };
                let b_meta = if let Ok(ok_b_metadata) = std::fs::metadata(b) {
                    ok_b_metadata.created().unwrap_or(SystemTime::UNIX_EPOCH)
                } else {
                    SystemTime::UNIX_EPOCH
                };
                a_meta.cmp(&b_meta)
            });
        }
        "size" => {
            state.file_names.sort_by(|a, b| {
                let a_meta = if let Ok(ok_a_metadata) = std::fs::metadata(a) {
                    ok_a_metadata.len()
                } else {
                    0
                };
                let b_meta = if let Ok(ok_b_metadata) = std::fs::metadata(b) {
                    ok_b_metadata.len()
                } else {
                    0
                };
                a_meta.cmp(&b_meta)
            });
        }
        "type" => {
            state.file_names.sort_by(|a, b| {
                let a_ext = Path::new(a)
                    .extension()
                    .map_or_else(|| ".zzz".to_string(), |ext| ext.to_string_lossy().to_string());
                let b_ext = Path::new(b)
                    .extension()
                    .map_or_else(|| ".zzz".to_string(), |ext| ext.to_string_lossy().to_string());

                a_ext.cmp(&b_ext)
            });
        }
        _ => {
            state
                .file_names
                .sort_by(|a, b| natord::compare(&a.to_lowercase(), &b.to_lowercase()));
        }
    }

    if state.sort_descending {
        state.file_names.reverse();
    }
}

use std::collections::HashSet;
pub fn check_file_permissions(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    // let mut found_errors = Vec::new();
    let mut problem_files: HashSet<String> = HashSet::new();

    {
        for file in state.file_names.as_slice() {
            match std::fs::metadata(file) {
                // we can access metadata
                Ok(data) => {
                    // but file is read-only
                    if data.permissions().readonly() {
                        let error = format!("Error: Read-only file: {}", file);
                        println!("{}", error.clone());
                        // found_errors.push(error);
                        problem_files.insert(file.clone());
                    }
                }
                // unable to access file metadata
                Err(e) => {
                    let error = format!("Error: {:?}: {}", e.to_string(), file);
                    println!("{}", error.clone());
                    // found_errors.push(error);
                    problem_files.insert(file.clone());
                }
            }
        }
    }

    // if problem_files are found
    if !problem_files.is_empty() {
        // let error_result = found_errors.join("\n");
        let dialog = rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Your user account does not have permission to edit these files")
            .set_description("Remove them from your processing list?")
            .set_buttons(rfd::MessageButtons::YesNo)
            .show();

        match dialog {
            rfd::MessageDialogResult::Yes => {
                state.file_names.retain(|file| !problem_files.contains(file));
            }
            _ => {
                println!("User decides to keep in files they don't have permission to edit 🤷");
            }
        }
    }
}

pub fn state_update_sort(sort_choice: String, sort_descending: bool, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.sort_choice = sort_choice;
    state.sort_descending = sort_descending;
    state.selected_filestatus_anchor = None;
    state.selected_filestatuses = None;
}

#[tauri::command]
pub fn state_update_tasks(task_list: Vec<Task>, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.tasks = task_list;
}

#[tauri::command]
pub fn state_update_search(search: String, state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.search = search;
    state.selected_filestatus_anchor = None;
}

pub fn state_clear_selected_filestatuses(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.selected_filestatuses = None;
    state.selected_filestatus_anchor = None;
}

#[tauri::command]
pub fn convert_file_names_to_working_files(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut new_working_files: Vec<WorkingFile> = Vec::with_capacity(state.file_names.len());
    let file_paths = &state.file_names;

    for (index, file_path) in file_paths.iter().enumerate() {
        let working_file = WorkingFile {
            source: PathBuf::from(file_path.clone()),
            target: PathBuf::new(),
            active: true,
            stable_id: index,
        };

        let is_file = working_file.source.is_file();
        let is_legal_file = !file_path.contains(".DS_Store");

        if is_file && is_legal_file {
            new_working_files.push(working_file);
        }
    }
    state.working_files = new_working_files;
}

// fn state_update_target_directory() {}

pub fn resolve_workingfile_duplicates(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let mut hash: HashMap<&PathBuf, usize> = HashMap::new();

    for working_file in &mut state.working_files {
        // if base filename is a match
        if let Some(count) = hash.get(&working_file.target) {
            let mut iteration = count.to_owned();
            let prefix = working_file.target.file_prefix().unwrap_or_default().to_string_lossy();
            let extension = working_file
                .target
                .extension()
                .map_or_default(|ext| format!(".{}", ext.to_string_lossy()));

            // need to operate on dummy
            let mut target_dummy = working_file.target.clone();

            // iterate until target_dummy is not found in hash
            'filename_search: loop {
                let new_filename = format!("{}{:04}{}", prefix, iteration, extension,);
                target_dummy.set_file_name(new_filename);

                if !hash.contains_key(&target_dummy) {
                    break 'filename_search;
                }
                iteration += 1;
            }

            working_file.target = target_dummy;
            hash.insert(&working_file.target, iteration);
        } else {
            hash.insert(&working_file.target, 1);
        }
    }
}

pub fn convert_working_files_to_file_status(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let state = &mut *state;
    let mut file_statuses: Vec<FileStatus> = Vec::with_capacity(state.working_files.len());
    let mut filtered_count: usize = 0;

    for (index, working_file) in state.working_files.iter().enumerate() {
        if !working_file.active {
            filtered_count += 1;
        }
        let file_status = FileStatus {
            old_file_name: working_file.source.file_name().unwrap().to_string_lossy().into_owned(),
            new_file_name: working_file.target.file_name().unwrap().to_string_lossy().into_owned(),
            active: working_file.active,
            selected: false,
            stable_id: index,
        };
        file_statuses.push(file_status);
    }
    state.file_statuses = file_statuses;
    state.filtered_count = filtered_count;
}

pub fn apply_selections_to_filestatuses(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let selected = state.selected_filestatuses.clone();
    let file_statuses = &mut state.file_statuses;

    if let Some(selected) = selected {
        file_statuses.iter_mut().enumerate().for_each(|(index, file_status)| {
            file_status.selected = selected.contains(&index);
        });
    }
}

pub fn apply_search(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let state = &mut *state;

    if state.search.is_empty() {
        state.filtered_filestatus_indices = None;
    } else {
        let search = state.search.clone();
        state.filtered_filestatus_indices = Some(
            state
                .file_statuses
                .iter()
                .enumerate()
                .filter_map(|(index, file_status)| {
                    let old = file_status.old_file_name.contains(&search);
                    let new = file_status.new_file_name.contains(&search);
                    (old || new).then_some(index)
                })
                .collect(),
        );
    };
}

pub fn build_response(state: &State<'_, Mutex<AppState>>) -> FileStatusResponse {
    let state = state.lock().unwrap();

    let statuses = if let Some(indices) = &state.filtered_filestatus_indices {
        indices
            .iter()
            .map(|index| {
                state
                    .file_statuses
                    .get(*index)
                    .cloned()
                    .expect("filtered filestatus index should point to an existing file_status")
            })
            .collect()
    } else {
        state.file_statuses.clone()
    };

    FileStatusResponse {
        statuses,
        stats: FileStatusStats {
            total: state.file_statuses.len(),
            selected: state.selected_filestatuses.as_ref().map_or(0, |s| s.len()),
            filtered: state.filtered_count,
            ready: state.file_statuses.len() - state.filtered_count,
        },
    }
}
