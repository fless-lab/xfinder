// src/ui/preview_panel.rs
// Panneau de prévisualisation de fichiers

use eframe::egui;
use crate::app::XFinderApp;
use crate::ui::icons;
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

    // Afficher l'icône et le type
    ui.horizontal(|ui| {
        // Icône SVG monochromes'adapte au thème
        render_file_icon_svg(ui, &extension);
        ui.add_space(10.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(get_file_type(&extension)).size(16.0).strong());
            ui.label(format!("Extension: .{}", extension));
        });
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    // Aperçu selon le type de fichier
    match extension.as_str() {
        "txt" | "md" | "log" | "json" | "xml" | "csv" | "rs" | "toml" | "yaml" | "yml" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "h" => {
            render_text_preview(ui, file_path, metadata.len());
        }
        "pdf" => {
            render_pdf_preview(ui, file_path);
        }
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" => {
            render_image_preview(ui, file_path);
        }
        "mp3" | "wav" | "ogg" | "flac" => {
            ui.label("Fichier audio");
            ui.label("Cliquez sur 'Ouvrir' pour ecouter");
        }
        "mp4" | "avi" | "mkv" | "mov" => {
            ui.label("Fichier video");
            ui.label("Cliquez sur 'Ouvrir' pour visualiser");
        }
        "zip" | "rar" | "7z" | "tar" | "gz" => {
            ui.label("Archive compressée");
            ui.label("Cliquez sur 'Ouvrir' pour extraire");
        }
        "exe" | "dll" | "msi" => {
            ui.label("Fichier executable/binaire Windows");
            ui.colored_label(egui::Color32::from_rgb(200, 100, 50), "! Attention: verifiez la source avant d'executer");
        }
        _ => {
            ui.label("Type de fichier non reconnu");
            ui.label("Cliquez sur 'Ouvrir' pour visualiser avec l'app par defaut");
        }
    }
}

fn render_image_preview(ui: &mut egui::Ui, file_path: &str) {
    ui.label("Aperçu de l'image:");

    match image::open(file_path) {
        Ok(img) => {
            let size = [img.width() as usize, img.height() as usize];

            // Limiter la taille de l'aperçu à 500x500
            let max_size = 500.0;
            let scale = (max_size / size[0] as f32).min(max_size / size[1] as f32).min(1.0);
            let display_size = [size[0] as f32 * scale, size[1] as f32 * scale];

            ui.label(format!("Dimensions: {}x{} pixels", size[0], size[1]));
            ui.add_space(5.0);

            // Convertir en RGBA8
            let rgba_img = img.to_rgba8();
            let pixels = rgba_img.as_flat_samples();

            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [size[0], size[1]],
                pixels.as_slice(),
            );

            let texture = ui.ctx().load_texture(
                file_path,
                color_image,
                egui::TextureOptions::default()
            );

            ui.add(egui::Image::new(&texture).max_size(egui::vec2(display_size[0], display_size[1])));
        }
        Err(e) => {
            ui.label(format!("Impossible de charger l'image: {}", e));
            ui.label("Cliquez sur 'Ouvrir' pour visualiser avec l'app par defaut");
        }
    }
}

