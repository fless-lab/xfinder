// src/system/autostart.rs
// Gestion du démarrage automatique avec Windows

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;
use std::path::PathBuf;

const APP_NAME: &str = "xfinder";

/// Activer le démarrage automatique avec Windows
#[cfg(windows)]
pub fn enable_autostart() -> anyhow::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    let (key, _) = hkcu.create_subkey(path)?;

    // Récupérer le chemin de l'exécutable actuel
    let exe_path = std::env::current_exe()?;
    let exe_path_str = exe_path.to_string_lossy().to_string();

    // Ajouter l'entrée dans le registre
    key.set_value(APP_NAME, &exe_path_str)?;

    Ok(())
}

/// Désactiver le démarrage automatique
#[cfg(windows)]
pub fn disable_autostart() -> anyhow::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";

    match hkcu.open_subkey_with_flags(path, KEY_WRITE) {
        Ok(key) => {
            // Supprimer l'entrée (ignorer si elle n'existe pas)
            let _ = key.delete_value(APP_NAME);
            Ok(())
        }
        Err(_) => Ok(()), // Clé n'existe pas, rien à faire
    }
}

/// Vérifier si le démarrage automatique est activé
#[cfg(windows)]
pub fn is_autostart_enabled() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";

    if let Ok(key) = hkcu.open_subkey(path) {
        if let Ok(value) = key.get_value::<String, _>(APP_NAME) {
            // Vérifier que le chemin correspond à l'exe actuel
            if let Ok(current_exe) = std::env::current_exe() {
                return value == current_exe.to_string_lossy();
            }
        }
    }

    false
}

// Stubs pour plateformes non-Windows
#[cfg(not(windows))]
pub fn enable_autostart() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("Auto-start non supporté sur cette plateforme"))
}

#[cfg(not(windows))]
pub fn disable_autostart() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(not(windows))]
pub fn is_autostart_enabled() -> bool {
    false
}
