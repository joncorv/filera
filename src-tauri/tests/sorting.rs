use filera_lib::atomics::sort_file_names;
use std::fs::File;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;
mod common;

#[test]
fn name_sort_uses_filename_not_path() {
    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec!["/b/apple.txt".into(), "/a/zebra.txt".into()];
        s.sort_choice = "name".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(state.lock().unwrap().file_names, vec!["/b/apple.txt", "/a/zebra.txt"]);
    println!("name sort uses filename not path success!")
}

#[test]
fn name_sort_uses_filename_not_path_reversed() {
    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec!["/b/apple.txt".into(), "/a/zebra.txt".into()];
        s.sort_choice = "name".into();
        s.sort_descending = true;
    }
    sort_file_names(&state);
    assert_eq!(state.lock().unwrap().file_names, vec!["/a/zebra.txt", "/b/apple.txt"]);
    println!("name sort uses filename not path (reversed) success!")
}

#[test]
fn type_sort_puts_no_extension_last() {
    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec!["/a/README".into(), "/a/photo.jpg".into(), "/a/notes.txt".into()];
        s.sort_choice = "type".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(state.lock().unwrap().file_names, vec!["/a/photo.jpg", "/a/notes.txt", "/a/README"]);
    println!("type sort puts no extension last success!")
}

#[test]
fn size_sort_puts_largest_first() {
    let dir = TempDir::new().unwrap();
    let small = dir.path().join("small.txt");
    let medium = dir.path().join("medium.txt");
    let large = dir.path().join("large.txt");
    std::fs::write(&small, vec![0u8; 10]).unwrap();
    std::fs::write(&medium, vec![0u8; 100]).unwrap();
    std::fs::write(&large, vec![0u8; 1000]).unwrap();

    let small = small.to_string_lossy().to_string();
    let medium = medium.to_string_lossy().to_string();
    let large = large.to_string_lossy().to_string();

    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec![medium.clone(), small.clone(), large.clone()];
        s.sort_choice = "size".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(state.lock().unwrap().file_names, vec![large, medium, small]);
    println!("size sort puts largest first success!")
}

#[test]
fn size_sort_puts_smallest_first_reversed() {
    let dir = TempDir::new().unwrap();
    let small = dir.path().join("small.txt");
    let medium = dir.path().join("medium.txt");
    let large = dir.path().join("large.txt");
    std::fs::write(&small, vec![0u8; 10]).unwrap();
    std::fs::write(&medium, vec![0u8; 100]).unwrap();
    std::fs::write(&large, vec![0u8; 1000]).unwrap();

    let small = small.to_string_lossy().to_string();
    let medium = medium.to_string_lossy().to_string();
    let large = large.to_string_lossy().to_string();

    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec![medium.clone(), large.clone(), small.clone()];
        s.sort_choice = "size".into();
        s.sort_descending = true;
    }
    sort_file_names(&state);
    assert_eq!(state.lock().unwrap().file_names, vec![small, medium, large]);
    println!("size sort puts smallest first (reversed) success!")
}

#[test]
fn name_sort_uses_natural_order() {
    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec!["/a/file10.txt".into(), "/a/file1.txt".into(), "/a/file2.txt".into()];
        s.sort_choice = "name".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(
        state.lock().unwrap().file_names,
        vec!["/a/file1.txt", "/a/file2.txt", "/a/file10.txt"]
    );
    println!("name sort uses natural order success!")
}

#[test]
fn name_sort_ignores_case() {
    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec!["/a/banana.txt".into(), "/a/Cherry.txt".into(), "/a/apple.txt".into()];
        s.sort_choice = "name".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(
        state.lock().unwrap().file_names,
        vec!["/a/apple.txt", "/a/banana.txt", "/a/Cherry.txt"]
    );
    println!("name sort ignores case success!")
}

#[test]
fn type_sort_ignores_case() {
    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec!["/a/b.TXT".into(), "/a/a.png".into(), "/a/c.txt".into()];
        s.sort_choice = "type".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(state.lock().unwrap().file_names, vec!["/a/a.png", "/a/b.TXT", "/a/c.txt"]);
    println!("type sort ignores case success!")
}

#[test]
fn modified_sort_puts_oldest_first() {
    let dir = TempDir::new().unwrap();
    let new = dir.path().join("new.txt");
    let old = dir.path().join("old.txt");
    let older = dir.path().join("older.txt");
    let oldest = dir.path().join("oldest.txt");

    let now = SystemTime::now();
    File::create(&new).unwrap().set_modified(now).unwrap();
    File::create(&old).unwrap().set_modified(now - Duration::from_hours(24)).unwrap();
    File::create(&older).unwrap().set_modified(now - Duration::from_hours(48)).unwrap();
    File::create(&oldest).unwrap().set_modified(now - Duration::from_hours(72)).unwrap();

    let new = new.to_string_lossy().to_string();
    let old = old.to_string_lossy().to_string();
    let older = older.to_string_lossy().to_string();
    let oldest = oldest.to_string_lossy().to_string();

    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec![older.clone(), new.clone(), oldest.clone(), old.clone()];
        s.sort_choice = "modified".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(state.lock().unwrap().file_names, vec![oldest, older, old, new]);
    println!("modified sort puts oldest first success!")
}

#[test]
fn unknown_sort_choice_falls_back_to_name_sort() {
    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec!["/a/zebra.txt".into(), "/c/mango.txt".into(), "/b/apple.txt".into()];
        s.sort_choice = "not_a_real_choice".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(
        state.lock().unwrap().file_names,
        vec!["/b/apple.txt", "/c/mango.txt", "/a/zebra.txt"]
    );
    println!("unknown sort choice falls back to name sort success!")
}

#[test]
fn type_sort_orders_by_name_within_type() {
    let state = common::create_mock_app_state();
    {
        let mut s = state.lock().unwrap();
        s.file_names = vec!["/a/zebra.jpg".into(), "/a/notes.txt".into(), "/a/apple.jpg".into()];
        s.sort_choice = "type".into();
        s.sort_descending = false;
    }
    sort_file_names(&state);
    assert_eq!(
        state.lock().unwrap().file_names,
        vec!["/a/apple.jpg", "/a/zebra.jpg", "/a/notes.txt"]
    );
    println!("type sort orders by name within type success!")
}
