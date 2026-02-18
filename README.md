# Recipes — Projektübersicht

Kleines Mono-Repo mit zwei Rust-Projekten:

- `backend` — Rocket-basiertes Backend (Postgres)
- `frontend` — Yew-basiertes Web-Frontend (WASM, empfohlen: `trunk`)

## Voraussetzungen

- Rust + Cargo installiert (rustup)
- PostgreSQL erreichbar (Standard: `postgres://postgres:postgres@localhost:5432/recipes`)
- `trunk` installiert für das Frontend (optional):

```bash
cargo install trunk
```

## Build & Run (empfohlen)

```bash
# baut beide Crates
cargo build --workspace

# startet nur das Backend
cargo run -p backend

# für Entwicklung des Frontends
cd frontend
trunk serve
```

Hinweis: `cargo build` kompiliert auch das Frontend-Crate, für die WebAssembly-Bündelung und das Dev-Server-Verhalten solltest du `trunk` verwenden.

## Schnelltest (Backend)

Ein Beispiel, um einen Recipe-POST zu testen (Backend muss laufen):

```bash
curl -X POST http://127.0.0.1:8000/api/recipes \
  -H 'Content-Type: application/json' \
  -d '{"title":"Test","short_description":"x","ingredients":[],"steps":[],"is_public":true}'
```

## Konfiguration

- Datenbank-URL: `backend/.env` (setzt `DATABASE_URL` für lokale Tests)
- In `backend/src/main.rs` sind die CORS-Optionen gesetzt; passe bei Bedarf die erlaubten Origins/Headers an.

## Datenbankmigrationen

Dieses Repo enthält Diesel-/SQL-Style-Migrationen im Verzeichnis `backend/migrations`.

Zwei einfache Wege, die Migrationen anzuwenden:

- Mit `diesel_cli` (empfohlen, wenn installiert):

```bash
# Installieren (benötigt libpq / PostgreSQL dev libs)
cargo install diesel_cli --no-default-features --features postgres

# Environment setzen (oder in backend/.env setzen)
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/recipes

# Migrationen ausführen
cd backend
diesel migration run
```

- Alternativ direkt mit `psql` die SQL-Dateien anwenden (kein diesel nötig):

```bash
# Beispiel: apply the up.sql for the migration
psql $DATABASE_URL -f backend/migrations/20260217120000_create_recipes/up.sql
```

Falls du eine lokale Postgres-Instanz per Homebrew verwendest, kannst du die DB vorher anlegen:

```bash
createdb -h localhost -U postgres recipes
```

Die Migrationen erzeugen die Tabellen `users`, `categories`, `recipes`, `recipe_categories`, `images`, `recipe_versions`.
