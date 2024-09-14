use std::path::{Path, PathBuf};

use serde::Serialize;
use sysinfo::Disks;
use tauri::api::dir::read_dir;
use window_shadows::set_shadow;

use crate::error::BackendError;

#[derive(Serialize)]
pub struct Folder {
    directory: PathBuf,
    items: Vec<Item>,
    have_parent: bool,
}

#[derive(Serialize)]
struct Item {
    path: PathBuf,
    name: String,
    is_dir: bool,
}

fn get_windows_drives() -> Vec<Item> {
    Disks::new_with_refreshed_list()
        .list()
        .iter()
        .map(|d| {
            let p = d.mount_point().to_path_buf();
            Item {
                path: p.clone(),
                name: p.to_str().unwrap().to_owned(),
                is_dir: true,
            }
        })
        .collect()
}

#[tauri::command]
pub async fn open_file_dialog(handle: tauri::AppHandle) -> Result<String, BackendError> {
    let dialog =
        tauri::WindowBuilder::new(&handle, "file", tauri::WindowUrl::App("file.html".into()))
            .title("Select File")
            .decorations(false)
            .resizable(false)
            .build()?;

    let new_label = dialog.label().into();

    set_shadow(dialog, true)?;

    Ok(new_label)
}

#[tauri::command]
pub fn get_items(dir: &str) -> Result<Folder, BackendError> {
    let path = PathBuf::from(dir);

    if cfg!(target_os = "windows") && path == PathBuf::from("") {
        return Ok(Folder {
            directory: path,
            items: get_windows_drives(),
            have_parent: false,
        });
    }

    if !path.exists() {
        return Err(BackendError {
            kind: "crate::error::BackendError".into(),
            message: "Path does not exist".into(),
        });
    }

    let have_parent = match cfg!(target_os = "windows") {
        true => true,
        false => path != Path::new("/"),
    };

    let items = read_dir(path.clone(), false)?
        .iter()
        .map(|item| Item {
            path: item.path.clone(),
            name: item.name.clone().unwrap(),
            is_dir: item.path.is_dir(),
        })
        .collect();

    Ok(Folder {
        directory: path,
        items,
        have_parent,
    })
}

#[tauri::command]
pub fn get_parent(dir: &str) -> Result<PathBuf, BackendError> {
    let path = PathBuf::from(dir);

    if cfg!(target_os = "windows") {
        if path.parent().is_none() {
            return Ok(PathBuf::from(""));
        }
    } else {
        if path == Path::new("/") {
            return Ok(PathBuf::from("/"));
        }
    }

    path.parent().map(|p| p.to_path_buf()).ok_or(BackendError {
        kind: "crate::error::BackendError".into(),
        message: format!("Path {} not found", dir),
    })
}
