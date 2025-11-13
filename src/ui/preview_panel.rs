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
        .max_width(800.0)
        .max_height(700.0)
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
                render_file_preview(ui, app, &file_path, &metadata);
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

fn render_file_preview(ui: &mut egui::Ui, app: &mut XFinderApp, file_path: &str, metadata: &std::fs::Metadata) {
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
            render_audio_preview(ui, app, file_path);
        }
        "mp4" | "avi" | "mkv" | "mov" | "wmv" => {
            render_video_preview(ui, file_path);
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

    // Support SVG via resvg
    let path_lower = file_path.to_lowercase();
    if path_lower.ends_with(".svg") {
        render_svg_preview(ui, file_path);
        return;
    }

    match image::open(file_path) {
        Ok(img) => {
            let size = [img.width() as usize, img.height() as usize];

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

            // S'adapter au container avec taille max pour ne pas agrandir la fenêtre
            let available_width = ui.available_width().min(750.0);
            let available_height = 400.0; // Hauteur max raisonnable

            ui.add(
                egui::Image::new(&texture)
                    .max_size(egui::vec2(available_width, available_height))
                    .shrink_to_fit()
            );
        }
        Err(e) => {
            ui.label(format!("Impossible de charger l'image: {}", e));
            ui.label("Cliquez sur 'Ouvrir' pour visualiser avec l'app par defaut");
        }
    }
}

fn render_svg_preview(ui: &mut egui::Ui, file_path: &str) {
    // Lire le fichier SVG
    match std::fs::read_to_string(file_path) {
        Ok(svg_content) => {
            // Afficher les dimensions du SVG si possible
            ui.label("Type: SVG (Scalable Vector Graphic)");
            ui.add_space(5.0);

            // Essayer de rendre le SVG
            // Pour l'instant, juste afficher un message
            // TODO: Implémenter le rendu SVG avec resvg quand disponible
            ui.label("Aperçu SVG pas encore disponible");
            ui.label(format!("Taille du fichier: {} octets", svg_content.len()));
            ui.label("Cliquez sur 'Ouvrir' pour visualiser");
        }
        Err(e) => {
            ui.label(format!("Impossible de lire le SVG: {}", e));
        }
    }
}

fn render_pdf_preview(ui: &mut egui::Ui, file_path: &str) {
    ui.label("Document PDF");
    ui.add_space(5.0);

    // Afficher les infos basiques
    if let Ok(metadata) = std::fs::metadata(file_path) {
        ui.group(|ui| {
            ui.label(format!("Taille: {}", format_size(metadata.len())));

            // Essayer de lire le nombre de pages (lecture rapide juste du header)
            if let Ok(bytes) = std::fs::read(file_path) {
                // Recherche simple du count de pages dans le PDF
                if let Ok(content) = String::from_utf8_lossy(&bytes[..bytes.len().min(10000)]).parse::<String>() {
                    if let Some(count_pos) = content.find("/Count ") {
                        if let Some(page_count_str) = content[count_pos+7..].split_whitespace().next() {
                            if let Ok(pages) = page_count_str.parse::<u32>() {
                                ui.label(format!("Pages: ~{}", pages));
                            }
                        }
                    }
                }
            }
        });
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    // Message explicatif
    ui.label("📄 Aperçu PDF non disponible");
    ui.small("L'extraction de texte peut être lente pour les gros fichiers.");
    ui.add_space(5.0);

    ui.horizontal(|ui| {
        ui.label("→");
        if ui.button("Ouvrir le PDF").clicked() {
            let _ = opener::open(file_path);
        }
        ui.label("pour le visualiser");
    });
}

fn render_audio_preview(ui: &mut egui::Ui, app: &mut XFinderApp, file_path: &str) {
    ui.label("Lecteur audio integre:");
    ui.add_space(5.0);

    if let Some(ref mut player) = app.audio_player {
        // Afficher le fichier en cours
        if let Some(current) = player.current_file() {
            if current == file_path {
                ui.label(format!("En lecture: {}",
                    Path::new(file_path).file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Inconnu")
                ));
            }
        }

        ui.add_space(5.0);

        // Boutons de controle
        ui.horizontal(|ui| {
            if ui.button("▶ Lire").clicked() {
                if let Err(e) = player.load_and_play(file_path) {
                    app.error_message = Some(format!("Erreur lecture audio: {}", e));
                }
            }

            if player.is_playing() {
                if ui.button("⏸ Pause").clicked() {
                    player.pause();
                }
            } else if player.current_file().is_some() {
                if ui.button("▶ Reprendre").clicked() {
                    player.resume();
                }
            }

            if ui.button("⏹ Stop").clicked() {
                player.stop();
            }
        });

        ui.add_space(5.0);

        // Controle de volume
        let mut volume = player.get_volume();
        ui.horizontal(|ui| {
            ui.label("Volume:");
            if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).show_value(false)).changed() {
                player.set_volume(volume);
            }
            ui.label(format!("{}%", (volume * 100.0) as i32));
        });
    } else {
        ui.label("Lecteur audio non disponible");
        ui.label("Cliquez sur 'Ouvrir' pour ecouter avec l'app par defaut");
    }
}

