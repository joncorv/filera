use std::fs::{rename};

use notify_rust::Notification;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct WorkingFile {
    path: String,
    old_file_name: String,
    new_file_name: String,
    active: bool,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RevertData {
    original: WorkingFile,
    changed: WorkingFile,
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
        .invoke_handler(tauri::generate_handler![greet, print_something_to_console, open_files, test_working_file, rename_files])
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

// Process files from the front end from paths to WorkingFiles
fn file_names_to_working_files(file_names: Vec<String>) -> Vec<WorkingFile> {

    // Instantiate a new Vector of Strings to work on
    let mut all_paths: Vec<String> = file_names;

    // Sort and De-Duplicate all paths
    all_paths.sort_unstable();
    all_paths.dedup();

    // Instantiate a empty vector of WorkingFile on the heap
    let mut all_working_files: Vec<WorkingFile> = Vec::new();

    // convert all paths into WorkingFiles
    for file in all_paths {
        let path_object = std::path::Path::new(&file).file_name().unwrap().to_str().unwrap();

        let working_workingfile = WorkingFile{
            path: file.to_string(),
            old_file_name: path_object.to_string(),
            new_file_name: path_object.to_string(),
            active: true,
        };
        all_working_files.push(working_workingfile);
    };

    all_working_files

}

fn process_tasks_on_working_files(mut all_working_files: Vec<WorkingFile>, task_list: Vec<Task>) -> Vec<WorkingFile> {
    // Process all WorkingFiles and apply the effects sent from the front end.

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

    return all_working_files;
    

}

#[tauri::command]
fn open_files(file_names: Vec<String>, task_list: Vec<Task>) -> Vec<WorkingFile> {
    println!("{:?}", &task_list);

    let mut all_working_files = file_names_to_working_files(file_names);
    all_working_files = process_tasks_on_working_files(all_working_files, task_list);
    return all_working_files;
}


#[tauri::command]
fn rename_files( file_names: Vec<String>, task_list: Vec<Task>) -> String  {

    // convert the file_names data to a Vec of WorkingFiles
    let mut all_working_files = file_names_to_working_files(file_names);

    all_working_files = process_tasks_on_working_files(all_working_files, task_list);
    
    for file in &all_working_files {

        let path = std::path::Path::new(&file.path);

        // I do need to wrap an error for this.
        // None() should return an empty string
        let parent = path.parent().unwrap().to_str().unwrap();
        let destination = format!("{}/{}", parent, &file.new_file_name);

        println!("We weill rename this file: {}", &file.path);
        println!("To be this file: {}", &destination);

        let rename_result = rename(&file.path, destination);
        match rename_result {
            Ok(_) => println!("File converted Successfully!"),
            Err(e) => println!("Rename Error: {}", e),
        }

    };

    return "ok this was a success".to_string();
 
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
