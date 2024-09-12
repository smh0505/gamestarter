use tauri::Manager;
use window_shadows::set_shadow;

use crate::{error::BackendError, Response};

#[tauri::command]
pub async fn open_file_dialog(handle: tauri::AppHandle) -> Result<Response, BackendError> {
    let dialog = tauri::WindowBuilder::new(&handle, "file", tauri::WindowUrl::App("file.html".into()))
        .decorations(false)
        .build()?;

    set_shadow(dialog, true)?;
    
    Ok(Response {
        kind: "success".to_owned(),
        message: "Window::OpenFileDialog successful".to_owned(),
    })
}

#[tauri::command]
pub async fn close_file_dialog(handle: tauri::AppHandle) -> Result<Response, BackendError> {
    handle
        .get_window("file")
        .ok_or(BackendError {
            kind: "tauri::Error".to_owned(),
            message: "the window with the label `file` not found".to_owned(),
        })?
        .close()?;

    Ok(Response {
        kind: "success".to_owned(),
        message: "Window::CloseFileDialog successful".to_owned(),
    })
}
