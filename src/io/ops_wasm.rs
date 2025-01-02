use crate::io;
use crate::io::filter::FileFilter;
use rfd::FileHandle;

pub async fn load_with_file_pick(
    file_filter: FileFilter,
) -> Option<Result<String, std::io::Error>> {
    let task = rfd::AsyncFileDialog::new()
        .add_filter(file_filter.name, &file_filter.file_extensions)
        .pick_file();

    if let Some(file_handle) = task.await {
        Some(Ok(load_from_file(file_handle).await))
    } else {
        None
    }
}

pub async fn save_with_file_pick(
    text: String, file_filter: FileFilter,
) -> Option<Result<(), std::io::Error>> {
    let name = io::filename::generate(6, &file_filter.file_extensions);

    let task = rfd::AsyncFileDialog::new()
        .add_filter(file_filter.name, &file_filter.file_extensions)
        .set_file_name(name)
        .save_file();

    if let Some(file_handle) = task.await {
        Some(save_to_file(file_handle, text).await)
    } else {
        None
    }
}

pub async fn load_from_file(handle: FileHandle) -> String {
    let bytes = handle.read().await;
    String::from_utf8_lossy(&bytes).to_string()
}

pub async fn save_to_file(
    handle: FileHandle, text: String,
) -> Result<(), std::io::Error> {
    handle.write(text.as_bytes()).await
}
