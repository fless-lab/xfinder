// src/app.rs
// Application principale xfinder

use eframe::egui;
use std::path::PathBuf;

use crate::search::{SearchIndex, SearchResult};
use crate::ui::{render_main_ui, render_side_panel, render_top_panel};

pub struct XFinderApp {
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub search_index: Option<SearchIndex>,
    pub index_dir: PathBuf,
    pub index_status: IndexStatus,
    pub indexing_in_progress: bool,
    pub error_message: Option<String>,
}

#[derive(Default)]
pub struct IndexStatus {
    pub is_ready: bool,
    pub file_count: usize,
    pub last_update: Option<String>,
}

impl Default for XFinderApp {
    fn default() -> Self {
        let index_dir = std::env::current_dir()
            .unwrap_or_default()
            .join(".xfinder_index");

        Self {
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: None,
            index_dir,
            index_status: IndexStatus::default(),
            indexing_in_progress: false,
            error_message: None,
        }
    }
}

impl XFinderApp {
    pub fn load_index(&mut self) {
        match SearchIndex::new(&self.index_dir) {
            Ok(index) => {
                self.search_index = Some(index);
                self.index_status.is_ready = true;
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Erreur chargement index: {}", e));
            }
        }
    }

    pub fn start_indexing(&mut self) {
        self.indexing_in_progress = true;
        self.error_message = None;

        if self.search_index.is_none() {
            self.load_index();
        }

        if let Some(ref index) = self.search_index {
            match index.create_writer() {
                Ok(mut writer) => {
                    let test_files = vec![
                        ("C:\\Users\\Public\\test1.txt", "test1.txt"),
                        ("C:\\Users\\Public\\test2.pdf", "test2.pdf"),
                        ("C:\\Users\\Public\\document.docx", "document.docx"),
                    ];

                    let mut indexed_count = 0;
                    for (path, filename) in test_files {
                        if index.add_file(&mut writer, path, filename).is_ok() {
                            indexed_count += 1;
                        }
                    }

                    match writer.commit() {
                        Ok(_) => {
                            self.index_status.file_count = indexed_count;
                            self.index_status.last_update = Some(
                                chrono::Local::now()
                                    .format("%Y-%m-%d %H:%M:%S")
                                    .to_string(),
                            );
                            self.error_message =
                                Some(format!("{} fichiers indexes avec succes", indexed_count));
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Erreur commit: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur creation writer: {}", e));
                }
            }
        }

        self.indexing_in_progress = false;
    }

    pub fn perform_search(&mut self) {
        if self.search_query.trim().is_empty() {
            self.search_results.clear();
            return;
        }

        if let Some(ref index) = self.search_index {
            match index.search(&self.search_query, 50) {
                Ok(results) => {
                    self.search_results = results;
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Erreur recherche: {}", e));
                    self.search_results.clear();
                }
            }
        } else {
            self.error_message =
                Some("Index non charge. Lancez une indexation d'abord.".to_string());
        }
    }
}

impl eframe::App for XFinderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        render_top_panel(ctx, self);
        render_side_panel(ctx, self);
        render_main_ui(ctx, self);
    }
}
