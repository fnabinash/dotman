use std::{fs, path::{self, PathBuf}};
use std::process::{Stdio, Command};
use std::io::Write;

use adof::{get_adof_dir, get_home_dir};

pub mod add;
pub mod init;

fn select_files(found_files: Vec<PathBuf>) -> Vec<String> {
    let found_files = found_files
        .iter()
        .map(|file| file.clone().into_os_string().into_string().unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut child = Command::new("fzf")
        .arg("--preview")
        .arg("cat {}")
        .arg("-m")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start fzf");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(found_files.as_bytes())
            .expect("Failed to write to fzf stdin");
    }

    let output = child.wait_with_output().expect("Failed to read fzf output");

    let selected_files = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()
        .lines()
        .map(|file| file.to_string())
        .collect::<Vec<String>>();

    if selected_files.is_empty() {
        println!("No file selected.");
    }

    selected_files
}

fn create_backup_file(original_file: &str) -> String {
    let home_dir = get_home_dir();
    let adof_dir = get_adof_dir();

    let backup_file = original_file.replace(&home_dir, &adof_dir);

    let path = path::Path::new(&backup_file);
    let path_dir = path.parent().unwrap();

    fs::create_dir_all(path_dir).unwrap();
    fs::File::create(&backup_file).unwrap();

    backup_file
}
