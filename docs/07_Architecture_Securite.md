# Architecture Sécurisée - xfinder
**Security Architecture & Threat Model**

---

## Vision sécurité

xfinder manipule des **données hautement sensibles** :
- Documents administratifs confidentiels
- Emails professionnels avec PII
- Fichiers RH, comptabilité, décisions politiques
- Chemins filesystem révélant structure organisation

**Principe fondamental :** **Zéro compromis sur la sécurité**

---

## Table des matières

1. [Modèle de menaces](#modèle-de-menaces)
2. [Architecture défense en profondeur](#architecture-défense-en-profondeur)
3. [Sécurité par composant](#sécurité-par-composant)
4. [Protection données sensibles](#protection-données-sensibles)
5. [Gestion intégrité index](#gestion-intégrité-index)
6. [Checklist sécurité](#checklist-sécurité)
7. [Plan réponse incidents](#plan-réponse-incidents)

---

## Modèle de menaces

### Acteurs malveillants

| Acteur | Capacités | Objectif | Probabilité | Impact |
|--------|-----------|----------|-------------|--------|
| **Attaquant externe** | Accès physique PC non verrouillé | Vol données index | Faible | Critique |
| **Malware local** | Exécution code, accès filesystem | Vol/chiffrement ransomware | Moyenne | Critique |
| **Attaquant réseau** | MitM, DNS poisoning | Compromission updates | Faible | Élevé |
| **Utilisateur malveillant** | Accès légitime app | Extraction massive données | Très faible | Moyen |
| **Insider IT** | Accès admin machine | Lecture index/logs | Très faible | Élevé |

### Vecteurs d'attaque

#### 1. **Compromission index/base de données**
- **Scénario** : Attaquant accède à `index.db` (non chiffré)
- **Impact** : Révèle tous chemins fichiers, métadonnées, contenu extrait
- **Mitigation** : Chiffrement DB optionnel (SQLCipher)

#### 2. **Injection code via recherche**
- **Scénario** : Query malveillante `'; DROP TABLE files; --`
- **Impact** : Corruption index, DoS
- **Mitigation** : Parameterized queries (rusqlite), validation inputs

#### 3. **Path traversal**
- **Scénario** : Ouvrir fichier `../../Windows/System32/config/SAM`
- **Impact** : Accès fichiers système sensibles
- **Mitigation** : Validation chemins, respect ACL Windows

#### 4. **Man-in-the-Middle updates**
- **Scénario** : Attaquant injecte update malveillante
- **Impact** : Installation malware
- **Mitigation** : Signing updates (Ed25519), HTTPS obligatoire

#### 5. **Memory corruption (Rust mitigation)**
- **Scénario** : Buffer overflow dans parsing PDF
- **Impact** : Code execution
- **Mitigation** : Rust memory safety + libs safe

#### 6. **XSS Frontend (Tauri)**
- **Scénario** : Affichage nom fichier `<script>alert(1)</script>.pdf`
- **Impact** : Code execution frontend
- **Mitigation** : Sanitization, CSP strict

#### 7. **Credential theft emails**
- **Scénario** : Dump `config.json` avec passwords IMAP
- **Impact** : Accès emails
- **Mitigation** : DPAPI Windows chiffrement

#### 8. **Ransomware ciblant index**
- **Scénario** : Malware chiffre `index.db`, demande rançon
- **Impact** : Perte index (réindexation nécessaire)
- **Mitigation** : Backup auto index, détection corruption

---

## Architecture défense en profondeur

### Layers de sécurité

```
┌─────────────────────────────────────────────┐
│   Layer 7 : Monitoring & Response          │ ← Détection intrusion
├─────────────────────────────────────────────┤
│   Layer 6 : Application Logic              │ ← Validation business
├─────────────────────────────────────────────┤
│   Layer 5 : Encryption at Rest             │ ← Chiffrement DB
├─────────────────────────────────────────────┤
│   Layer 4 : Input Validation               │ ← Sanitization
├─────────────────────────────────────────────┤
│   Layer 3 : Sandboxing (Tauri)             │ ← Isolation frontend
├─────────────────────────────────────────────┤
│   Layer 2 : Memory Safety (Rust)           │ ← Pas de buffer overflow
├─────────────────────────────────────────────┤
│   Layer 1 : OS Security (Windows ACL)      │ ← Permissions filesystem
└─────────────────────────────────────────────┘
```

### Principes appliqués

1. **Least Privilege** : App demande minimum permissions nécessaires
2. **Fail Secure** : En cas d'erreur → ferme accès, pas ouverture
3. **Defense in Depth** : Multiple barrières (pas une seule)
4. **Secure by Default** : Config par défaut = la plus sûre
5. **Privacy by Design** : Données locales uniquement, opt-in telemetry

---

## Sécurité par composant

### 1. Frontend (React + Tauri)

#### Menaces
- **XSS** : Injection script via noms fichiers
- **DOM-based attacks** : Manipulation DOM malveillante
- **IPC abuse** : Appels Tauri commands non autorisés

#### Mitigations

**CSP (Content Security Policy) strict**
```json
// tauri.conf.json
{
  "tauri": {
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'"
    }
  }
}
```

**Sanitization systématique**
```typescript
// JAMAIS faire ça (dangereux)
<div dangerouslySetInnerHTML={{ __html: result.filename }} />

// TOUJOURS sanitizer
import DOMPurify from 'dompurify';

function SearchResult({ result }) {
  const cleanFilename = DOMPurify.sanitize(result.filename);
  return <div>{cleanFilename}</div>;
}
```

**Validation Tauri commands**
```rust
#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    // 1. Validation format
    if !path.starts_with("C:\\") && !path.starts_with("D:\\") {
        return Err("Invalid path".to_string());
    }

    // 2. Canonicalize (résout .., symlinks)
    let canonical = std::fs::canonicalize(&path)
        .map_err(|_| "Path not found")?;

    // 3. Vérifie pas système critique
    if canonical.starts_with("C:\\Windows\\System32") {
        return Err("Access denied".to_string());
    }

    // 4. Vérifie ACL utilisateur
    if !user_has_access(&canonical)? {
        return Err("Permission denied".to_string());
    }

    // 5. Ouvre
    open::that(canonical).map_err(|e| e.to_string())
}
```

---

### 2. Backend (Rust)

#### Avantages Rust (built-in)
- ✅ **Memory safety** : Pas de buffer overflow, use-after-free
- ✅ **Thread safety** : Pas de data races (borrow checker)
- ✅ **No null pointers** : `Option<T>` explicite
- ✅ **Type safety** : Erreurs compilateur, pas runtime

#### Bonnes pratiques supplémentaires

**Validation inputs systématique**
```rust
// search_engine/query_parser.rs

pub fn parse_query(query: &str) -> Result<Query> {
    // 1. Limite taille (anti-DoS)
    if query.len() > 1000 {
        return Err(Error::QueryTooLong);
    }

    // 2. Whitelist caractères autorisés
    let allowed = query.chars().all(|c| {
        c.is_alphanumeric() ||
        c.is_whitespace() ||
        ".-_*\"".contains(c)
    });

    if !allowed {
        return Err(Error::InvalidCharacters);
    }

    // 3. Parse safe (pas d'injection SQL possible)
    Ok(Query::new(query))
}
```

**Gestion erreurs sécurisée**
```rust
// Ne JAMAIS exposer détails internes
fn index_file(path: &Path) -> Result<(), String> {
    // ❌ MAL : Révèle structure interne
    db.insert(path).map_err(|e| format!("DB error: {:?}", e))

    // ✅ BIEN : Message générique, log détails
    db.insert(path).map_err(|e| {
        error!("Failed to index {:?}: {}", path, e); // Log serveur
        "Indexing failed".to_string() // Message user
    })
}
```

**Dépendances sécurisées**
```toml
# Cargo.toml

# Audit automatique vulnérabilités
[dev-dependencies]
cargo-audit = "0.20"

# Build time checks
[profile.release]
overflow-checks = true  # Panic sur integer overflow
```

```bash
# CI/CD : Scan vulnérabilités
cargo audit
cargo clippy -- -D warnings
```

---

### 3. Database (SQLite)

#### Menaces
- **SQL Injection** : Queries malveillantes
- **Data leakage** : Index non chiffré sur disque
- **Corruption** : Crash pendant write

#### Mitigations

**Parameterized queries TOUJOURS**
```rust
// ❌ JAMAIS faire ça (injection SQL)
let query = format!("SELECT * FROM files WHERE path = '{}'", user_input);
conn.execute(&query, [])?;

// ✅ TOUJOURS faire ça
conn.execute(
    "SELECT * FROM files WHERE path = ?1",
    params![user_input],
)?;
```

**Chiffrement optionnel (SQLCipher)**
```rust
// database/mod.rs

pub struct Database {
    conn: rusqlite::Connection,
    encrypted: bool,
}

impl Database {
    pub fn new(path: &Path, encryption_key: Option<&str>) -> Result<Self> {
        let conn = Connection::open(path)?;

        if let Some(key) = encryption_key {
            // Active SQLCipher
            conn.execute(&format!("PRAGMA key = '{}'", key), [])?;
            Ok(Self { conn, encrypted: true })
        } else {
            Ok(Self { conn, encrypted: false })
        }
    }
}
```

**Protection intégrité**
```rust
// Détecte corruption DB
pub fn verify_integrity(conn: &Connection) -> Result<bool> {
    let result: String = conn.query_row(
        "PRAGMA integrity_check",
        [],
        |row| row.get(0)
    )?;

    Ok(result == "ok")
}

// Appelé au démarrage
if !verify_integrity(&conn)? {
    error!("Database corrupted! Restoring from backup...");
    restore_backup()?;
}
```

**Backup automatique**
```rust
use rusqlite::backup::Backup;

pub fn backup_database(src: &Connection, dest_path: &Path) -> Result<()> {
    let mut dest = Connection::open(dest_path)?;
    let backup = Backup::new(src, &mut dest)?;
    backup.run_to_completion(100, Duration::from_millis(250), None)?;
    Ok(())
}

// Backup quotidien auto
tokio::spawn(async {
    let mut interval = tokio::time::interval(Duration::from_secs(86400));
    loop {
        interval.tick().await;
        backup_database(&db, Path::new("backups/index_backup.db"))?;
    }
});
```

---

### 4. Filesystem Access

#### Menaces
- **Path traversal** : Accès fichiers hors scope
- **Symlink attacks** : Liens symboliques malveillants
- **ACL bypass** : Lecture fichiers interdits

#### Mitigations

**Validation chemins stricte**
```rust
use std::path::{Path, PathBuf};

pub struct SecurePath {
    canonical: PathBuf,
}

impl SecurePath {
    pub fn new(user_path: &str, allowed_roots: &[PathBuf]) -> Result<Self> {
        // 1. Canonicalize (résout .., symlinks, etc.)
        let canonical = std::fs::canonicalize(user_path)
            .map_err(|_| Error::InvalidPath)?;

        // 2. Vérifie dans roots autorisées
        let in_allowed = allowed_roots.iter().any(|root| {
            canonical.starts_with(root)
        });

        if !in_allowed {
            return Err(Error::PathOutsideScope);
        }

        // 3. Blacklist dossiers système
        let blacklist = [
            "C:\\Windows\\System32",
            "C:\\Windows\\SysWOW64",
            "C:\\Program Files",
        ];

        for blocked in &blacklist {
            if canonical.starts_with(blocked) {
                return Err(Error::SystemPathBlocked);
            }
        }

        Ok(Self { canonical })
    }

    pub fn as_path(&self) -> &Path {
        &self.canonical
    }
}

// Usage
let safe_path = SecurePath::new(
    user_input,
    &[PathBuf::from("C:\\Users\\Admin\\Documents")]
)?;

let content = std::fs::read_to_string(safe_path.as_path())?;
```

**Respect ACL Windows**
```rust
#[cfg(target_os = "windows")]
use windows::Win32::Storage::FileSystem::GetFileSecurityW;

fn user_has_read_access(path: &Path) -> Result<bool> {
    // Vérifie ACL Windows (Security Descriptor)
    // Complexe, utiliser lib `windows-acl` ou fallback

    // Fallback simple : tente lecture
    match std::fs::metadata(path) {
        Ok(metadata) => Ok(metadata.permissions().readonly() == false),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(false),
        Err(e) => Err(e.into()),
    }
}
```

---

### 5. Email Credentials

#### Menaces
- **Plaintext passwords** : Config.json lisible
- **Memory dumps** : Passwords en RAM
- **Log leakage** : Passwords dans logs

#### Mitigations

**Chiffrement DPAPI (Windows)**
```rust
use windows::Win32::Security::Cryptography::{
    CryptProtectData, CryptUnprotectData, CRYPTOAPI_BLOB
};

pub fn encrypt_password(plaintext: &str) -> Result<Vec<u8>> {
    let data = plaintext.as_bytes();

    let mut blob_in = CRYPTOAPI_BLOB {
        cbData: data.len() as u32,
        pbData: data.as_ptr() as *mut u8,
    };

    let mut blob_out = CRYPTOAPI_BLOB::default();

    unsafe {
        CryptProtectData(
            &mut blob_in,
            None,
            None,
            None,
            None,
            0,
            &mut blob_out,
        )?;
    }

    let encrypted = unsafe {
        std::slice::from_raw_parts(blob_out.pbData, blob_out.cbData as usize)
    }.to_vec();

    Ok(encrypted)
}

pub fn decrypt_password(encrypted: &[u8]) -> Result<String> {
    let mut blob_in = CRYPTOAPI_BLOB {
        cbData: encrypted.len() as u32,
        pbData: encrypted.as_ptr() as *mut u8,
    };

    let mut blob_out = CRYPTOAPI_BLOB::default();

    unsafe {
        CryptUnprotectData(
            &mut blob_in,
            None,
            None,
            None,
            None,
            0,
            &mut blob_out,
        )?;
    }

    let decrypted = unsafe {
        std::slice::from_raw_parts(blob_out.pbData, blob_out.cbData as usize)
    };

    Ok(String::from_utf8(decrypted.to_vec())?)
}
```

**Stockage config sécurisé**
```json
// config.json (passwords chiffrés DPAPI)
{
  "email_sources": [
    {
      "type": "imap",
      "server": "imap.example.com",
      "username": "admin@example.com",
      "password_encrypted": "AQAAANCMnd8BFdERjHoAwE/Cl+sBAAAA..." // Base64 DPAPI
    }
  ]
}
```

**Zeroization mémoire**
```rust
use zeroize::Zeroize;

struct Credentials {
    username: String,
    password: String, // Sera effacé de la RAM
}

impl Drop for Credentials {
    fn drop(&mut self) {
        self.password.zeroize(); // Écrase mémoire avec 0x00
    }
}
```

---

### 6. Network (Updates)

#### Menaces
- **MitM attacks** : Interception/modification updates
- **Downgrade attacks** : Force version vulnérable
- **DNS spoofing** : Redirige vers serveur malveillant

#### Mitigations

**Signing updates (Ed25519)**
```rust
// build.rs (génère keypair)
use ed25519_dalek::{Keypair, PublicKey, Signature};

// Server side : signe update
fn sign_update(update_file: &[u8], private_key: &Keypair) -> Signature {
    private_key.sign(update_file)
}

// Client side : vérifie signature
fn verify_update(update_file: &[u8], signature: &Signature, public_key: &PublicKey) -> bool {
    public_key.verify_strict(update_file, signature).is_ok()
}
```

**Tauri updater config**
```json
// tauri.conf.json
{
  "tauri": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://updates.xfinder.app/{{target}}/{{current_version}}"
      ],
      "dialog": true,
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEFCQ0RFRgo="
    }
  }
}
```

**HTTPS obligatoire + pinning certificat**
```rust
use reqwest::ClientBuilder;

let client = ClientBuilder::new()
    .https_only(true)
    .min_tls_version(reqwest::tls::Version::TLS_1_3)
    .build()?;
```

---

## Protection données sensibles

### 1. Données au repos (At Rest)

| Donnée | Stockage | Chiffrement | Justification |
|--------|----------|-------------|---------------|
| **Index métadonnées** | `index.db` | Optionnel (SQLCipher) | Si très sensible, activer |
| **Contenu extrait** | `content.db` | Optionnel | Contient texte docs |
| **Embeddings** | `vectors.db` | Non (déjà opaque) | Vecteurs = pas lisibles humains |
| **Passwords emails** | `config.json` | **Obligatoire (DPAPI)** | Critique |
| **Logs** | `logs/xfinder.log` | Non, mais **sanitizés** | Pas de PII/passwords |

### 2. Données en transit (In Transit)

| Communication | Protocole | Chiffrement | Validation |
|---------------|-----------|-------------|------------|
| **Updates** | HTTPS | TLS 1.3 | Signature Ed25519 |
| **IMAP emails** | IMAPS | TLS 1.2+ | Certificat serveur |
| **IPC Tauri** | Interne | N/A | Validation commands |

### 3. Données en mémoire (In Memory)

**Pratiques sécurisées :**
```rust
// 1. Limiter durée vie credentials
{
    let creds = decrypt_credentials()?;
    connect_imap(&creds)?;
    // creds.drop() → zeroize automatique
}

// 2. Pas de logs passwords
fn connect_imap(creds: &Credentials) {
    info!("Connecting to IMAP: {}", creds.username);
    // ❌ JAMAIS : info!("Password: {}", creds.password);
}

// 3. Effacer variables sensibles
let mut api_key = get_api_key();
use_api_key(&api_key);
api_key.zeroize(); // Efface de la RAM
```

---

## Gestion intégrité index

### Problème : Fichiers déplacés/supprimés

**Sans watchdog** (spotlight_windows actuel) :
- Index référence `C:\Docs\Rapport.pdf`
- User déplace → `D:\Archives\Rapport.pdf`
- Recherche retourne chemin invalide → **Clic = erreur**

**Avec watchdog xfinder** :

#### Scénario 1 : Fichier déplacé
```rust
// watchdog/event_handler.rs

async fn handle_renamed(from: PathBuf, to: PathBuf) {
    // 1. Récupère file_id via ancien chemin
    let file_id = db.get_file_id_by_path(&from).await?;

    // 2. Met à jour chemin dans DB
    db.update_path(&file_id, &to).await?;

    // 3. Embeddings gardent même file_id → pas de régénération
    info!("Updated path: {} → {}", from.display(), to.display());

    // 4. Tantivy index : update document
    tantivy_index.update_path(&file_id, &to).await?;
}
```

#### Scénario 2 : Fichier supprimé
```rust
async fn handle_deleted(path: PathBuf) {
    let file_id = db.get_file_id_by_path(&path).await?;

    // 1. Supprime de DB
    db.delete_file(&file_id).await?;

    // 2. Supprime embeddings associés
    vector_db.delete_embeddings(&file_id).await?;

    // 3. Supprime de Tantivy
    tantivy_index.delete_document(&file_id).await?;

    info!("Removed from index: {}", path.display());
}
```

#### Scénario 3 : Fichier modifié
```rust
async fn handle_modified(path: PathBuf) {
    let file_id = db.get_file_id_by_path(&path).await?;

    // 1. Calcule nouveau hash
    let new_hash = hash_file(&path).await?;
    let old_hash = db.get_hash(&file_id).await?;

    if new_hash == old_hash {
        // Juste metadata changed (date) → skip
        return Ok(());
    }

    // 2. Contenu changé → réindexation
    info!("Content changed, re-indexing: {}", path.display());

    // 3. Re-extract contenu
    let new_content = extract_content(&path).await?;
    db.update_content(&file_id, &new_content).await?;

    // 4. Régénère embeddings
    let new_embeddings = generate_embeddings(&new_content).await?;
    vector_db.update_embeddings(&file_id, &new_embeddings).await?;
}
```

### Nettoyage orphelins (maintenance)

```rust
// Appelé 1x/jour ou manuellement
pub async fn cleanup_orphans(db: &Database) -> Result<CleanupStats> {
    let mut stats = CleanupStats::default();

    // 1. Vérifie tous fichiers indexés
    let indexed_files = db.get_all_files().await?;

    for file in indexed_files {
        // 2. Vérifie existence sur disque
        if !Path::new(&file.path).exists() {
            // Supprimé sans événement watchdog (PC éteint, réseau déconnecté)
            db.delete_file(&file.id).await?;
            vector_db.delete_embeddings(&file.id).await?;
            stats.removed_count += 1;
        }
    }

    // 3. Vérifie embeddings orphelins (pas de file_id correspondant)
    let orphan_embeddings = vector_db.find_orphans().await?;
    for orphan in orphan_embeddings {
        vector_db.delete_embeddings(&orphan).await?;
        stats.orphan_embeddings_removed += 1;
    }

    // 4. Optimise DB (VACUUM)
    db.vacuum().await?;

    Ok(stats)
}
```

---

## Checklist sécurité

### Avant chaque commit

- [ ] `cargo clippy -- -D warnings` (pas de warnings)
- [ ] `cargo audit` (pas de vulnérabilités connues)
- [ ] Pas de `unwrap()` ou `expect()` dans code production (gestion erreurs)
- [ ] Validation inputs sur nouvelles fonctions
- [ ] Pas de logs passwords/secrets

### Avant chaque release

- [ ] Scan dépendances : `cargo audit`
- [ ] Review code sécurité (peer review)
- [ ] Tests fuzzing inputs malveillants
- [ ] Tests path traversal
- [ ] Tests SQL injection (si nouvelles queries)
- [ ] Vérif CSP frontend (`tauri.conf.json`)
- [ ] Signing certificat Windows valide
- [ ] Update signature validée (Ed25519)
- [ ] Backup DB automatique fonctionne
- [ ] Cleanup orphans testé

### Tous les mois

- [ ] Mise à jour dépendances Rust (`cargo update`)
- [ ] Revue nouvelles CVE publiées
- [ ] Tests penetration (scan nmap, etc.)
- [ ] Revue logs erreurs production

---

## Plan réponse incidents

### Incident 1 : Vulnérabilité découverte

**Procédure :**
1. **Évaluation** : Criticité (CVSS score), exploitation possible ?
2. **Hotfix** : Patch code, tests
3. **Release urgente** : v1.0.1 dans les 24-48h
4. **Communication** : GitHub Security Advisory, email utilisateurs
5. **Post-mortem** : Comment éviter futur ?

### Incident 2 : Index corrompu utilisateur

**Procédure :**
1. **Détection** : `PRAGMA integrity_check` au démarrage
2. **Notification** : Dialog "Index corrompu, restauration backup..."
3. **Restauration** : Copie `backup/index_backup.db` → `index.db`
4. **Si pas de backup** : Réindexation complète (longue)
5. **Log** : Envoi telemetry anonyme (opt-in) pour debug

### Incident 3 : Malware détecté dans app

**Procédure :**
1. **Vérification** : Faux positif antivirus ou vrai positif ?
2. **Si vrai** : Retrait immédiat release, investigation source
3. **Si faux positif** : Soumission VirusTotal, contact éditeur AV
4. **Communication** : Transparence utilisateurs

### Incident 4 : Credentials IMAP leakés

**Procédure :**
1. **Notification** : User concerné immédiatement
2. **Révocation** : Changer password IMAP
3. **Investigation** : Comment leak ? (log, dump mémoire, config partagée)
4. **Amélioration** : Renforcer chiffrement/zeroization

---

## Métriques sécurité

### KPIs à suivre

| Métrique | Cible | Mesure |
|----------|-------|--------|
| **CVE dépendances** | 0 critique, <3 high | `cargo audit` |
| **Code warnings** | 0 | `cargo clippy` |
| **Unsafe Rust** | <1% codebase | `cargo-geiger` |
| **Test coverage** | >80% | tarpaulin |
| **Update adoption** | >70% en 1 mois | Telemetry |
| **Crashes sécurité** | 0 | Crash reports |

---

## Ressources & références

### Standards
- **OWASP Top 10** : https://owasp.org/www-project-top-ten/
- **CWE (Common Weakness Enumeration)** : https://cwe.mitre.org/
- **NIST Cybersecurity Framework** : https://www.nist.gov/cyberframework

### Outils
- `cargo-audit` : Scan vulnérabilités Rust
- `cargo-deny` : Politique dépendances
- `cargo-geiger` : Détecte unsafe code
- `rustsec` : Advisory database

### Rust Security
- **Rust Security WG** : https://www.rust-lang.org/governance/wgs/wg-security-response
- **RustSec Database** : https://rustsec.org/

---

## Conclusion

**Sécurité = non négociable pour xfinder**

Grâce à :
1. **Rust** : Élimine 70% vulnérabilités communes (memory safety)
2. **Tauri** : Sandboxing frontend, CSP strict
3. **Defense in depth** : Multiples layers protection
4. **Watchdog** : Intégrité index maintenue automatiquement
5. **Chiffrement** : Credentials, optionnel DB
6. **Validation** : Inputs, chemins, queries
7. **Audits** : Continus, automatisés

**Rappel** : Sécurité = processus continu, pas état final

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
**Prochaine revue sécurité :** Phase 1 MVP (semaine 8)
