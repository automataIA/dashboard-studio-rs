use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{File, FileReader, ProgressEvent};
use std::sync::Arc;

/// Read file as text with progress tracking
///
/// # Arguments
/// * `file` - The file to read
/// * `on_progress` - Callback for progress updates (bytes_read, total_bytes)
/// * `on_complete` - Callback for completion (result)
///
/// This version doesn't use channels and is non-blocking
pub fn read_file_with_progress(
    file: File,
    on_progress: impl Fn(u64, u64) + 'static,
    on_complete: impl FnOnce(Result<String, String>) + 'static + Clone,
) {
    let file_size = file.size() as u64;
    let file_name = file.name();

    // Maximum file size: 100MB
    const MAX_SIZE: u64 = 100 * 1024 * 1024;

    if file_size > MAX_SIZE {
        on_complete(Err(format!(
            "File '{}' ({} MB) exceeds maximum size of {} MB",
            file_name,
            file_size / (1024 * 1024),
            MAX_SIZE / (1024 * 1024)
        )));
        return;
    }

    // Create FileReader
    let reader = match FileReader::new() {
        Ok(r) => r,
        Err(e) => {
            on_complete(Err(format!("Failed to create FileReader: {:?}", e)));
            return;
        }
    };

    // Setup progress callback
    let on_progress = Arc::new(on_progress);
    let progress_callback = {
        let on_progress = on_progress.clone();
        Closure::wrap(Box::new(move |ev: ProgressEvent| {
            let loaded = ev.loaded();
            on_progress(loaded as u64, file_size);
        }) as Box<dyn Fn(ProgressEvent)>)
    };

    reader
        .set_onprogress(Some(progress_callback.as_ref().unchecked_ref()));
    progress_callback.forget();

    // Setup load callback (success)
    let on_complete_clone = on_complete.clone();
    let load_callback = Closure::once(move |ev: ProgressEvent| {
        let target = ev.target().unwrap();
        let reader: &FileReader = target.unchecked_ref();

        let result = reader.result().unwrap();
        let text = result.as_string().unwrap();

        on_complete_clone(Ok(text));
    });

    reader.set_onload(Some(load_callback.as_ref().unchecked_ref()));
    load_callback.forget();

    // Setup error callback
    let error_callback = Closure::once(move |_ev: ProgressEvent| {
        on_complete(Err("Failed to read file".to_string()));
    });

    reader
        .set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    error_callback.forget();

    // Start reading
    if let Err(e) = reader.read_as_text(&file) {
        // Can't call on_complete here as it's already moved, so we log instead
        log::error!("Failed to start reading file: {:?}", e);
    }
}
