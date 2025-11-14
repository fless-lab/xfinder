// src/hash.rs
// Module pour le hashing blake3 des fichiers (détection de doublons)

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Calcule le hash blake3 d'un fichier
///
/// Utilise un buffer de 1MB pour lire le fichier par morceaux
/// et calculer le hash de manière efficace sans charger tout en mémoire.
///
/// Retourne le hash sous forme de string hexadécimale (64 caractères)
pub fn hash_file(path: &Path) -> Result<String> {
    let file = File::open(path)
        .with_context(|| format!("Impossible d'ouvrir le fichier: {:?}", path))?;

    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = vec![0u8; 1024 * 1024]; // Buffer de 1MB

    loop {
        let bytes_read = reader.read(&mut buffer)
            .with_context(|| format!("Erreur lecture fichier: {:?}", path))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(hash.to_hex().to_string())
}

/// Calcule le hash blake3 rapide d'un fichier (premiers 1MB seulement)
///
/// Utilisé pour une détection rapide de doublons sur de gros fichiers.
/// Pour une détection précise, utiliser `hash_file()`.
pub fn hash_file_fast(path: &Path) -> Result<String> {
    let file = File::open(path)
        .with_context(|| format!("Impossible d'ouvrir le fichier: {:?}", path))?;

    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = vec![0u8; 1024 * 1024]; // Lire max 1MB

    let bytes_read = reader.read(&mut buffer)
        .with_context(|| format!("Erreur lecture fichier: {:?}", path))?;

    hasher.update(&buffer[..bytes_read]);

    let hash = hasher.finalize();
    Ok(hash.to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_hash_file() {
        // Créer un fichier temporaire
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("xfinder_hash_test.txt");

        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"Hello, World!").unwrap();
        drop(file);

        // Calculer le hash
        let hash = hash_file(&test_file).unwrap();

        // Le hash blake3 de "Hello, World!" est connu
        assert_eq!(hash.len(), 64); // 32 bytes = 64 hex chars

        // Nettoyer
        std::fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_hash_file_fast() {
        // Créer un fichier temporaire
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("xfinder_hash_fast_test.txt");

        let mut file = File::create(&test_file).unwrap();
        file.write_all(b"Fast hash test").unwrap();
        drop(file);

        // Calculer le hash rapide
        let hash = hash_file_fast(&test_file).unwrap();

        assert_eq!(hash.len(), 64);

        // Nettoyer
        std::fs::remove_file(&test_file).unwrap();
    }

    #[test]
    fn test_same_content_same_hash() {
        let temp_dir = std::env::temp_dir();
        let file1 = temp_dir.join("xfinder_hash_test1.txt");
        let file2 = temp_dir.join("xfinder_hash_test2.txt");

        // Créer deux fichiers identiques
        let content = b"Duplicate content test";
        std::fs::write(&file1, content).unwrap();
        std::fs::write(&file2, content).unwrap();

        // Calculer les hash
        let hash1 = hash_file(&file1).unwrap();
        let hash2 = hash_file(&file2).unwrap();

        // Les hash doivent être identiques
        assert_eq!(hash1, hash2);

        // Nettoyer
        std::fs::remove_file(&file1).unwrap();
        std::fs::remove_file(&file2).unwrap();
    }
}