fn render_video_preview(ui: &mut egui::Ui, file_path: &str) {
    ui.label("Fichier video:");
    ui.add_space(5.0);

    // Afficher les infos du fichier
    if let Ok(metadata) = std::fs::metadata(file_path) {
        let size_bytes = metadata.len();
        ui.label(format!("Taille: {}", format_size(size_bytes)));

        if let Some(extension) = Path::new(file_path).extension().and_then(|e| e.to_str()) {
            ui.label(format!("Format: {}", extension.to_uppercase()));
        }

        ui.add_space(5.0);
    }

    // Extraire les métadonnées pour MP4
    let path_lower = file_path.to_lowercase();
    if path_lower.ends_with(".mp4") || path_lower.ends_with(".m4v") {
        match extract_mp4_metadata(file_path) {
            Ok(info) => {
                if info.resolution.is_some() || info.duration.is_some() {
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Informations video:").strong());
                        if let Some((width, height)) = info.resolution {
                            ui.label(format!("Resolution: {}x{} pixels", width, height));
                        }
                        if let Some(duration) = info.duration {
                            ui.label(format!("Duree: {}", format_duration(duration)));
                        }
                    });
                    ui.add_space(5.0);
                }
            }
            Err(_) => {
                // Échec silencieux, on continue
            }
        }
    }

    ui.label("Note: Extraction de frame necessiterait ffmpeg (trop lourd)");
    ui.label("Cliquez sur 'Ouvrir' pour lire la video");
}

struct VideoInfo {
    resolution: Option<(u32, u32)>,
    duration: Option<f64>,
}

fn extract_mp4_metadata(file_path: &str) -> Result<VideoInfo, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let context = mp4parse::read_mp4(&mut reader)?;

    let mut info = VideoInfo {
        resolution: None,
        duration: None,
    };

    // Extraire les infos de la première piste vidéo
    for track in &context.tracks {
        if let mp4parse::TrackType::Video = track.track_type {
            // Résolution depuis tkhd
            if let Some(ref tkhd) = track.tkhd {
                let width = (tkhd.width >> 16) as u32;
                let height = (tkhd.height >> 16) as u32;
                if width > 0 && height > 0 {
                    info.resolution = Some((width, height));
                }
            }

            // Durée (secondes) depuis edited_duration si disponible
            if let Some(ref duration) = track.edited_duration {
                if let Some(timescale) = context.timescale {
                    if timescale.0 > 0 {
                        info.duration = Some(duration.0 as f64 / timescale.0 as f64);
                    }
                }
            }

            break; // On prend juste la première piste vidéo
        }
    }

    Ok(info)
}

fn format_duration(seconds: f64) -> String {
    let total_secs = seconds as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
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
