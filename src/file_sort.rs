use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, Timelike, Datelike};
//分类：按照年月日时来分类
pub fn organize_files_by_time(source_dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let modified: DateTime<Local> = metadata.modified()?.into();
            
            let day = modified.day();
            let hour = modified.hour();

            // 判断上下午
            let am_pm = if hour < 12 { "上午" } else { "下午" };

            // 转换为12小时制
            let hour_12 = if hour == 0 || hour == 12 { 12 } else { hour % 12 };

            // 格式化文件夹名称
            let hour_folder = format!("{}{}点", am_pm, hour_12);
            
            let new_path = source_dir.join(format!("{}年/{:02}月/{}号/{}", 
                modified.year(),
                modified.month(),
                day,
                hour_folder
            ));
            
            fs::create_dir_all(&new_path)?;
            fs::rename(&path, new_path.join(path.file_name().unwrap()))?;
        }
    }
    Ok(())
}

//重置文件夹
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

