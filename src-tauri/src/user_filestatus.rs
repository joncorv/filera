use crate::{AppState, FileStatus, HashSet, Mutex, State};

use crate::atomics::{
    apply_search_to_filestatuses, apply_selections_to_filestatuses, convert_file_names_to_working_files,
    convert_working_files_to_file_status, resolve_workingfile_duplicates, solve_duplicates, sort_file_names, state_update_search,
    state_update_sort, state_update_tasks,
};

#[tauri::command]
pub fn user_filestatus_click(index: usize, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
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
    apply_search_to_filestatuses(&state)
}

#[tauri::command]
pub fn user_filestatus_ctrl_click(index: usize, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
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
    apply_search_to_filestatuses(&state)
}

#[tauri::command]
pub fn user_filestatus_shift_click() {}

#[tauri::command]
pub fn user_filestatus_selection_clear() {}

#[tauri::command]
pub fn user_filestatus_delete() {}

// #[tauri::command]
// pub fn apply_selections_to_filestatuses(state: &State<'_, Mutex<AppState>>) {
//     let mut state = state.lock().unwrap();
//     let selected = state.selected_filestatuses.clone();
//
//     if let Some(selected) = selected {
//         if let Some(file_statuses) = &mut state.file_statuses {
//             selected.iter().for_each(|selected_index| {
//                 if let Some(foo) = file_statuses.get_mut(*selected_index) {
//                     foo.selected = true;
//                 }
//             })
//         }
//     }
// }

// pub fn user_open_files(file_names: Vec<String>, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
//     solve_duplicates(file_names, &state);
//     sort_file_names(&state);
//     convert_file_names_to_working_files(&state);
//     process_tasks_on_working_files(&state);
//     resolve_workingfile_duplicates(&state);
//     convert_working_files_to_file_status(&state);
//     apply_selections_to_filestatuses(&state);
//     apply_search_to_filestatuses(&state)
// }
