# Telegram Space
## Dependencies
 - godot 4.0 beta 13
   - https://godotengine.org/article/dev-snapshot-godot-4-0-beta-13/
   - https://downloads.tuxfamily.org/godotengine/4.0/beta13/
 - rust 1.66.1 (90743e729 2023-01-10)
   - https://rustup.rs/
 - sfz 0.7.1 (for serving build artifacts)
   - https://crates.io/crates/sfz
   - after installing rust, run `cargo install sfz`

## How to Build
### Server
 - run `cargo build` in `server/`

### Client
 - option 1: export the project using godot 4.0 beta 13 editor
 - option 2:
   - edit the godot binary location in `client/build.sh`
   - run the script
 - html5 build artifacts should be exported to `client/exports`

## How to Run
### Server
 - run `cargo run` in `server/`

### Client
 - serve the exports by running `sfz --coi --render-index --port 5000` in `client/exports`
 - go to `http://localhost:5000`
