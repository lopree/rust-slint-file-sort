#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rfd::FileDialog;
use std::path::Path;
use std::{cell, rc};
mod file_sort;
use file_sort::*;
use std::io;
slint::include_modules!();
const MSG_DELETE_SUCCESS: &str = "文件删除成功";
const MSG_ORGANIZE_SUCCESS: &str = "文件以时间整理完成";
const MSG_ORGANIZE_BY_EXTENSION_SUCCESS: &str = "文件以后缀分类完成";
const MSG_RENAME_SUCCESS: &str = "文件重命名成功";
const MSG_RESET_SUCCESS: &str = "重置完成";
fn handle_result(app: &AppWindow, result: io::Result<()>, success_message: &str) {
    match result {
        Ok(_) => {
            app.invoke_show_popup_with_message(success_message.into());
        }
        Err(e) => {
            app.invoke_show_popup_with_message(format!("操作失败: {}", e).into());
        }
    }
}


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
            handle_result(&app,result,MSG_DELETE_SUCCESS)
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
            handle_result(&app,result,MSG_ORGANIZE_SUCCESS);
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
            handle_result(&app,result,MSG_ORGANIZE_BY_EXTENSION_SUCCESS);
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
            handle_result(&app,result,MSG_RENAME_SUCCESS);
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
            app.invoke_show_popup_with_message(MSG_RESET_SUCCESS.to_string().into());
        }
    });

    app.run()
}
