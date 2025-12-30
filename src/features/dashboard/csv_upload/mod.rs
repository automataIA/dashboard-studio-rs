mod file_reader;
mod csv_parser;
mod type_detector;
mod schema_validator;
mod upload_progress;

pub use upload_progress::{UploadState, UploadProgress};
pub use file_reader::read_file_with_progress;
pub use csv_parser::parse_csv_to_dataset;

use leptos::prelude::*;
use crate::features::dashboard::DashboardContext;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{File, FileList, HtmlInputElement};
use send_wrapper::SendWrapper;

/// CSV Upload Manager - Smart Component
///
/// Handles file selection, upload progress, parsing, and integration with DashboardContext.
/// This is the main interface for CSV upload functionality.
#[derive(Clone, Copy)]
pub struct CsvUploadManager {
    progress: ReadSignal<UploadProgress>,
    set_progress: WriteSignal<UploadProgress>,
    dashboard: DashboardContext,
}

impl CsvUploadManager {
    /// Create new upload manager
    pub fn new(dashboard: DashboardContext) -> Self {
        let (progress, set_progress) = signal(UploadProgress::default());
        Self {
            progress,
            set_progress,
            dashboard,
        }
    }

    /// Get current upload progress
    pub fn progress(&self) -> UploadProgress {
        self.progress.get()
    }

    /// Trigger file selection dialog
    pub fn select_file(&self) {
        self.set_progress.update(|p| {
            p.state = UploadState::SelectingFile;
        });

        // Create hidden file input
        if let Some(window) = web_sys::window()
            && let Some(document) = window.document() {
                let input = document
                    .create_element("input")
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();

                input.set_type("file");
                input.set_accept(".csv,.txt,.tsv");
                input.set_multiple(false);

                // Store closure to prevent GC
                #[allow(clippy::clone_on_copy)]
                let set_progress = SendWrapper::new(self.set_progress.clone());
                let dashboard = self.dashboard;

                let on_change = Closure::wrap(Box::new(move |ev: web_sys::Event| {
                    if let Some(target) = ev.target()
                        && let Ok(input) = target.dyn_into::<HtmlInputElement>()
                        && let Some(files) = input.files()
                        && files.length() > 0 {
                            let file = files.get(0).unwrap();
                            let set_progress = set_progress.clone();
                            Self::process_file(file, set_progress, dashboard);
                        }
                }) as Box<dyn Fn(web_sys::Event)>);

                input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
                on_change.forget();

                input.click();
            }
    }

    /// Handle drag & drop file
    pub fn handle_drop(&self, files: FileList) {
        if files.length() > 0 {
            let file = files.get(0).unwrap();
            #[allow(clippy::clone_on_copy)]
            let set_progress = SendWrapper::new(self.set_progress.clone());
            let dashboard = self.dashboard;

            Self::process_file(file, set_progress, dashboard);
        }
    }

    /// Process uploaded file
    fn process_file(
        file: File,
        set_progress: SendWrapper<WriteSignal<UploadProgress>>,
        dashboard: DashboardContext,
    ) {
        let filename = file.name();
        let file_size = file.size() as u64;

        // Update state
        set_progress.update(|p| {
            p.filename = Some(filename.clone());
            p.file_size = Some(file_size);
            p.state = UploadState::Uploading {
                progress: 0.0,
                bytes_read: 0,
                total_bytes: file_size,
            };
        });

        // Validate file
        if let Err(e) = schema_validator::validate_file(
            file_size,
            &filename,
            &schema_validator::ValidationConfig::default(),
        ) {
            set_progress.update(|p| {
                p.state = UploadState::Failed(e.to_string());
            });
            return;
        }

        // Read file with progress (callback-based, non-blocking)
        let set_progress_clone = set_progress.clone();
        let set_progress_clone2 = set_progress.clone();
        let dashboard_clone = dashboard;

        read_file_with_progress(
            file.clone(),
            move |bytes_read, total| {
                set_progress_clone.update(|p| {
                    p.state = UploadState::Uploading {
                        progress: (bytes_read as f64 / total as f64) * 100.0,
                        bytes_read,
                        total_bytes: total,
                    };
                });
            },
            move |result| {
                match result {
                    Ok(csv_text) => {
                        // File read successful, now parse
                        set_progress_clone2.update(|p| {
                            p.state = UploadState::Parsing {
                                progress: 0.0,
                                rows_processed: 0,
                            };
                        });

                        // Parse CSV (this is fast, so we can do it synchronously)
                        let filename = file.name();
                        let file_size = file.size() as u64;

                        match parse_csv_to_dataset(&csv_text, &filename, file_size) {
                            Ok((dataset, _fields)) => {
                                // Add to dashboard context
                                let dataset_id = dataset.id.clone();
                                dashboard_clone.add_dataset(dataset.clone());

                                // Set as active using ID, not name
                                dashboard_clone.set_active_dataset(Some(dataset_id));

                                // Complete
                                set_progress_clone2.update(|p| {
                                    p.state = UploadState::Completed;
                                });

                                // Log success
                                log::info!("Successfully uploaded and parsed: {}", filename);
                            }
                            Err(e) => {
                                set_progress_clone2.update(|p| {
                                    p.state = UploadState::Failed(e.to_string());
                                });
                            }
                        }
                    }
                    Err(e) => {
                        set_progress_clone2.update(|p| {
                            p.state = UploadState::Failed(e);
                        });
                    }
                }
            }
        );
    }
}
