# Audio File Player (Rust)

Un lecteur audio performant et extensible en ligne de commande. Ce projet implémente un **pipeline de traitement audio** modulaire permettant le décodage, la gestion d'état et la lecture en temps réel sur le matériel système.

---

## 🛠 Architecture & Design

Le projet est découpé en modules distincts pour respecter le principe de responsabilité unique :

- **`decoder/`** : Utilise `Symphonia` pour ouvrir les fichiers et extraire les échantillons audio par morceaux (*chunks*) afin d'optimiser l'usage mémoire.
- **`pipeline/`** : Le "cerveau" du lecteur. Il gère l'état global (`PlayerState`) et la communication entre les threads via des canaux (`mpsc`).
- **`output/`** : Abstraction de la sortie audio. Le trait `Output` définit le contrat pour les moteurs de rendu, permettant d'étendre facilement le projet vers d'autres backends.
- **`ui/`** : Interface utilisateur interactive en ligne de commande (CLI).

### Flux de données

```text
CLI → Output (CPAL) → Pipeline (État/Volume) ←→ Thread Audio
                              ↑
                        Decoder (Chunks f32)
```

### Structure du projet

```
audio-file-player/
├── src/
│   ├── decoder/mod.rs       — décodage audio via Symphonia
│   ├── output/
│   │   ├── mod.rs
│   │   ├── output.rs        — trait Output
│   │   └── cpal_output.rs   — implémentation CPAL
│   ├── pipeline/mod.rs      — gestion d'état et de volume
│   ├── ui/mod.rs            — interface CLI
│   ├── lib.rs
│   └── main.rs
├── tests/
│   ├── audio_pipeline_test.rs
│   ├── cpal_output_test.rs
│   └── decoder_test.rs
└── Cargo.toml
```

---

## 🚀 Installation & Lancement

### Dépendances système (Linux)

La bibliothèque `cpal` nécessite les fichiers de développement ALSA :

```bash
sudo apt install libasound2-dev
```

### Dépendances Rust (`Cargo.toml`)

```toml
[dependencies]
# Gestion des erreurs simplifiée
anyhow = "1.0"

# Bibliothèque audio de bas niveau pour la sortie son
cpal = "0.15"

# Bibliothèque de décodage audio (support MP3 et WAV activé)
symphonia = { version = "0.5", features = ["mp3", "wav", "isomp4"] }

# Gestion du terminal (nécessaire pour l'interaction clavier dans main.rs)
crossterm = "0.27"
```

### Exécution

```bash
cargo run -- chemin/vers/votre_musique.mp3
```

Formats supportés : MP3, WAV, FLAC (selon les features Symphonia activées).

---

## ⌨️ Commandes du lecteur

| Commande | Action |
|----------|--------|
| `p` | Mettre la lecture en pause |
| `l` | Reprendre la lecture |
| `s` | Stop — arrête et revient au début |
| `v <valeur>` | Régler le volume, ex: `v 0.5` pour 50% |
| `q` | Quitter l'application |

---

## 🧪 Tests & Qualité

### Lancer tous les tests

```bash
cargo test
```

### Lancer les tests nécessitant un fichier audio réel

```bash
# ou tester avec un fichier audio
export TEST_AUDIO_FILE=chemin/vers/fichier_existant.mp3
cargo test -- --include-ignored
```

### Structure des tests

| Fichier | Type | Ce qui est testé |
|---------|------|-----------------|
| `src/pipeline/mod.rs` | Unitaire | Cycle d'états Playing / Paused / Stopped, volume |
| `src/output/cpal_output.rs` | Unitaire | Initialisation, commandes vers le pipeline |
| `tests/audio_pipeline_test.rs` | Intégration | Interactions Pipeline ↔ CpalOutput |
| `tests/cpal_output_test.rs` | Intégration | Création du flux, échec propre sans décodeur |
| `tests/decoder_test.rs` | Intégration | Ouverture, lecture par chunks, reset |

---

## 📚 Documentation technique

Générer et consulter la documentation complète des APIs :

```bash
cargo doc --open
```

---

## 🦀 Concepts Rust exploités

- **Traits** : Le trait `Output` abstrait le moteur de rendu audio pour permettre l'extensibilité vers d'autres backends (JACK, PulseAudio, etc.).
- **Concurrence** : `Arc<Mutex<T>>` pour partager le décodeur et l'état entre le thread UI et le thread audio CPAL de façon sûre.
- **Canaux `mpsc`** : Communication non bloquante entre l'interface utilisateur et le thread de contrôle du pipeline.
- **Gestion d'erreurs** : `Result` et la crate `anyhow` pour une remontée d'erreurs robuste et lisible.
- **Génériques** : La fonction `run_cli<T: Output>` accepte n'importe quelle implémentation de sortie audio.

---

> Ce projet a été réalisé dans le cadre du module **COA — Rust**.