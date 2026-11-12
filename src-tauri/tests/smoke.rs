use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use filera_lib::Task;
use filera_lib::atomics::state_update_tasks;
use filera_lib::user_std::user_dragdrop_files;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
mod common;

#[test]
fn simple_sort_files() -> Result<(), Error> {
    let app_state = common::create_mock_app_state();
    {
        let task_list = vec![
            Task::FindAndReplace {
                find_text: "_".to_string(),
                replace_text: "-".to_string(),
                active: true,
            },
            Task::ChangeCase {
                case_choice: 0, // 0 = lowercase, 1 = uppercase
                active: true,
            },
            Task::CustomText {
                text: "test-".to_string(),
                at_start: true,
                active: true,
            },
            Task::CustomText {
                text: "-end".to_string(),
                at_start: false,
                active: true,
            },
        ];
        state_update_tasks(task_list, &app_state);
    }

    let directory = TempDir::new()?;
    let files = vec!["FILE_005.txt", "FILE_004.txt", "file_003.txt", "file_002.txt", "file_001.txt"];
    common::create_tempfiles(&directory, &files)?;
    let image_directory = vec![directory.path().to_string_lossy().to_string()];
    let file_result = user_dragdrop_files(image_directory, app_state);
    let expected_files = vec![
        "test-file-001-end.txt".to_string(),
        "test-file-002-end.txt".to_string(),
        "test-file-003-end.txt".to_string(),
        "test-file-004-end.txt".to_string(),
        "test-file-005-end.txt".to_string(),
    ];
    let mut actual_files = Vec::new();
    for result in file_result.statuses {
        actual_files.push(result.new_file_name.clone());
    }
    assert_eq!(expected_files, actual_files);
    Ok(())
}

#[test]
fn tempfile_framework() -> Result<(), Error> {
    let tmp_dir = TempDir::new()?;
    let files = vec!["file_005.txt", "file_004.txt", "file_003.txt", "file_002.txt", "file_001.txt"];

    for file in files {
        let file_path = tmp_dir.path().join(file);
        let mut tmp_file = File::create(&file_path)?;
        writeln!(tmp_file, "This is the body of the temp file.")?;
        println!("created: {file:?}");

        assert!(&file_path.exists());
    }

    Ok(())
}

#[test]
fn check_mock_app_state() {
    let app_state = common::create_mock_app_state();
    let state = app_state.lock();

    assert!(state.is_ok());
    println!("App state works!");
}
