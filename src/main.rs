#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::Path;
use rfd::FileDialog;
use std::{cell, rc};
mod file_sort;
use file_sort::reset_folder;

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
    //重置文件夹
    let folder_path_clone = folder_path.clone();
    let app_weak = app.as_weak();
    app.on_reposition(move || {
        let path_string = folder_path_clone.borrow();
        if !path_string.is_empty() {
            reset_folder(Path::new(&*path_string));
            let app = app_weak.unwrap();
            app.invoke_show_popup();
        }
    });

    app.run()
}
