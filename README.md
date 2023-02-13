# Telegram Space
## Dependencies
 - godot 4.0 rc 1
   - https://godotengine.org/article/release-candidate-godot-4-0-rc-1/
   - https://downloads.tuxfamily.org/godotengine/4.0/rc1
 - rust 1.66.1 (90743e729 2023-01-10)
   - https://rustup.rs/
 - sfz 0.7.1 (for serving build artifacts)
   - https://crates.io/crates/sfz
   - after installing rust, run `cargo install sfz`
   - You can also do brew install on macOS: https://github.com/weihanglo/sfz#macos
   - or just download prebuilt binary from gh: https://github.com/weihanglo/sfz#prebuilt-binaries

---

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

---

## Non-Goal

**Robustness:**

This is a prototype for MVP, so for the sake of similicity, we won't waste our
time on making things robust. So for now, the networking/protocol implementation
only contains happy path, and we just simply trust clients to never lie, and
server to never fail.

**Efficiency:**

So for now we won't worry about things like:
 - Websocket being by default only supports TCP and UDP being more suitable for realtime usage
 - WebRTC is UDP based but design to be used in P2P architecture so requires a hack to use it for server-client architecture

**Visual Design:**

Game engins are built to support dynamically loading game assets, updating,
remastering etc, meaning we should not worry about ironing out all the
nitty-gritty details for the graphics for now. We can do that later when we
transition to production mode.

**Extra Miles:**

There are industry standard optimization techniques--e.g. coordinate
interpolation, extrapolation, etc--which requires a bit more complex networking
architecture. We won't touch this at this time.

## Goal

**System Validation:**
 - wasm + webgl client using any of the common transport or application layer protocols
   - is it fast enough?
   - is it reliable?
   - can it be developed with reasonable amount of pain?
 - is Godot 4 wasm target ready for production?

**UX Validation**:
 - general performance (feel, smoothness)
 - mode of interaction (touch input on mobile)
 - integration:
   - can a user enter the client smoothly from telegram?
   - can it be loaded on different mobile platforms and web browsers?
   - can wasm + webgl render reliably on in-app view?

