# tui-async-template

*(English version below / versione inglese più sotto)*

## Italiano

Template `cargo-generate` per applicazioni TUI in Rust basate su [ratatui](https://ratatui.rs) e [tokio](https://tokio.rs), con un pattern architetturale Elm-like (`Message` → `update` → `render`) e un esempio funzionante di integrazione con I/O asincrono in background.

### Cosa include

- **Canale di messaggi unificato**: tastiera, tick periodico e task in background (es. richieste HTTP) comunicano tutti tramite lo stesso canale `mpsc`, convergendo in un unico punto di aggiornamento dello stato.
- **Separazione controllo/applicazione**: i messaggi di controllo (`Quit`, `InputError`) sono intercettati direttamente nel loop principale; tutti gli altri passano per `update::update`.
- **Gestione degli errori** con `color_eyre`.
- **Log a video** con [`tui-logger`](https://docs.rs/tui-logger): premi `l` per mostrare/nascondere il pannello di log in fondo allo schermo.
- **Gestione del resize** del terminale (forza un redraw riusando il messaggio di tick).
- **Esempio di task in background** (`ApiClient`, in `src/api.rs`): due chiamate HTTP di esempio (verso [jsonplaceholder](https://jsonplaceholder.typicode.com)) che mostrano il pattern "fetch → invia risultato come `Message`". È pensato per essere sostituito con qualsiasi altro lavoro asincrono (un'altra API, un file watcher, un websocket...) mantenendo lo stesso schema.

Nel codice, i commenti distinguono tra:
- documentazione (`///`) sulle parti di **pattern/infrastruttura**, pensate per sopravvivere quando personalizzi il progetto;
- commenti semplici (`//` con prefisso `Example:`) sulle parti di **dominio** (utenti, to-do, URL...), pensate per essere cancellate o riscritte.

### Come personalizzare il progetto

| Voglio... | Dove guardare |
|---|---|
| Aggiungere un nuovo tasto | `src/event.rs`, funzione `key_to_message`: aggiungi un branch che mappa il `KeyCode` al `Message` desiderato. Se il messaggio è nuovo, va prima definito in `src/update.rs`. |
| Definire un nuovo messaggio | `src/update.rs`, enum `Message`: aggiungi il variant nel blocco "control" o "application" a seconda che debba essere gestito dal loop principale (`src/main.rs`) o da `update::update`. |
| Sapere dove vive lo stato dell'applicazione | `src/state.rs`, struct `AppState`. Le mutazioni passano sempre da un metodo `pub` chiamato da `update::update`, mai direttamente dall'esterno. |
| Cambiare cosa viene disegnato a schermo | `src/ui.rs`, funzione `render` (e `render_ui`/`render_list`/`render_table` per il contenuto d'esempio, da sostituire). |
| Sostituire il fetch HTTP di esempio con un'altra fonte async (altra API, file watcher, websocket...) | `src/api.rs`, struct `ApiClient`: mantieni il pattern "spawna un task, invia il risultato come `Message` sul canale `tx`", cambia solo cosa fa il task. |
| Cambiare la frequenza del tick | `src/main.rs`, `Duration::from_millis(100)` nella creazione dell'`Interval`. |
| Cambiare stile/colori del pannello di log | `src/ui.rs`, funzione `render_log`. |
| Aggiungere una nuova sorgente di eventi in background (oltre tastiera/tick/HTTP) | Segui lo schema di `api.rs` o `event.rs`: uno `spawn` (task tokio o thread OS se bloccante) che invia `Message` su un clone di `tx`; nessuna modifica al loop principale se il messaggio è già gestito da `update::update`. |

### Come generare un nuovo progetto

```bash
cargo generate --git https://github.com/marc0x71/tui_async --name mio-progetto
```

oppure, per testare in locale senza passare da un repository remoto:

```bash
cargo generate --path . --name mio-progetto --destination /tmp
```

### Sviluppo del template

Il repository sorgente resta un progetto Cargo normale e compilabile: `Cargo.toml` contiene un nome di pacchetto reale (`tui-async-template`), non il placeholder `{{project-name}}`. Questo permette di lavorare, compilare ed eseguire il progetto direttamente durante lo sviluppo del template, senza doverlo prima "espandere".

Il placeholder va inserito **solo al momento del rilascio**, con lo script:

```bash
#!/usr/bin/env bash
# scripts/prepare-release.sh
sed -i 's/^name = ".*"/name = "{{project-name}}"/' Cargo.toml
git diff Cargo.toml
```

Da lanciare a mano prima di un commit/tag di rilascio, controllando sempre il diff prima di confermare.

Per verificare che il template si espanda correttamente (anche dopo aver lanciato lo script sopra):

```bash
cargo generate --path . --name dev-test --destination /tmp --silent
cd /tmp/dev-test && cargo check
```

### Roadmap / cose rimandate volutamente

- Configurazione esterna (CLI/file): non inclusa di proposito — resta libertà di chi genera il progetto decidere se e come renderlo configurabile.
- Test di esempio: non inclusi, dato che la logica di stato di esempio (`AppState`) è contenuto di dominio, non pattern riusabile.

---

## English

`cargo-generate` template for Rust TUI applications built with [ratatui](https://ratatui.rs) and [tokio](https://tokio.rs), following an Elm-like architecture (`Message` → `update` → `render`) with a working example of background async I/O integration.

### What's included

- **Unified message channel**: keyboard input, the periodic tick, and background tasks (e.g. HTTP requests) all communicate through the same `mpsc` channel, converging into a single state-update point.
- **Control vs. application messages**: control messages (`Quit`, `InputError`) are intercepted directly in the main loop; everything else flows through `update::update`.
- **Error handling** with `color_eyre`.
- **On-screen logging** via [`tui-logger`](https://docs.rs/tui-logger): press `l` to toggle a log panel at the bottom of the screen.
- **Terminal resize handling** (forces a redraw by reusing the tick message).
- **Background task example** (`ApiClient`, in `src/api.rs`): two example HTTP calls (against [jsonplaceholder](https://jsonplaceholder.typicode.com)) demonstrating the "fetch → send result as a `Message`" pattern. Meant to be replaced with any other async work (a different API, a file watcher, a websocket...) while keeping the same shape.

In the source, comments distinguish between:
- doc comments (`///`) on **pattern/infrastructure** code, meant to survive as you customize the project;
- plain comments (`//`, prefixed with `Example:`) on **domain-specific** code (users, todos, URLs...), meant to be deleted or rewritten.

### Customizing the project

| I want to... | Where to look |
|---|---|
| Add a new key binding | `src/event.rs`, `key_to_message` function: add a branch mapping the `KeyCode` to the desired `Message`. If the message is new, define it in `src/update.rs` first. |
| Define a new message | `src/update.rs`, `Message` enum: add the variant to the "control" or "application" block, depending on whether it should be handled by the main loop (`src/main.rs`) or by `update::update`. |
| Find where the application state lives | `src/state.rs`, `AppState` struct. Mutations always go through a `pub` method called from `update::update`, never directly from the outside. |
| Change what's drawn on screen | `src/ui.rs`, `render` function (and `render_ui`/`render_list`/`render_table` for the example content, meant to be replaced). |
| Replace the example HTTP fetch with another async source (a different API, a file watcher, a websocket...) | `src/api.rs`, `ApiClient` struct: keep the "spawn a task, send the result as a `Message` on the `tx` channel" pattern, only change what the task does. |
| Change the tick frequency | `src/main.rs`, `Duration::from_millis(100)` where the `Interval` is created. |
| Change the log panel's style/colors | `src/ui.rs`, `render_log` function. |
| Add a new background event source (beyond keyboard/tick/HTTP) | Follow the shape of `api.rs` or `event.rs`: a `spawn` (a tokio task, or an OS thread if the work is blocking) that sends `Message`s over a clone of `tx`; no changes to the main loop needed if the message is already handled by `update::update`. |

### Generating a new project

```bash
cargo generate --git https://github.com/marc0x71/tui_async --name my-project
```

or, to test locally without a remote repository:

```bash
cargo generate --path . --name my-project --destination /tmp
```

### Developing the template

The source repository is kept as a normal, buildable Cargo project: `Cargo.toml` holds a real package name (`tui-async-template`), not the `{{project-name}}` placeholder. This lets you build and run the project directly while working on the template, without expanding it first.

The placeholder is inserted **only at release time**, via:

```bash
#!/usr/bin/env bash
# scripts/prepare-release.sh
sed -i 's/^name = ".*"/name = "{{project-name}}"/' Cargo.toml
git diff Cargo.toml
```

Run it by hand before a release commit/tag, always reviewing the diff before confirming.

To verify the template still expands correctly (including after running the script above):

```bash
cargo generate --path . --name dev-test --destination /tmp --silent
cd /tmp/dev-test && cargo check
```

### Roadmap / intentionally deferred

- External configuration (CLI/file): deliberately not included — left to whoever generates the project to decide whether and how to make it configurable.
- Example tests: not included, since the example state logic (`AppState`) is domain content, not a reusable pattern.

---

*Questo README è stato generato con l'assistenza di un'intelligenza artificiale (Claude, Anthropic), a partire dalle decisioni architetturali prese durante lo sviluppo del progetto.*

*This README was generated with the assistance of an AI (Claude, Anthropic), based on the architectural decisions made during the project's development.*
