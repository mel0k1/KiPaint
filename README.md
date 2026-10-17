<div align="center">

# KiPaint

**Free & open-source digital painting app for illustration, comics and manga.**
**Built in Rust. GPU-accelerated. A Clip Studio Paint-style workflow, done right.**

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-1.98%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)]()
[![Build](https://github.com/mel0k1/KiPaint/actions/workflows/build.yml/badge.svg)](https://github.com/mel0k1/KiPaint/actions/workflows/build.yml)

*Русская версия: [README.ru.md](README.ru.md)*

</div>

---

## What is KiPaint

KiPaint is a free and open-source painting application aimed at one thing: **the drawing feel of Clip Studio Paint, without the paywall or the closed source.** It is built on top of the [Graphite](https://github.com/GraphiteEditor/Graphite) engine (Apache-2.0), keeping its node-based core, GPU renderer and CRDT document model, while replacing the designer-oriented interface and toolset with a workflow tuned for illustrators, comic and manga artists.

The focus is not on reinventing a graphics engine — it is on delivering the daily-driver features that painters actually need: a natural brush engine with pressure dynamics and line stabilization, raster layers with clipping masks, flood fill that just works, vector line editing, an animation timeline, and comic panels — in a native desktop app that feels like a real paint tool, not a node editor with tools bolted on.

> *CLIP STUDIO PAINT is a trademark of Celsys, Inc. KiPaint is an independent open-source project, not affiliated with or endorsed by Celsys. All interface design and assets are original.*

## Status

The project is in early development. The current state is the imported Graphite base (editor core, node graph, desktop shell, web frontend) with KiPaint documentation. Functional KiPaint-specific work starts now — see the [roadmap](ROADMAP.md).

**What already works (inherited from the Graphite base):**

| Area | State |
|---|---|
| Native desktop shell | winit + wgpu window, Windows / macOS / Linux |
| Tablet input | Pressure, tilt, twist, eraser tip via a patched winit fork |
| Brush engine | GPU brush (supergaussian kernel): diameter, hardness, flow, pressure → size |
| Layers | Node-graph layers, folders, 27+ blend modes, clipping masks, opacity |
| Vector tools | Pen, path, freehand, spline, shapes, boolean path ops |
| Raster editing | Levels, curves, hue/saturation, blur, LUT color lookup (`.cube`, `.3dl`, `.csp`…) |
| Text | Font catalog, on-canvas editing, convert-to-path |
| Document format | CRDT-based `.gdd` container — undo, migrations and merging from one storage |
| UI framework | Svelte frontend, dockable panel system, full localization infrastructure |

**What KiPaint adds (planned):** line stabilization and pressure curves, per-brush opacity and textures, flood fill & auto select with gap closing, CSP-style default layout, brush palette with sub-tools, color wheel and mixer, navigator, animation timeline with onion skin, vector line correction tools, rulers and perspective, comic frames and speech balloons, screentones, PSD support. Details in [ROADMAP.md](ROADMAP.md).

## Building

Requirements: Rust 1.98+, Node.js + npm, `wasm32-unknown-unknown` target, pinned `wasm-bindgen-cli`, `wasm-opt`, `cargo-about` (the `cargo run` bootstrapper below checks and installs most of this automatically; CEF builds additionally need CMake + Ninja on Windows/macOS).

```bash
# first build (downloads branding assets, builds wasm, bundles the desktop app)
cargo run --package cargo-run -- desktop

# or via the shorthand helper
cargo run -- desktop
```

Build artifacts and bundles are produced by `graphite-desktop-bundle`; on first run the frontend wasm module is compiled to `frontend/wrapper/pkg-native/` and the Svelte frontend to `frontend/dist/`.

Web (WASM) build:

```bash
cargo run -- web
```

## Project structure

| Directory | Contents |
|---|---|
| `desktop/` | Native desktop shell: window, compositor, input, bundling, platform code (Linux / macOS / Windows) |
| `editor/` | Rust editor core: message system, tools, document and portfolio state |
| `node-graph/` | Node engine: graph compiler, executor, wgpu GPU runtime, brush, raster, vector, blending nodes |
| `document/` | Document format and CRDT storage (`.gdd` container) |
| `frontend/` | Svelte/TypeScript UI (runs in CEF on desktop, browser on web) |
| `tools/` | Dev tooling: `cargo-run` bootstrapper, CI drivers, docs generators |

## Credits

KiPaint is built on the excellent work of the [Graphite](https://graphite.art) project and its contributors — the editor core, node engine, renderer and desktop shell all originate from the Graphite codebase. Without it, KiPaint would not exist. Go star [their repository](https://github.com/GraphiteEditor/Graphite) and consider [supporting the upstream project](https://graphite.art/donate/).

## License

Apache License 2.0 — see [LICENSE-APACHE](LICENSE-APACHE). The same license as the upstream Graphite project.
