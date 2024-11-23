use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use snapshots::{capture_snapshot, hash_file, state::DeepEq};

fn create_dir_and_files<'a>() -> (PathBuf, Vec<&'static str>) {
    let dir_path = PathBuf::from("temp_dir");
    if fs::exists(&dir_path).unwrap() {
        fs::remove_dir_all(&dir_path).unwrap();
    }
    fs::create_dir(&dir_path).unwrap();
    let files = vec!["1.txt", "2.txt", "3.txt"];
    for file in files.iter() {
        let mut new_file = fs::File::create(&dir_path.join(file)).unwrap();
        let content = format!("file content for {}", file);
        new_file.write_all(content.as_bytes()).unwrap();
        new_file.flush().unwrap();
        new_file.sync_all().unwrap();
    }
    (dir_path, files)
}
fn delete_dir_and_files(dir_path: &PathBuf) -> bool {
    match fs::remove_dir_all(&dir_path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn modifiy_files(dir_path: &PathBuf) {
    let mut i = 0;
    for entry in fs::read_dir(dir_path).expect("Failed to read dir") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)
                .unwrap();
            let content = format!("Modified content for file {}.txt", i);
            file.write_all(content.as_bytes()).unwrap();
            file.sync_all().unwrap();
            file.flush().unwrap();
        }
        i += 1;
    }
}

#[test]
fn test_hash_file() {
    let current_dir_str = env!("CARGO_MANIFEST_DIR");
    let file_name = format!("{}/{}", current_dir_str, "hello.txt");
    let file_name_1 = format!("{}/{}", current_dir_str, "hello_1.txt");

    let mut file = File::create(&file_name).unwrap();
    let mut file_1 = File::create(&file_name_1).unwrap();

    let content = b"Hello, World!";
    let content_1 = b"Hello, World"; // Removed the exclamation mark
    file.write_all(content).unwrap();
    file_1.write_all(content_1).unwrap();

    let path_buf = PathBuf::from(&file_name);
    let path_buf_1 = PathBuf::from(&file_name_1);

    let hash = hash_file(&path_buf).unwrap_or("".to_string());
    let hash_1 = hash_file(&path_buf_1).unwrap_or("".to_string());

    file_1.flush().unwrap();
    file.flush().unwrap();
    file_1.sync_all().unwrap();
    file.sync_all().unwrap();

    fs::remove_file(&file_name).unwrap();
    fs::remove_file(&file_name_1).unwrap();

    let hello_world_file_hash = "288a86a79f20a3d6dccdca7713beaed178798296bdfa7913fa2a62d9727bf8f8";
    assert_eq!(hash, hello_world_file_hash);
    assert_ne!(hash_1, hello_world_file_hash);
}

#[test]
fn test_capture_snapshot() {
    let (dir_path, files) = create_dir_and_files();

    // ensures that files and folders are created
    let dir_exist = fs::exists(&dir_path).unwrap();
    for file in files.iter() {
        let file_exist = fs::exists(&dir_path.join(file)).unwrap();
        assert!(file_exist);
    }
    assert!(dir_exist);

    let snapshot = capture_snapshot(&dir_path);
    let without_modified_snapshot = capture_snapshot(&dir_path);

    // compares file content
    assert!(snapshot == without_modified_snapshot);

    // compares file content, id and timestamp of snapshot
    assert!(!snapshot.deep_eq(&without_modified_snapshot));

    modifiy_files(&dir_path);
    let snapshot_2 = capture_snapshot(&dir_path);

    // compares file content
    assert!(snapshot != snapshot_2);
    delete_dir_and_files(&dir_path);
}
