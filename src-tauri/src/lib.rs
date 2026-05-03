mod atomics;
mod process_tasks;
mod user_filestatus;
mod user_std;

use user_std::{
    user_clear_files, user_dialog, user_dragdrop_files, user_notification, user_open_files, user_open_folders, user_rename_files,
    user_update_search, user_update_sort, user_update_tasks,
};

use user_filestatus::{
    user_filestatus_click, user_filestatus_ctrl_click, user_filestatus_selection_clear, user_filestatus_selection_delete,
    user_filestatus_shift_click,
};

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkingFile {
    source: PathBuf,
    target: PathBuf,
    active: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileStatus {
    old_file_name: String,
    new_file_name: String,
    active: bool,
    selected: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct FileStatusStats {
    pub total: usize,
    pub selected: usize,
    pub filtered: usize,
    pub ready: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileStatusResponse {
    pub statuses: Vec<FileStatus>,
    pub stats: FileStatusStats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct AppState {
    file_names: Vec<String>,
    file_names_sorted: Vec<String>,
    working_files: Vec<WorkingFile>,
    tasks: Vec<Task>,
    sort_choice: String,
    sort_ascending: bool,
    search: String,
    output: Output,
    file_statuses: Vec<FileStatus>,
    selected_filestatuses: Option<HashSet<usize>>,
    last_selected_filestatus: Option<usize>,
    filtered_count: usize,
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
    FilterName {
        inclusive: bool,
        name: String,
    },
    FilterDocType {
        inclusive: bool,
        doc_types: Vec<String>,
    },
    FilterTimePeriod {
        inclusive: bool,
        start_time: Option<String>,
        end_time: Option<String>,
    },
    FilterTime {
        before: bool,
        time: Option<String>,
    },
    FilterSize {
        greater_than: bool,
        byte_base_size: u64,
        size: u64,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum SortMetadata {
    #[default]
    Name,
    DateCreated,
    DateModified,
    Type,
    Size,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum Output {
    #[default]
    Replace,
    Copy {
        directory: String,
    },
    Move {
        directory: String,
    },
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
            user_open_folders,
            user_update_sort,
            user_update_tasks,
            user_update_search,
            user_clear_files,
            user_update_search,
            user_rename_files,
            user_notification,
            user_dialog,
            user_dragdrop_files,
            user_filestatus_click,
            user_filestatus_ctrl_click,
            user_filestatus_shift_click,
            user_filestatus_selection_clear,
            user_filestatus_selection_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
