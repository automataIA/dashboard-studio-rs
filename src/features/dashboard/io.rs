use super::export::DashboardTemplate;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Blob, BlobPropertyBag, HtmlAnchorElement, Url};
use leptos::logging::*;

/// Download JSON template to user's computer
pub fn download_json(template: &DashboardTemplate, filename: &str) -> Result<(), JsValue> {
    debug_log!("download_json: Starting download for {}", filename);

    let json = template.to_json()
        .map_err(|e| {
            let err = format!("Serialization failed: {}", e);
            error!("{}", err);
            JsValue::from_str(&err)
        })?;

    debug_log!("JSON size: {} bytes", json.len());

    // Create Blob
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(&json));

    let options = BlobPropertyBag::new();
    options.set_type("application/json");

    let blob = Blob::new_with_str_sequence_and_options(&array, &options)?;
    debug_log!("Blob created");

    // Create download link
    let url = Url::create_object_url_with_blob(&blob)?;
    debug_log!("Object URL created");

    let document = window().unwrap().document().unwrap();
    let anchor = document
        .create_element("a")?
        .dyn_into::<HtmlAnchorElement>()?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.click();
    debug_log!("Download triggered");

    // Cleanup
    Url::revoke_object_url(&url)?;
    debug_log!("Object URL revoked");

    log!("download_json: Successfully triggered download");
    Ok(())
}

/// LocalStorage functions (WASM-compatible)
///
/// These functions provide dashboard persistence to browser localStorage.
pub mod storage {
    use super::*;

    const STORAGE_PREFIX: &str = "dashboard_template_";

    /// Save template to localStorage
    pub fn save_template(name: &str, template: &DashboardTemplate) -> Result<(), JsValue> {
        let json = template.to_json()
            .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))?;

        let storage = window()
            .unwrap()
            .local_storage()?
            .ok_or_else(|| JsValue::from_str("localStorage not available"))?;

        let key = format!("{}{}", STORAGE_PREFIX, name);
        storage.set_item(&key, &json)?;

        Ok(())
    }

    /// Load template from localStorage
    pub fn load_template(name: &str) -> Result<DashboardTemplate, String> {
        let storage = window()
            .unwrap()
            .local_storage()
            .map_err(|e| format!("Storage error: {:?}", e))?
            .ok_or_else(|| "localStorage not available".to_string())?;

        let key = format!("{}{}", STORAGE_PREFIX, name);
        let json = storage
            .get_item(&key)
            .map_err(|e| format!("Read error: {:?}", e))?
            .ok_or_else(|| format!("Template '{}' not found", name))?;

        DashboardTemplate::from_json(&json)
            .map_err(|e| format!("Parse error: {}", e))
    }

    /// List all saved template names
    pub fn list_templates() -> Result<Vec<String>, JsValue> {
        let storage = window()
            .unwrap()
            .local_storage()?
            .ok_or_else(|| JsValue::from_str("localStorage not available"))?;

        let len = storage.length()?;
        let mut names = Vec::new();

        for i in 0..len {
            if let Some(key) = storage.key(i)?
                && let Some(name) = key.strip_prefix(STORAGE_PREFIX) {
                    names.push(name.to_string());
                }
        }

        Ok(names)
    }

    /// Delete template from localStorage
    #[allow(dead_code)]
    pub fn delete_template(name: &str) -> Result<(), JsValue> {
        let storage = window()
            .unwrap()
            .local_storage()?
            .ok_or_else(|| JsValue::from_str("localStorage not available"))?;

        let key = format!("{}{}", STORAGE_PREFIX, name);
        storage.remove_item(&key)?;

        Ok(())
    }
}

/// ZIP Bundle functions (requires zip dependency)
///
/// To enable: Add `zip = { version = "0.6", default-features = false }` to Cargo.toml
/// and define feature `zip_export` in [features] section.
#[cfg(any())] // Disabled for now - remove this line to enable
#[allow(dead_code)]
pub mod zip_export {
    use super::*;
    use super::super::models::Dataset;
    use zip::write::{FileOptions, ZipWriter};
    use std::io::Cursor;

    /// Download ZIP bundle with dashboard.json + CSV files
    pub fn download_zip_bundle(
        template: &DashboardTemplate,
        datasets: &[Dataset],
        filename: &str,
    ) -> Result<(), JsValue> {
        let mut zip_buffer = Vec::new();
        let mut zip = ZipWriter::new(Cursor::new(&mut zip_buffer));

        let options = FileOptions::default();

        // 1. Add dashboard.json
        let json = template.to_json()
            .map_err(|e| JsValue::from_str(&format!("Serialization failed: {}", e)))?;

        zip.start_file("dashboard.json", options)
            .map_err(|e| JsValue::from_str(&format!("ZIP error: {}", e)))?;
        zip.write_all(json.as_bytes())
            .map_err(|e| JsValue::from_str(&format!("ZIP write error: {}", e)))?;

        // 2. Add CSV files
        for dataset in datasets {
            let csv_path = format!("data/{}", dataset.name);
            zip.start_file(&csv_path, options)
                .map_err(|e| JsValue::from_str(&format!("ZIP error: {}", e)))?;

            // Convert dataset.data to CSV string
            let csv_content = dataset_to_csv_string(dataset);
            zip.write_all(csv_content.as_bytes())
                .map_err(|e| JsValue::from_str(&format!("ZIP write error: {}", e)))?;
        }

        zip.finish()
            .map_err(|e| JsValue::from_str(&format!("ZIP finish error: {}", e)))?;

        // 3. Download as Blob
        let array = js_sys::Uint8Array::from(&zip_buffer[..]);
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&array);

        let mut blob_options = BlobPropertyBag::new();
        blob_options.type_("application/zip");

        let blob = Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options)?;

        // Create download link
        let url = Url::create_object_url_with_blob(&blob)?;

        let document = window().unwrap().document().unwrap();
        let anchor = document
            .create_element("a")?
            .dyn_into::<HtmlAnchorElement>()?;

        anchor.set_href(&url);
        anchor.set_download(filename);
        anchor.click();

        // Cleanup
        Url::revoke_object_url(&url)?;

        Ok(())
    }

    /// Convert Dataset to CSV string
    fn dataset_to_csv_string(dataset: &Dataset) -> String {
        use std::io::Write;

        // Header row (field names)
        let header = dataset.fields.iter()
            .map(|f| &f.name)
            .collect::<Vec<_>>()
            .join(",");

        // Data rows
        let rows = dataset.data.iter()
            .map(|row| {
                row.iter()
                    .map(|value| {
                        // Handle different JSON value types
                        match value {
                            serde_json::Value::String(s) => {
                                // Escape quotes in CSV
                                if s.contains(',') || s.contains('"') || s.contains('\n') {
                                    format!("\"{}\"", s.replace('"', "\"\""))
                                } else {
                                    s.clone()
                                }
                            }
                            serde_json::Value::Number(n) => n.to_string(),
                            serde_json::Value::Bool(b) => b.to_string(),
                            serde_json::Value::Null => String::new(),
                            _ => value.to_string(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!("{}\n{}", header, rows)
    }
}
