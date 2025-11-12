// src/ui/preview_panel.rs
// Panneau de prévisualisation de fichiers

use eframe::egui;
use crate::app::XFinderApp;
use std::path::Path;

pub fn render_preview_panel(ctx: &egui::Context, app: &mut XFinderApp) {
    if app.preview_file_path.is_none() {
        return;
    }

    let file_path = app.preview_file_path.as_ref().unwrap().clone();

    egui::Window::new("Previsualisation")
        .default_width(600.0)
        .default_height(500.0)
        .resizable(true)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Fichier:");
                ui.label(Path::new(&file_path).file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Inconnu"));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Fermer").clicked() {
                        app.preview_file_path = None;
                    }
                });
            });

            ui.separator();
            ui.add_space(5.0);

            // Métadonnées détaillées
            if let Ok(metadata) = std::fs::metadata(&file_path) {
                ui.group(|ui| {
                    ui.label(format!("Chemin: {}", file_path));

                    let size_bytes = metadata.len();
                    let size_str = format_size(size_bytes);
                    ui.label(format!("Taille: {}", size_str));

                    if let Ok(created) = metadata.created() {
                        let datetime: chrono::DateTime<chrono::Local> = created.into();
                        ui.label(format!("Cree: {}", datetime.format("%Y-%m-%d %H:%M:%S")));
                    }

                    if let Ok(modified) = metadata.modified() {
                        let datetime: chrono::DateTime<chrono::Local> = modified.into();
                        ui.label(format!("Modifie: {}", datetime.format("%Y-%m-%d %H:%M:%S")));
                    }

                    if let Ok(accessed) = metadata.accessed() {
                        let datetime: chrono::DateTime<chrono::Local> = accessed.into();
                        ui.label(format!("Dernier acces: {}", datetime.format("%Y-%m-%d %H:%M:%S")));
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Aperçu du contenu selon le type
                render_file_preview(ui, &file_path, &metadata);
            } else {
                ui.label("Impossible de lire les informations du fichier");
            }

            ui.add_space(10.0);
            ui.separator();

            // Actions rapides
            ui.horizontal(|ui| {
                if ui.button("Ouvrir avec app par defaut").clicked() {
                    let _ = opener::open(&file_path);
                }
                if ui.button("Ouvrir dossier parent").clicked() {
                    if let Some(parent) = Path::new(&file_path).parent() {
                        let _ = opener::open(parent);
                    }
                }
            });
        });
}

fn render_file_preview(ui: &mut egui::Ui, file_path: &str, metadata: &std::fs::Metadata) {
    let path = Path::new(file_path);
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    ui.label(format!("Type: {}", get_file_type(&extension)));
    ui.add_space(5.0);

    // Aperçu selon le type de fichier
    match extension.as_str() {
        "txt" | "md" | "log" | "json" | "xml" | "csv" | "rs" | "toml" | "yaml" | "yml" => {
            render_text_preview(ui, file_path, metadata.len());
        }
        "pdf" => {
            ui.label("Fichier PDF - Cliquez sur 'Ouvrir' pour visualiser");
        }
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" => {
            ui.label("Image - Aperçu non disponible pour le moment");
            ui.label("Cliquez sur 'Ouvrir' pour visualiser");
        }
        "mp3" | "wav" | "ogg" | "flac" => {
            ui.label("Fichier audio");
        }
        "mp4" | "avi" | "mkv" | "mov" => {
            ui.label("Fichier video");
        }
        "zip" | "rar" | "7z" | "tar" | "gz" => {
            ui.label("Archive compressée");
        }
        "exe" | "dll" | "msi" => {
            ui.label("Fichier executable Windows");
        }
        _ => {
            ui.label("Type de fichier non reconnu");
            ui.label("Cliquez sur 'Ouvrir' pour visualiser avec l'app par defaut");
        }
    }
}

fn render_text_preview(ui: &mut egui::Ui, file_path: &str, file_size: u64) {
    // Limite à 50KB pour la prévisualisation
    if file_size > 50_000 {
        ui.label(format!(
            "Fichier texte trop volumineux ({}) - Aperçu limite aux 50KB premiers octets",
            format_size(file_size)
        ));
    }

    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            let preview = if content.len() > 10_000 {
                // Limite à 10k caractères
                format!("{}...\n\n[Contenu tronque]", &content[..10_000])
            } else {
                content
            };

            ui.label("Aperçu:");
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut preview.as_str())
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace)
                    );
                });
        }
        Err(e) => {
            ui.label(format!("Impossible de lire le fichier: {}", e));
        }
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} octets", bytes)
    }
}

fn get_file_type(extension: &str) -> &str {
    match extension {
        "txt" => "Texte",
        "md" => "Markdown",
        "log" => "Fichier log",
        "json" => "JSON",
        "xml" => "XML",
        "csv" => "CSV",
        "rs" => "Rust source",
        "toml" => "TOML config",
        "yaml" | "yml" => "YAML config",
        "pdf" => "PDF",
        "png" | "jpg" | "jpeg" | "gif" | "bmp" => "Image",
        "mp3" | "wav" | "ogg" | "flac" => "Audio",
        "mp4" | "avi" | "mkv" | "mov" => "Video",
        "zip" | "rar" | "7z" | "tar" | "gz" => "Archive",
        "exe" | "dll" | "msi" => "Executable",
        _ => "Inconnu",
    }
}
