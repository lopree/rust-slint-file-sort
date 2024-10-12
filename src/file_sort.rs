use std::fs;
use std::path::Path;

pub fn reset_folder(path: &Path) {
    if !path.is_dir() {
        return;
    }

    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                // 处理子文件夹
                reset_folder(&entry_path);
                // 移动子文件夹中的文件到主文件夹
                move_files_to_parent(&entry_path, path);
                // 删除空的子文件夹
                if is_directory_empty(&entry_path) {
                    fs::remove_dir(entry_path).unwrap();
                }
            }
        }
    }
}

fn move_files_to_parent(from: &Path, to: &Path) {
    if let Ok(entries) = fs::read_dir(from) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let new_path = to.join(path.file_name().unwrap());
                    fs::rename(path, new_path).unwrap();
                }
            }
        }
    }
}

fn is_directory_empty(path: &Path) -> bool {
    fs::read_dir(path).map(|mut i| i.next().is_none()).unwrap_or(false)
}

