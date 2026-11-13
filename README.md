# KJ Apps — Web Desktop OS

KJ Apps adalah **web desktop operating system** berbasis **OS.js v3.1.12** (kernel aplikasi). Berjalan di browser via GitHub Pages static hosting.

Live: https://kj564.github.io/kj-apps/

## Arsitektur

```
┌─────────────────────────────────────────────────┐
│ KJ Apps (OS shell)                              │
│ - Desktop, panel, taskbar, window manager       │
│ - App launcher, system tray                      │
├─────────────────────────────────────────────────┤
│ OS.js v3.1.12 (Application Kernel)              │
│ - VFS abstraction (Virtual File System)          │
│ - Process manager (app lifecycle)                │
│ - Auth service (auto-login demo/demo)            │
│ - GUI toolkit (buttons, lists, dialogs)          │
│ - Package system (apps as modules)               │
├─────────────────────────────────────────────────┤
│ Browser Engine (V8 + Web APIs)                  │
│ - File: OPFS, IndexedDB (future VFS backend)     │
│ - Network: fetch, WebSocket                       │
│ - Display: Canvas, WebGL, DOM                     │
└─────────────────────────────────────────────────┘
```

## Built-in Apps (OS.js packages)

| App | Category | Description |
|---|---|---|
| Calculator | office | Calculator |
| Draw | graphics | Drawing tool |
| FileManager | utilities | File manager |
| HTMLViewer | office | HTML viewer |
| MusicPlayer | multimedia | Music player |
| Preview | graphics | Image/video preview |
| Settings | utilities | System settings |
| Textpad | utilities | Text editor |

## External Apps (existing, to be integrated)

- **WordCraft** — Rust+WASM word processor (`/wordcraft/`)
- **Nobar** — Streaming film indie (`https://kj564.github.io/Nobar/`)
- **VuCall** — Video call 1:1 (`https://kj564.github.io/VuCall/`)

## Login

Default credentials (static deployment, auto-fill):
- Username: `demo`
- Password: `demo`

## Limitations (browser sandbox)

- VFS calls fail gracefully (no server-side storage) — future: OPFS backend
- Server-dependent features (MusicPlayer streaming) won't work
- Client-side apps (Calculator, Draw, Textpad, Settings) work fine

## Tech Stack

- **Kernel**: OS.js v3.1.12 (BSD-2-Clause license)
- **Build**: webpack 4 + babel
- **Fonts**: Roboto (Google Fonts, bundled)
- **Theme**: Standard Theme
- **Icons**: Gnome Icons
- **Hosting**: GitHub Pages (static, .nojekyll for raw file serving)

## Source

- OS.js source: https://github.com/os-js/OS.js
- KJ Apps repo: https://github.com/kj564/kj-apps

## Roadmap

- [ ] Integrate WordCraft/Nobar/VuCall as OS.js iframe apps
- [ ] Custom ChromiumOS-style theme
- [ ] VFS backend with OPFS (real file system in browser)
- [ ] Service Worker for offline mode
- [ ] Custom package manager (`kjpm install`)
