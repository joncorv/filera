use filera_lib::{AppState, Mutex};
use tauri::test::{MockRuntime, mock_app};
use tauri::{App, Manager, State};
use tempfile::TempDir;

use super::*;

pub fn create_mock_app_state() -> State<'static, Mutex<AppState>> {
    let app = mock_app();
    app.manage(Mutex::new(AppState::default()));

    let app: &'static App<MockRuntime> = Box::leak(Box::new(app));
    app.state::<Mutex<AppState>>()
}

// pub fn create_basic_tasks() -> Vec<Task> {
//     vec![
//         Task::FindAndReplace {
//             find_text: "_".to_string(),
//             replace_text: "-".to_string(),
//             active: true,
//         },
//         Task::ChangeCase {
//             case_choice: 0, // 0 = lowercase, 1 = uppercase
//             active: true,
//         },
//         Task::CustomText {
//             text: "archive-".to_string(),
//             at_start: true,
//             active: true,
//         },
//         Task::NumSequence {
//             start_num: 1,
//             num_padding: 3,
//             at_start: false,
//             separator: "-".to_string(),
//             active: true,
//         },
//         Task::FilterName {
//             inclusive: true,
//             name: "draft".to_string(),
//         },
//     ]
// }

pub fn create_tempfiles(dir: &TempDir, files: &Vec<&str>) -> Result<(), Error> {
    for file in files {
        let file_path = dir.path().join(*file);
        let mut tmp_file = File::create(&file_path)?;
        writeln!(tmp_file, "This is the body of the temp file.")?;
        println!("created: {file:?}");
        assert!(&file_path.exists());
    }

    Ok(())
}
