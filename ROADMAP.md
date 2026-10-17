# KiPaint Roadmap

Goal: a Clip Studio Paint-class painting workflow on the Graphite engine — native desktop first, original UI with a CSP-familiar layout (tools left, sub-tool options, layers right, timeline bottom).

## Phase 0 — Base (done)

- [x] Import Graphite codebase (editor core, node graph, desktop shell, frontend)
- [x] Project docs: bilingual README, roadmap, repository branding

## Phase 1 — Identity & Layout

- [ ] Rebrand desktop app: name, app id, icons, window title, about dialog
- [ ] CSP-style default panel layout: toolbar left, tool options, layers + inspector right, status bar
- [ ] Dark painting-first theme and original icon set
- [ ] Keymap audit: match CSP defaults (B brush, E eraser, G fill, L lasso…), user-configurable

## Phase 2 — Brush Engine MVP

- [ ] Line stabilization (post-filter and predictive smoothing, per-brush strength)
- [ ] Pressure curve editor + velocity-based dynamics
- [ ] Per-brush opacity (separate from flow), brush textures, eraser mode
- [ ] Extend brush node params: tilt and twist influence, stroke seed variation
- [ ] Brush presets: save/load, import pack format

## Phase 3 — Painting UX

- [ ] Flood fill and magic wand with gap closing, refer-other-layers, multi-area fill
- [ ] Color wheel panel, color mixer, color sets and color history
- [ ] Navigator panel, canvas rotate/flip shortcuts polish
- [ ] Multi-touch gestures (pinch zoom / two-finger pan)
- [ ] Selection suite: lasso, polygon, quick mask, selection layers

## Phase 4 — CSP Parity Features

- [ ] Vector layers with line correction: pinch, adjust width, redraw, connect, vector eraser
- [ ] Mesh transform, liquify, puppet warp
- [ ] Rulers: linear, curve, figure, symmetry; perspective ruler with vanishing points
- [ ] Animation: frame timeline, cels, onion skin, audio track, video export
- [ ] Tonal adjustment layers

## Phase 5 — Comics & Manga

- [ ] Frame borders and panel divide tools
- [ ] Speech balloons, screentone layers and materials
- [ ] Multi-page project management, page template presets

## Ongoing

- [ ] PSD import/export
- [ ] CMYK export and print color profiles
- [ ] Auto-actions (macro recorder)
- [ ] Brush/material asset format for community sharing

Details and current state of the inherited Graphite base are described in the repository README.
