use std::collections::HashMap;
use std::path::Path;

struct FolderManager {
    folders: HashMap<String, bool>,
    files: HashMap<String, bool>,
}

impl FolderManager {
    pub fn new(folders: HashMap<String, bool>, files: HashMap<String, bool>) -> FolderManager {
        FolderManager { folders, files }
    }
    pub fn recurse_clean(&self, path: &Path) {
        let folders_res = path.read_dir();
        if folders_res.is_err() {
            return;
        }
        let folders = folders_res.unwrap();
        for folder in folders {
            if folder.is_err() {
                continue;
            }
            let folder = folder.unwrap();
            let path = folder.path();
            if path.is_dir() {
                let is_folder_prob = self
                    .folders
                    .get(path.file_name().unwrap().to_str().unwrap());
                if is_folder_prob.is_some() {
                    let is_folder = is_folder_prob.unwrap();
                    if *is_folder {
                        println!("removing: {:?}", path);
                        std::fs::remove_dir_all(&path).unwrap();
                        continue;
                    }
                }
                self.recurse_clean(&path);
            } else {
                let is_file_prob = self.files.get(path.file_name().unwrap().to_str().unwrap());
                if is_file_prob.is_some() {
                    println!("removing: {:?}", path);
                    std::fs::remove_file(&path).unwrap();
                }
            }
        }
    }
}

fn main() {
    let current_path = Path::new(".");
    let args: Vec<String> = std::env::args().collect();
    let mut folders: HashMap<String, bool> = HashMap::new();
    let mut files: HashMap<String, bool> = HashMap::new();
    // if -d is passed, then the next argument is a folder to delete
    // if -f is passed, then the next argument is a file to delete
    for i in 0..args.len() {
        if args[i] == "-d" {
            folders.insert(args[i + 1].clone(), true);
        }
        if args[i] == "-f" {
            files.insert(args[i + 1].clone(), true);
        }
    }
    FolderManager::new(folders, files).recurse_clean(&current_path);
}
