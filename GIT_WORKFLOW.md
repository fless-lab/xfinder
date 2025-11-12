# 📝 Git Workflow - xfinder

**Guide pour commit régulièrement sans se perdre**

---

## 🎯 Philosophie

**Commit SOUVENT = Sauvegarde régulière**

- ✅ Après chaque petite feature qui marche
- ✅ Avant de tester quelque chose de risqué
- ✅ En fin de session de code
- ✅ Même si c'est pas parfait

**Pas besoin que ce soit parfait pour commit !**

---

## 🚀 Workflow simple (tous les jours)

### 1. Avant de commencer à coder

```bash
# Vérifie où tu en es
git status

# Si besoin, crée une branche pour ta feature
git checkout -b feature/search-tantivy
# (Optionnel - pour features importantes)
```

### 2. Pendant que tu codes

```bash
# Toutes les 30 min - 1h, ou dès qu'une petite chose marche :

# 1. Vérifie ce qui a changé
git status

# 2. Ajoute les fichiers modifiés
git add .

# 3. Commit avec message simple
git commit -m "feat: ajout recherche tantivy basique"

# C'est tout !
```

### 3. En fin de journée

```bash
# Commit final du jour
git add .
git commit -m "wip: fin session - tantivy indexation fonctionne"

# Push vers GitHub (si tu as un repo distant)
git push origin main
```

---

## 📋 Format messages de commit

### Structure simple

```
<type>: <description courte>

[optionnel] Détails supplémentaires
```

### Types de commit

| Type | Quand l'utiliser | Exemple |
|------|------------------|---------|
| `feat` | Nouvelle feature | `feat: ajout barre recherche egui` |
| `fix` | Correction bug | `fix: crash lors recherche vide` |
| `refactor` | Améliorer code existant | `refactor: simplification module search` |
| `docs` | Documentation | `docs: ajout commentaires tantivy` |
| `test` | Tests | `test: ajout tests indexation` |
| `wip` | Work in progress (fin session) | `wip: en cours indexation SQLite` |
| `chore` | Maintenance | `chore: mise à jour dependencies` |

### Exemples concrets

```bash
# Feature qui marche
git commit -m "feat: indexation de 100 fichiers avec Tantivy"

# Bug corrigé
git commit -m "fix: résultats s'affichent maintenant dans la liste"

# Fin de session (pas fini)
git commit -m "wip: en cours config UI - pas encore terminé"

# Amélioration
git commit -m "refactor: nettoyage code search module"

# Tests ajoutés
git commit -m "test: ajout test search vitesse <100ms"
```

---

## ⏱️ Quand commit ? (règles simples)

### ✅ COMMIT maintenant si :
- ✅ Une petite feature fonctionne (même basique)
- ✅ Tu as corrigé un bug
- ✅ Ça compile sans erreur (`cargo build`)
- ✅ Tu vas tester quelque chose de nouveau (backup avant)
- ✅ Fin de session de code
- ✅ Avant de partir déjeuner/pause

### 🔄 COMMIT même si :
- 🔄 C'est pas parfait
- 🔄 Y'a des `TODO` dans le code
- 🔄 C'est juste un prototype
- 🔄 Tu vas refactoriser plus tard

