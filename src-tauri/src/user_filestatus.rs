use crate::{AppState, FileStatusResponse, HashSet, Mutex, SelectedFileStatusAnchor, State};

use crate::atomics::{apply_search, build_response, convert_working_files_to_file_status};

fn set_file_status_selected(state: &mut AppState, stable_id: usize, selected: bool) {
    if let Some(filestatus) = state.file_statuses.get_mut(stable_id) {
        filestatus.selected = selected;
    } else {
        eprintln!("error: can't set filestatus.selected on stable_id: {stable_id:?}");
    }
}

fn clear_selected_file_statuses(state: &mut AppState) {
    if let Some(selected) = state.selected_filestatuses.take() {
        for stable_id in selected {
            set_file_status_selected(state, stable_id, false);
        }
    }
}

fn set_single_selection(state: &mut AppState, visible_index: usize, stable_id: usize) {
    clear_selected_file_statuses(state);

    let mut selected = HashSet::new();
    selected.insert(stable_id);

    state.selected_filestatuses = Some(selected);
    state.selected_filestatus_anchor = Some(SelectedFileStatusAnchor {
        stable_id,
        visible_index,
    });
    set_file_status_selected(state, stable_id, true);
}

fn visible_range_stable_ids(state: &AppState, start: usize, end: usize) -> Option<Vec<usize>> {
    if let Some(indices) = &state.filtered_filestatus_indices {
        indices.get(start..=end).map(|range| range.to_vec())
    } else if end < state.file_statuses.len() {
        Some((start..=end).collect())
    } else {
        None
    }
}

#[tauri::command]
pub fn user_filestatus_click(
    visible_index: usize,
    stable_id: usize,
    state: State<'_, Mutex<AppState>>,
) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();
        set_single_selection(&mut state, visible_index, stable_id);
    }
    apply_search(&state);
    build_response(&state)
}

#[tauri::command]
pub fn user_filestatus_ctrl_click(
    visible_index: usize,
    stable_id: usize,
    state: State<'_, Mutex<AppState>>,
) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();
        let already_selected = state
            .selected_filestatuses
            .as_ref()
            .map_or(false, |selected| selected.contains(&stable_id));

        if already_selected {
            set_file_status_selected(&mut state, stable_id, false);

            let selection_is_empty = if let Some(selected) = &mut state.selected_filestatuses {
                selected.remove(&stable_id);
                selected.is_empty()
            } else {
                false
            };

            if selection_is_empty {
                state.selected_filestatuses = None;
            }

            if state
                .selected_filestatus_anchor
                .as_ref()
                .map_or(false, |anchor| anchor.stable_id == stable_id)
            {
                state.selected_filestatus_anchor = None;
            }
        } else {
            if state.selected_filestatuses.is_none() {
                state.selected_filestatuses = Some(HashSet::new());
            }

            if let Some(selected) = &mut state.selected_filestatuses {
                selected.insert(stable_id);
            }

            state.selected_filestatus_anchor = Some(SelectedFileStatusAnchor {
                stable_id,
                visible_index,
            });
            set_file_status_selected(&mut state, stable_id, true);
        }
    }
    apply_search(&state);
    build_response(&state)
}

#[tauri::command]
pub fn user_filestatus_shift_click(
    visible_index: usize,
    stable_id: usize,
    state: State<'_, Mutex<AppState>>,
) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();

        if state.selected_filestatuses.is_none() || state.selected_filestatus_anchor.is_none() {
            set_single_selection(&mut state, visible_index, stable_id);
        } else if let Some(anchor) = state.selected_filestatus_anchor.clone() {
            let start = anchor.visible_index.min(visible_index);
            let end = anchor.visible_index.max(visible_index);

            if let Some(stable_ids) = visible_range_stable_ids(&state, start, end) {
                if state.selected_filestatuses.is_none() {
                    state.selected_filestatuses = Some(HashSet::new());
                }

                for stable_id in stable_ids {
                    if let Some(selected) = &mut state.selected_filestatuses {
                        selected.insert(stable_id);
                    }

                    set_file_status_selected(&mut state, stable_id, true);
                }
            } else {
                eprintln!("error: visible filestatus range is out of bounds: {start:?}..={end:?}");
            }
        }
    }
    apply_search(&state);
    build_response(&state)
}

#[tauri::command]
pub fn user_filestatus_selection_clear(state: State<'_, Mutex<AppState>>) -> FileStatusResponse {
    {
        let mut state = state.lock().unwrap();
        clear_selected_file_statuses(&mut state);
        state.selected_filestatus_anchor = None;
    }
    apply_search(&state);
    build_response(&state)
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
        state.selected_filestatus_anchor = None;
    }
    convert_working_files_to_file_status(&state);
    apply_search(&state);
    build_response(&state)
}
