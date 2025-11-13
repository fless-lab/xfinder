// src/ui/icons.rs
// Icônes SVG monochromes pour les types de fichiers

pub fn get_file_icon_svg(extension: &str) -> &'static str {
    match extension {
        "txt" | "md" | "log" => ICON_TEXT,
        "json" | "xml" | "csv" | "toml" | "yaml" | "yml" => ICON_CONFIG,
        "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "h" => ICON_CODE,
        "pdf" => ICON_PDF,
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" | "svg" => ICON_IMAGE,
        "mp3" | "wav" | "ogg" | "flac" => ICON_AUDIO,
        "mp4" | "avi" | "mkv" | "mov" | "wmv" => ICON_VIDEO,
        "zip" | "rar" | "7z" | "tar" | "gz" => ICON_ARCHIVE,
        "exe" | "msi" => ICON_EXECUTABLE,
        "dll" | "so" => ICON_LIBRARY,
        "doc" | "docx" => ICON_DOCUMENT,
        "xls" | "xlsx" => ICON_SPREADSHEET,
        "ppt" | "pptx" => ICON_PRESENTATION,
        _ => ICON_FILE,
    }
}

// Icône fichier texte
const ICON_TEXT: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
  <line x1="8" y1="13" x2="16" y2="13"/>
  <line x1="8" y1="17" x2="16" y2="17"/>
</svg>"#;

// Icône configuration
const ICON_CONFIG: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
  <circle cx="12" cy="15" r="2"/>
  <path d="M12 13v-2m0 6v-2m-2.83-.83l-1.42-1.42m7.08 0l-1.42 1.42m-4.24-4.24l-1.42-1.42m7.08 0l-1.42 1.42"/>
</svg>"#;

// Icône code
const ICON_CODE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
  <polyline points="10 17 7 14 10 11"/>
  <polyline points="14 11 17 14 14 17"/>
</svg>"#;

// Icône PDF
const ICON_PDF: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
  <text x="7" y="17" font-size="6" fill="currentColor" font-weight="bold">PDF</text>
</svg>"#;

// Icône image
const ICON_IMAGE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
  <circle cx="8.5" cy="8.5" r="1.5"/>
  <polyline points="21 15 16 10 5 21"/>
</svg>"#;

// Icône audio
const ICON_AUDIO: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M9 18V5l12-2v13"/>
  <circle cx="6" cy="18" r="3"/>
  <circle cx="18" cy="16" r="3"/>
</svg>"#;

// Icône vidéo
const ICON_VIDEO: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <polygon points="23 7 16 12 23 17 23 7"/>
  <rect x="2" y="5" width="14" height="14" rx="2" ry="2"/>
</svg>"#;

// Icône archive
const ICON_ARCHIVE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <polyline points="21 8 21 21 3 21 3 8"/>
  <rect x="1" y="3" width="22" height="5"/>
  <line x1="10" y1="12" x2="14" y2="12"/>
</svg>"#;

// Icône exécutable
const ICON_EXECUTABLE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <circle cx="12" cy="12" r="3"/>
  <path d="M12 1v6m0 6v6m5.2-14.2l-4.2 4.2m0 6l4.2 4.2M23 12h-6m-6 0H1m14.2 5.2l-4.2-4.2m0-6l-4.2-4.2"/>
</svg>"#;

// Icône bibliothèque/DLL
const ICON_LIBRARY: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <rect x="4" y="4" width="16" height="16" rx="2" ry="2"/>
  <rect x="9" y="9" width="6" height="6"/>
  <line x1="9" y1="1" x2="9" y2="4"/>
  <line x1="15" y1="1" x2="15" y2="4"/>
  <line x1="9" y1="20" x2="9" y2="23"/>
  <line x1="15" y1="20" x2="15" y2="23"/>
  <line x1="20" y1="9" x2="23" y2="9"/>
  <line x1="20" y1="14" x2="23" y2="14"/>
  <line x1="1" y1="9" x2="4" y2="9"/>
  <line x1="1" y1="14" x2="4" y2="14"/>
</svg>"#;

// Icône document Word
const ICON_DOCUMENT: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
  <line x1="8" y1="13" x2="16" y2="13"/>
  <line x1="8" y1="17" x2="12" y2="17"/>
</svg>"#;

// Icône tableur Excel
const ICON_SPREADSHEET: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
  <line x1="8" y1="13" x2="16" y2="13"/>
  <line x1="12" y1="11" x2="12" y2="19"/>
</svg>"#;

// Icône présentation PowerPoint
const ICON_PRESENTATION: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
  <rect x="8" y="12" width="8" height="6"/>
</svg>"#;

// Icône fichier générique
const ICON_FILE: &str = r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
  <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
  <polyline points="14 2 14 8 20 8"/>
</svg>"#;
