<div align="center">
    <h1>🎥 Tape</h1>
    <p>tape is an application for recording mouse and/or keyboard actions,<br/>which can generate a recording file for later playback.</p>
    <div align="center">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="">
        <img src="https://img.shields.io/github/last-commit/lopo12123/tape" alt="">
    </div>
</div>

---

## Architecture

This project uses the workspace structure, the following are its members

- [`tape-core`](./crates/tape-core) Core library -- ✅ **Completed**
    - Rust implementation of recording and simulation of mouse/keyboard actions
    - Canonical mouse/keyboard key and event declarations
- [`tape-node`](./crates/tape-node) Node.js binding of the core library -- ⏳ **WIP**
    - Node.js binding of the core library (using [napi-rs](https://github.com/napi-rs/napi-rs))
- [`tape-egui`](./crates/tape-egui) Egui implementation of the application -- 🚫 **Suspended**
    - GUI implementation of the application (using [egui](https://github.com/emilk/egui))
- [`tape-tauri`](./crates/tape-tauri) Tauri implementation of the application -- ⏳ **WIP**
    - GUI implementation of the application (using [tauri](https://tauri.app/))

## License

See [LICENSE](./LICENSE) for details.