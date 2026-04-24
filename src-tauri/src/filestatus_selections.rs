use crate::{AppState, FileStatus, Mutex, State};

// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
// pub struct AppState {
//     file_names: Vec<String>,
//     file_names_sorted: Vec<String>,
//     working_files: Vec<WorkingFile>,
//     tasks: Vec<Task>,
//     sort_choice: String,
//     sort_ascending: bool,
//     search: String,
//     output: Output,
//     file_statuses: Option<Vec<FileStatus>>,
//     selected_filestatuses: Option<Vec<usize>>,
//     last_selected_filestatus: Option<usize>,
// }

#[tauri::command]
pub fn user_filestatus_click(selected_filename_index: usize, state: State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    let mut state = state.lock().unwrap();

    state.selected_filestatuses = Some(vec![selected_filename_index]);

    // if let Some(selected_already) = state.selected_filestatuses.as_ref() {
    //     if selected_already.contains(&selected_filename_index) {
    //         state.selected_filestatuses = None;
    //     } else {
    //         state.selected_filestatuses = Some(vec![selected_filename_index]);
    //     }
    // } else {
    //         state.selected_filestatuses = Some(vec![selected_filename_index]);
    // }

    // make the fucking compiler happy, JESUS
    let blank: Vec<FileStatus> = Vec::new();
    return blank;
}

#[tauri::command]
pub fn user_filestatus_ctrl_click() {}

#[tauri::command]
pub fn user_filestatus_shift_click() {}

#[tauri::command]
pub fn user_filestatus_selection_clear() {}

#[tauri::command]
pub fn user_filestatus_delete() {}

#[tauri::command]
pub fn apply_selections_to_filestatuses(state: &State<'_, Mutex<AppState>>) -> Vec<FileStatus> {
    let state = state.lock().unwrap();
    // let file_statuses = state.file_statuses;

    state.file_statuses.expect("something")
}