fn render_pdf_preview(ui: &mut egui::Ui, _file_path: &str) {
    ui.label("Document PDF");
    ui.label("Aperçu texte non disponible pour le moment");
    ui.label("Cliquez sur 'Ouvrir' pour visualiser le PDF");

    // TODO: Utiliser pdf-extract pour extraire le texte
    // Pour l'instant, juste afficher l'info
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

fn render_file_icon_svg(ui: &mut egui::Ui, extension: &str) {
    // Dessiner icone directement avec egui painter (monochrome adapte au theme)
    let (response, painter) = ui.allocate_painter(
        egui::vec2(48.0, 48.0),
        egui::Sense::hover()
    );

    let rect = response.rect;
    let color = ui.style().visuals.text_color();
    let stroke = egui::Stroke::new(2.0, color);

    // Dessiner selon le type de fichier
    match extension {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" | "svg" => {
            // Image: rectangle avec cercle
            painter.rect_stroke(rect.shrink(4.0), 2.0, stroke);
            painter.circle_filled(rect.center() + egui::vec2(-10.0, -8.0), 4.0, color);
            painter.line_segment([rect.left_bottom() + egui::vec2(4.0, -4.0), rect.center() + egui::vec2(0.0, 8.0)], stroke);
            painter.line_segment([rect.center() + egui::vec2(0.0, 8.0), rect.right_bottom() + egui::vec2(-4.0, -4.0)], stroke);
        }
        "mp3" | "wav" | "ogg" | "flac" => {
            // Audio: note de musique
            painter.circle_filled(rect.center() + egui::vec2(-6.0, 12.0), 6.0, color);
            painter.line_segment([rect.center() + egui::vec2(0.0, 12.0), rect.center() + egui::vec2(0.0, -12.0)], egui::Stroke::new(3.0, color));
            painter.line_segment([rect.center() + egui::vec2(0.0, -12.0), rect.center() + egui::vec2(10.0, -8.0)], egui::Stroke::new(3.0, color));
        }
        "mp4" | "avi" | "mkv" | "mov" | "wmv" => {
            // Video: rectangle + triangle play
            painter.rect_stroke(rect.shrink(4.0), 2.0, stroke);
            let triangle = vec![
                rect.center() + egui::vec2(-6.0, -8.0),
                rect.center() + egui::vec2(-6.0, 8.0),
                rect.center() + egui::vec2(8.0, 0.0),
            ];
            painter.add(egui::Shape::convex_polygon(triangle, color, egui::Stroke::NONE));
        }
        "zip" | "rar" | "7z" | "tar" | "gz" => {
            // Archive: boite
            painter.rect_stroke(rect.shrink(8.0), 2.0, stroke);
            painter.line_segment([rect.center_top() + egui::vec2(0.0, 8.0), rect.center_bottom() + egui::vec2(0.0, -8.0)], stroke);
        }
        "exe" | "msi" => {
            // Executable: engrenage
            painter.circle_stroke(rect.center(), 12.0, stroke);
            for i in 0..8 {
                let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
                let start = rect.center() + egui::vec2(angle.cos(), angle.sin()) * 12.0;
                let end = rect.center() + egui::vec2(angle.cos(), angle.sin()) * 18.0;
                painter.line_segment([start, end], stroke);
            }
        }
        "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "h" => {
            // Code: chevrons <>
            painter.line_segment([rect.center() + egui::vec2(-12.0, 0.0), rect.center() + egui::vec2(-18.0, -12.0)], stroke);
            painter.line_segment([rect.center() + egui::vec2(-12.0, 0.0), rect.center() + egui::vec2(-18.0, 12.0)], stroke);
            painter.line_segment([rect.center() + egui::vec2(12.0, 0.0), rect.center() + egui::vec2(18.0, -12.0)], stroke);
            painter.line_segment([rect.center() + egui::vec2(12.0, 0.0), rect.center() + egui::vec2(18.0, 12.0)], stroke);
        }
        _ => {
            // Fichier par defaut: document
            let points = vec![
                rect.left_top() + egui::vec2(6.0, 4.0),
                rect.right_top() + egui::vec2(-12.0, 4.0),
                rect.right_top() + egui::vec2(-6.0, 10.0),
                rect.right_bottom() + egui::vec2(-6.0, -4.0),
                rect.left_bottom() + egui::vec2(6.0, -4.0),
            ];
            painter.add(egui::Shape::closed_line(points, stroke));
            painter.line_segment([rect.center() + egui::vec2(-8.0, 2.0), rect.center() + egui::vec2(8.0, 2.0)], stroke);
            painter.line_segment([rect.center() + egui::vec2(-8.0, 8.0), rect.center() + egui::vec2(8.0, 8.0)], stroke);
        }
    }
}
