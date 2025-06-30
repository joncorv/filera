use notify_rust::Notification;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct WorkingFile {
    path: String,
    old_file_name: String,
    new_file_name: String,
    active: bool,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
enum Task {
    Prefix {
        text: String,
        active: bool,
    },
    Postfix{
        text: String,
        active: bool,
    },
    FindAndReplace {
        find_text: String,
        replace_text: String,
        active: bool,
    }

}


// App Entry Point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, print_something_to_console, open_files, test_working_file])
        .setup(|app| {
            use tauri_plugin_notification::NotificationExt;
            app.notification()
                .builder()
                .title("Tauri")
                .body("Tauri is awesome")
                .show()
                .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// The most simple command
#[tauri::command]
fn greet(name: &str) -> String {
    if name == "" {
        "Hey you need to put in your name!".to_string()
    } else {
        format!("Hello, {}! Greetings from the Rust backend", name)
    }
}

#[tauri::command]
fn open_files(file_names: Vec<String>, task_list: Vec<Task>) -> Vec<WorkingFile> {
    use std::path::Path;
    println!("{:?}", &task_list);

    // let mut all_paths: Vec<String> = Vec::new();
    let mut all_paths: Vec<String> = file_names;
    all_paths.sort_unstable();
    all_paths.dedup();

    let mut all_working_files: Vec<WorkingFile> = Vec::new();

    for file in all_paths {
        let path_object = Path::new(&file).file_name().unwrap().to_str().unwrap();

        let working_workingfile = WorkingFile{
            path: file.to_string(),
            old_file_name: path_object.to_string(),
            new_file_name: path_object.to_string(),
            active: true,
        };
        all_working_files.push(working_workingfile);
    };

    for file in &mut all_working_files{
        for task in &task_list {
            match task {
                Task::Prefix { text, active } => {
                    if *active {
                        file.new_file_name = format!("{}{}", text, file.new_file_name);
                    };
                },
                Task::Postfix { text, active } => {
                    if *active{
                        file.new_file_name = format!("{}{}", file.new_file_name, text);
                    };
                },
                Task::FindAndReplace { find_text, replace_text, active } => {
                    if *active {
                        file.new_file_name = file.new_file_name.replace(find_text, replace_text);
                    }
                },
            }
        }
    };

    all_working_files
}

#[tauri::command]
fn test_working_file(working_file: WorkingFile) -> WorkingFile {
    // let incoming_info = format!("well this is what we got: {:?}", &working_file);
    working_file

}


// This should print to the fucking console but it doesn't.
#[tauri::command]
fn print_something_to_console(console_title: &str, console_text: &str) -> String {
    let my_line: String = format!("{}\n{}", console_title, console_text);
    println!("{}", &my_line);

    Notification::new()
        .summary("This is the summary")
        .body("This is the body")
        .icon("firefox")
        .show()
        .unwrap();
    return my_line;
}
