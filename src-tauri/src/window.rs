use window_shadows::set_shadow;

use crate::{error::BackendError, Response};

#[tauri::command]
pub async fn open_file_dialog(handle: tauri::AppHandle) -> Result<Response, BackendError> {
    let dialog =
        tauri::WindowBuilder::new(&handle, "file", tauri::WindowUrl::App("file.html".into()))
            .decorations(false)
            .build()?;

    set_shadow(dialog, true)?;

    Ok(Response {
        kind: "success".to_owned(),
        message: "Window::OpenFileDialog successful".to_owned(),
    })
}
