# Audio File Player

Lecteur audio en ligne de commande écrit en Rust, utilisant **Symphonia** pour le décodage et **CPAL** pour la sortie audio.

## Architecture

```
src/
├── decoder/   — ouverture et décodage des fichiers audio (Symphonia)
├── output/    — sortie audio via CPAL, trait Output
├── pipeline/  — gestion de l'état (Playing / Paused / Stopped) et du volume
└── ui/        — boucle de commandes clavier (CLI)
```

Les composants communiquent ainsi :

```
CLI → Output → Pipeline ←→ Callback audio (thread CPAL)
                   ↑
               Decoder (lecture par morceaux)
```

## Prérequis

- Rust 2024 edition
- Un périphérique de sortie audio disponible

Dépendances principales (`Cargo.toml`) :

```toml
[dependencies]
cpal      = "..."
symphonia = { version = "...", features = ["mp3", "flac", "wav"] }
anyhow    = "..."
```

## Lancer le lecteur

```bash
cargo run -- chemin/vers/fichier.mp3
```

## Commandes disponibles

| Touche | Action |
|--------|--------|
| `p` | Pause |
| `l` | Reprendre la lecture |
| `s` | Stop (retour au début) |
| `v 0.5` | Régler le volume (0.0 à 1.0) |
| `q` | Quitter |

## Tests

### Lancer tous les tests (sans fichier audio)

```bash
cargo test
```

### Lancer les tests nécessitant un fichier audio réel

```bash
export TEST_AUDIO_FILE=chemin/vers/sample.mp3
cargo test -- --include-ignored
```

### Structure des tests

| Fichier | Ce qui est testé |
|---------|-----------------|
| `src/pipeline/mod.rs` | Cycle d'états du Pipeline (unitaire) |
| `src/output/cpal_output.rs` | Initialisation et commandes de CpalOutput (unitaire) |
| `tests/audio_pipeline_test.rs` | Interactions Pipeline ↔ CpalOutput (intégration) |
| `tests/cpal_output_test.rs` | Création du flux, échec sans décodeur (intégration) |
| `tests/decoder_test.rs` | Ouverture, lecture, reset du Decoder (intégration) |