# Rust Learning Workspace

Depot de TP et d'exemples pour apprivoiser Rust via une serie de petits crates autonomes. Chaque dossier contient son propre `Cargo.toml` et une entree principale `src/main.rs` (et parfois une lib ou des tests) qui illustre un concept du langage ou de la bibliotheque standard.

## Prerequis
- Rust et Cargo installs avec `rustup`.
- Toolchain stable par defaut, sauf mention contraire.
- Pour `structs_lv_2`, activer le toolchain nightly (le crate utilise la feature instable `box_patterns`).

```
rustup install nightly
rustup override set nightly # uniquement si vous travaillez dans structs_lv_2/
```

## Lancer un exemple
Chaque projet est independant. Naviguez dans le dossier cible puis lancez `cargo run` :

```
cd channel
cargo run
```

Certains crates exposent aussi des tests (`cargo test`).

## Apercu rapide des dossiers

| Dossier | Type | Concepts clefs | Notes |
| --- | --- | --- | --- |
| `channel/` | binaire | Threads, `std::sync::mpsc`, temporisation | Compare une version tokio (commentee) et une version std. |
| `lifetime/` | binaire | Lifetimes explicites, concatenation de `String` | Montre comment retourner une reference valide depuis une fonction. |
| `matching/` | binaire | `match` sur `Option`, boucles sur vecteurs | Recherche le premier nombre pair dans des listes imbriquees. |
| `operation/` | binaire | `enum`, pattern matching, `Result` | Mini calculatrice arithmetique avec gestion d'erreur division par zero. |
| `operator/` | binaire + lib | Modules, erreurs personnalisees, propag. `?` | `process_user_data` compose plusieurs Result pour charger des donnees utilisateurs. |
| `options/` | binaire | `Result`, validation d'entrees | Division sure qui remonte une erreur si denomin. nul. |
| `overflow/` | binaire | Operations sur entiers, `wrapping_add` | Demontre l'arithmetique avec depassement sur `u8`. |
| `read-line/` | binaire | Lecture standard, nettoyage d'entree | Invite l'utilisateur et salue en retirant les sauts de ligne. |
| `smart_pointeur/` | binaire | `Arc`, `Mutex`, macros, threads | Macros maison pour factoriser les verrous et delais aleatoires. |
| `structs_lv_1/` | binaire | Structs, impl, pattern matching | `Person` avec methodes, controle de la majorite. |
| `structs_lv_2/` | lib + binaire + tests | Enums, macros, CSV, liste chainee | Mediatheque miniature, macro `vec_mediaitem!`, necessite nightly. |

## Focus projets

### channel/
Mise en pratique des canaux mpsc. Un producteur et un consommateur s'executent dans des threads distincts et synchronisent leurs operations avec des pauses (`thread::sleep`). Le code contient des variantes `tokio` commentees pour basculer vers l'ecosysteme async.

### lifetime/
Experimente la notion de duree de vie en concatenant deux `String` et en retournant une reference mutable. Illustrations de concatenations et d'emprunts successifs (`&String`, `&mut String`).

### matching/
Combine `Option` et pattern matching pour extraire la premiere valeur paire dans des vecteurs imbriques, avec journalisation differenciee (`println!` vs `eprintln!`).

### operation/
Definit un `enum Operation` et une methode `run()` qui renvoie un `Result`. Quatre operations de base sont couvretes et une erreur descriptive est retournee lors d'une division par zero.

### operator/
Expose un module `utils` partage entre binaire et librairie. `AppError` implante `Display`, et une chaine de fonctions (`get_user_id`, `load_data`, `process_user_data`) montre la propagation d'erreurs avec l'operateur `?`.

### options/
Propose `safe_divide` qui encapsule la validation dans un `Result`. Illustration simple de pattern matching sur `Result` pour afficher soit le quotient soit un message d'erreur.

### overflow/
Demonstration de `wrapping_add` pour gerer explicitement le depassement arithmetique sur un `u8`.

### read-line/
Lit une entree utilisateur depuis `stdin`, supprime le saut de ligne final et renvoie un message de salutation. Exemple classique d'interaction console.

### smart_pointeur/
Utilise `Arc<Mutex<Vec<i32>>>` afin de partager et securiser l'acces a un vecteur entre threads. Trois macros (`sleep_and_lock!`, `sleep_range_and_lock!`, `lock!`) evitent la duplication de code. Un module `list.rs` esquis se trouve en commentaires pour experimenter les pointeurs partages (`Rc`).

### structs_lv_1/
Revisite les structs et impl en definissant `Person`. Fournit des methodes de construction, d'affichage formate et de verification d'age legal via pattern matching.

### structs_lv_2/
Crate plus avance contenant :
- `library::book` et `library::cd` (structs + `Display`).
- `library::media_item` (enum, macro `vec_mediaitem!`, trait `Printable`).
- `library::csv` (lecture/ecriture dans `note.csv`).
- `library::list` (liste chainee recursive avec conversions vers/depuis `Vec`).
- Un binaire qui construit et affiche une liste, ainsi qu'un test d'integration `tests/book.rs`.
Active la feature `#![feature(box_patterns)]`, donc utiliser `cargo +nightly`. Un README detaille supplementaire est accessible dans `structs_lv_2/`.

## Tests
Pour les crates qui fournissent des tests :

```
cd structs_lv_2
cargo +nightly test
```

Vous pouvez etendre la couverture (par exemple ajoutez des tests pour `smart_pointeur` ou `operator`) en creant des modules `tests` et des tests d'integration sous `tests/`.

## Pistes de progression
- Centraliser certains exemples dans un workspace Cargo si vous souhaitez construire un binaire global.
- Ajouter des benches ou des tests de concurrence pour `smart_pointeur`.
- Ecrire des exemples d'utilisation additionnels (`examples/`) pour chaque crate.
- Documenter les crates avec `//!` et publier localement la documentation via `cargo doc --open`.

Bon code !