### ⏸️ PAS BESOIN de commit si :
- ⏸️ Juste modifié 1 ligne (attends un peu)
- ⏸️ Ça compile pas (corrige d'abord)
- ⏸️ C'est juste un test qui marche pas (annule les changements)

---

## 🔄 Commandes utiles quotidiennes

### Vérifier l'état

```bash
# Qu'est-ce qui a changé ?
git status

# Détails des modifications
git diff

# Historique des commits
git log --oneline -10  # 10 derniers commits
```

### Annuler des changements

```bash
# Annuler modifications d'un fichier (AVANT git add)
git checkout -- src/main.rs

# Annuler TOUS les changements (ATTENTION !)
git reset --hard

# Annuler dernier commit (garde les changements)
git reset --soft HEAD~1
```

### Sauvegarder sans commit

```bash
# Mettre de côté (si tu veux tester autre chose)
git stash

# Récupérer ce qui était de côté
git stash pop
```

---

## 📅 Routine quotidienne recommandée

### Matin (début session)

```bash
cd D:\DataLab\xfinder

# Vérifie où tu en es
git status
git log --oneline -5

# Commence à coder...
```

### Pendant la journée (toutes les heures)

```bash
# Petite feature finie ?
git add .
git commit -m "feat: [ce que tu viens de faire]"

# Continue à coder...
```

### Soir (fin session)

```bash
# Commit tout ce qui reste
git add .
git commit -m "wip: fin jour - [où tu en es]"

# Optionnel : Push vers GitHub
git push origin main
```

---

## 🌳 Gestion branches (optionnel au début)

### Quand utiliser les branches ?

**Au début (Semaines 1-4) : PAS BESOIN**
- Reste sur `main`
- Commit directement

**Plus tard (Semaines 5+) : UTILE**
- Feature importante = branche
- Expérimentation = branche

### Créer une branche

```bash
# Nouvelle feature
git checkout -b feature/ocr-tesseract

# Code, commit...
git add .
git commit -m "feat: intégration Tesseract"

# Fusionner dans main quand fini
git checkout main
git merge feature/ocr-tesseract

# Supprimer branche
git branch -d feature/ocr-tesseract
```

---

## 💾 Backup sur GitHub (recommandé)

### Setup une fois

```bash
# Sur GitHub.com : Créer repo "xfinder"

# Dans ton terminal :
cd D:\DataLab\xfinder
git remote add origin https://github.com/TON_USERNAME/xfinder.git

# Premier push
git push -u origin main
```

### Ensuite (quotidien)

```bash
# Push après tes commits
git push

# Pull si tu codes sur plusieurs PCs
git pull
```

---

## 🆘 Problèmes courants

### "Je veux annuler mes derniers changements"

```bash
# Annuler TOUT (ATTENTION - perte définitive)
git reset --hard

# Annuler 1 fichier
git checkout -- src/fichier.rs
```

### "J'ai commit trop tôt"

```bash
# Annuler dernier commit (garde les fichiers modifiés)
git reset --soft HEAD~1

# Modifie ce que tu veux...

# Re-commit
git commit -m "feat: version corrigée"
```

### "J'ai oublié d'ajouter un fichier au commit"

```bash
# Ajoute le fichier oublié
git add fichier_oublie.rs

# Amend le dernier commit
git commit --amend --no-edit
```

### "Ça compile plus, je veux revenir en arrière"

```bash
# Liste les commits récents
git log --oneline -10

# Reviens au commit qui marchait (ex: abc123)
git checkout abc123

# Si ça marche, tu peux rester là
# Sinon retourne au dernier :
git checkout main
```

---

## 📊 Exemple historique commit (première semaine)

```bash
git log --oneline

abc1234 wip: fin jour 5 - watchdog fonctionne
def5678 feat: détection ajout fichier watchdog
ghi9012 feat: affichage résultats dans liste scrollable
jkl3456 fix: crash quand recherche vide
mno7890 feat: recherche Tantivy retourne résultats
pqr1234 feat: indexation 100 fichiers test
stu5678 feat: setup Tantivy index basique
vwx9012 feat: hello world egui fonctionne
yz01234 docs: ajout documentation complète
```

**= 1 commit par petite étape qui marche ✅**

---

## ✅ Checklist quotidienne

### Matin
- [ ] `git status` (vérifier où j'en suis)
- [ ] `git log --oneline -5` (voir mes derniers commits)

### Pendant la journée (après chaque feature)
- [ ] Tester que ça compile : `cargo build`
- [ ] Tester que ça marche : `cargo run`
- [ ] Si OK : `git add .`
- [ ] `git commit -m "feat: [ce que j'ai fait]"`

### Soir
- [ ] Commit final : `git add .`
- [ ] `git commit -m "wip: fin jour - [état actuel]"`
- [ ] `git push` (si GitHub configuré)

---

## 💡 Conseils

### ✅ BONNE pratique
```bash
# Commit spécifique et testé
git add src/search/tantivy_index.rs
cargo test
git commit -m "feat: indexation Tantivy fonctionne"
```

### ❌ MAUVAISE pratique
```bash
# Commit géant en fin de semaine
git add .
git commit -m "plein de trucs"
# (Tu sauras plus ce que tu as fait)
```

---

## 🎯 Résumé ultra-simple

**3 commandes essentielles :**

```bash
# 1. Vérifier
git status

# 2. Sauvegarder
git add .
git commit -m "feat: ce que tu viens de faire"

# 3. Backup cloud (optionnel)
git push
```

**Fréquence : Toutes les heures ou dès qu'une petite chose marche ✅**

---

## 📝 Template message commit rapide

**Copie/colle et adapte :**

```bash
# Feature nouvelle
git commit -m "feat: ajout [nom feature]"

# Bug corrigé
git commit -m "fix: [description bug] corrigé"

# Fin de journée
git commit -m "wip: fin jour - [module en cours] - [statut]"

# Tests ajoutés
git commit -m "test: ajout tests [module]"

# Amélioration code
git commit -m "refactor: amélioration [module]"
```

---

**Commit souvent = Moins de stress = Travail sécurisé ! 💪**

---

**Document version :** 1.0
**Dernière mise à jour :** 2025-11-12
