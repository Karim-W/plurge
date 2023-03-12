use std::path::Path;
fn main() {
    let current_path = Path::new(".");
    recurse_clean(&current_path);
}

fn recurse_clean(path: &Path) {
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
            if path.file_name().unwrap() == "target" {
                println!("removing: {:?}", path);
                std::fs::remove_dir_all(&path).unwrap();
                continue;
            }
            if path.file_name().unwrap() == ".git" {
                println!("removing: {:?}", path);
                std::fs::remove_dir_all(&path).unwrap();
                continue;
            }
            if path.file_name().unwrap() == "node_modules" {
                println!("removing: {:?}", path);
                std::fs::remove_dir_all(&path).unwrap();
                continue;
            }
            recurse_clean(&path);
        }
    }
}
