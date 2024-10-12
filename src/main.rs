#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rfd::FileDialog;
use std::path::Path;
use std::{cell, rc};
mod file_sort;
use file_sort::*;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let folder_path = rc::Rc::new(cell::RefCell::new(String::new()));
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();
    //记录选择的文件夹
    let folder_path_clone = folder_path.clone();
    app.on_select_folder(move || {
        let app = app_weak.unwrap();
        if let Some(folder) = FileDialog::new().pick_folder() {
            *folder_path_clone.borrow_mut() = folder.to_string_lossy().to_string();
            // 更新 Slint UI 中 LineEdit 的文本内容
            app.set_new_folder_path(folder.to_string_lossy().to_string().into());
        }
    });
    //分类文件夹
    let folder_path_clone = folder_path.clone();
    let app_weak = app.as_weak();
    app.on_delete_lrf(move || {
        let path_string = folder_path_clone.borrow();
        if !path_string.is_empty() {
            let result = delete_lrf_files(Path::new(&*path_string));
            let app = app_weak.unwrap();
            match result {
                Ok(_) => {
                    app.invoke_show_popup_with_message("删除成功".into());
                }
                Err(e) => {
                    app.invoke_show_popup_with_message(format!("文件整理失败: {}", e).into());
                }
            };
        }
    });
    //分类文件夹
    let folder_path_clone = folder_path.clone();
    let app_weak = app.as_weak();
    app.on_organize(move || {
        let path_string = folder_path_clone.borrow();
        if !path_string.is_empty() {
            let result = organize_files_by_time(Path::new(&*path_string));
            let app = app_weak.unwrap();
            match result {
                Ok(_) => {
                    app.invoke_show_popup_with_message("文件整理完成".into());
                }
                Err(e) => {
                    app.invoke_show_popup_with_message(format!("文件整理失败: {}", e).into());
                }
            };
        }
    });
    //以后缀分类文件
    let folder_path_clone = folder_path.clone();
    let app_weak = app.as_weak();
    app.on_organize_by_extension(move || {
        let path_string = folder_path_clone.borrow();
        if !path_string.is_empty() {
            let result = sort_files_by_extension(Path::new(&*path_string));
            let app = app_weak.unwrap();
            match result {
                Ok(_) => {
                    app.invoke_show_popup_with_message("文件以后缀分类完成".into());
                }
                Err(e) => {
                    app.invoke_show_popup_with_message(format!("文件整理失败: {}", e).into());
                }
            };
        }
    });
    //重命名文件
    let folder_path_clone = folder_path.clone();
    let app_weak = app.as_weak();
    app.on_rename(move || {
        let path_string = folder_path_clone.borrow();
        if !path_string.is_empty() {
            let result = rename_files_by_modified_time(Path::new(&*path_string));
            let app = app_weak.unwrap();
            match result {
                Ok(_) => {
                    app.invoke_show_popup_with_message("文件重命名成功".into());
                }
                Err(e) => {
                    app.invoke_show_popup_with_message(format!("文件整理失败: {}", e).into());
                }
            };
        }
    });
    //重置文件夹
    let folder_path_clone = folder_path.clone();
    let app_weak = app.as_weak();
    app.on_reposition(move || {
        let path_string = folder_path_clone.borrow();
        if !path_string.is_empty() {
            reset_folder(Path::new(&*path_string));
            let app = app_weak.unwrap();
            app.invoke_show_popup_with_message("重置完成".to_string().into());
        }
    });

    app.run()
}
