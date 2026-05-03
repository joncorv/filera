use crate::{AppState, FileStatusResponse, HashSet, Mutex, State};

use crate::atomics::{apply_search_and_build_response, apply_selections_to_filestatuses, convert_working_files_to_file_status};

#[tauri::command]
pub fn user_filestatus_click(index: usize, state: State<'_, Mutex<AppState>>) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();
        let selected_filestatuses = state.selected_filestatuses.clone();

        if selected_filestatuses.is_none() {
            let mut new_hash: HashSet<usize> = HashSet::new();
            new_hash.insert(index);
            state.selected_filestatuses = Some(new_hash);
            state.last_selected_filestatus = Some(index);

            if let Some(filestatus) = state.file_statuses.get_mut(index) {
                filestatus.selected = true;
            } else {
                eprintln!("errror: can't set filestatus.selected on index: {index:?}");
            }
        } else if let Some(sf) = selected_filestatuses {
            sf.iter().for_each(|selection_index| {
                if let Some(filestatus) = state.file_statuses.get_mut(*selection_index) {
                    filestatus.selected = false;
                } else {
                    eprintln!("errror: can't set filestatus.selected on index: {selection_index:?}");
                }
            });

            if !sf.contains(&index) {
                if let Some(filestatus) = state.file_statuses.get_mut(index) {
                    filestatus.selected = true;
                } else {
                    eprintln!("errror: can't set filestatus.selected on index: {index:?}");
                }
                let mut new_hash: HashSet<usize> = HashSet::new();
                new_hash.insert(index);
                state.selected_filestatuses = Some(new_hash);
                state.last_selected_filestatus = Some(index);
            } else {
                state.selected_filestatuses = None;
                state.last_selected_filestatus = None;
            }
        }
    }
    apply_search_and_build_response(&state)
}

#[tauri::command]
pub fn user_filestatus_ctrl_click(index: usize, state: State<'_, Mutex<AppState>>) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();
        let selected_filestatuses = state.selected_filestatuses.clone();

        if selected_filestatuses.is_none() {
            let mut new_hash: HashSet<usize> = HashSet::new();
            new_hash.insert(index);
            state.selected_filestatuses = Some(new_hash);
            state.last_selected_filestatus = Some(index);

            if let Some(filestatus) = state.file_statuses.get_mut(index) {
                filestatus.selected = true;
            } else {
                eprintln!("errror: can't set filestatus.selected on index: {index:?}");
            }
        } else if let Some(sf) = selected_filestatuses {
            if sf.contains(&index) {
                if let Some(filestatus) = state.file_statuses.get_mut(index) {
                    filestatus.selected = false;
                    state.last_selected_filestatus = None;

                    if let Some(ssf) = &mut state.selected_filestatuses {
                        ssf.remove(&index);
                    }
                } else {
                    eprintln!("errror: can't set filestatus.selected on index: {index:?}");
                }
            } else {
                if let Some(filestatus) = state.file_statuses.get_mut(index) {
                    filestatus.selected = true;
                    state.last_selected_filestatus = Some(index);

                    if let Some(ssf) = &mut state.selected_filestatuses {
                        ssf.insert(index);
                    }
                } else {
                    eprintln!("errror: can't set filestatus.selected on index: {index:?}");
                }
            }
        }
    }
    apply_search_and_build_response(&state)
}

#[tauri::command]
pub fn user_filestatus_shift_click(index: usize, state: State<'_, Mutex<AppState>>) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();
        let selected_filestatuses = state.selected_filestatuses.clone();
        let last_selected_filestatus = state.last_selected_filestatus.clone();

        if selected_filestatuses.is_none() {
            let mut new_hash: HashSet<usize> = HashSet::new();
            new_hash.insert(index);
            state.selected_filestatuses = Some(new_hash);
            state.last_selected_filestatus = Some(index);

            if let Some(filestatus) = state.file_statuses.get_mut(index) {
                filestatus.selected = true;
            } else {
                eprintln!("errror: can't set filestatus.selected on index: {index:?}");
            }
        } else if let Some(sf) = selected_filestatuses {
            if let Some(ls) = last_selected_filestatus {
                let start = std::cmp::min(ls, index);
                let end = std::cmp::max(ls, index);
                (start..=end).for_each(|i| {
                    if let Some(ssf) = &mut state.selected_filestatuses {
                        ssf.insert(i);
                    }

                    if let Some(filestatus) = state.file_statuses.get_mut(i) {
                        filestatus.selected = true;
                    } else {
                        eprintln!("errror: can't set filestatus.selected on index: {i:?}");
                    }
                });
            }
        }
    }
    apply_search_and_build_response(&state)
}

#[tauri::command]
pub fn user_filestatus_selection_clear(state: State<'_, Mutex<AppState>>) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();
        if let Some(selected) = state.selected_filestatuses.take() {
            selected.iter().for_each(|i| {
                if let Some(fs) = state.file_statuses.get_mut(*i) {
                    fs.selected = false;
                }
            });
        }
        state.last_selected_filestatus = None;
    }
    apply_search_and_build_response(&state)
}

#[tauri::command]
pub fn user_filestatus_selection_delete(state: State<'_, Mutex<AppState>>) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();
        if let Some(selected) = state.selected_filestatuses.take() {
            let mut indices: Vec<usize> = selected.into_iter().collect();
            indices.sort_unstable();
            indices.into_iter().rev().for_each(|i| {
                let src = state.working_files[i].source.to_string_lossy().to_string();
                state.working_files.remove(i);
                state.file_names.retain(|path| path != &src);
            });
        }
        state.last_selected_filestatus = None;
    }
    convert_working_files_to_file_status(&state);
    apply_search_and_build_response(&state)
}
