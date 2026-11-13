# KJ Apps Desktop — Window Manager

KJ Apps sekarang berperilaku seperti desktop OS. Aplikasi terbuka sebagai jendela (window) yang bisa di-drag, di-resize, di-minimize, di-maximize, dan di-close — semua dalam satu page.

Live: https://kj564.github.io/kj-apps/

## Fitur Window Manager

### Membuka app
- Klik app tile di launchpad → app terbuka sebagai window
- App yang sudah terbuka: klik tile lagi = restore/focus (single instance)

### Window controls (titlebar)
- **Minimize** (kuning): sembunyikan window ke taskbar
- **Maximize** (hijau): toggle fullscreen viewport
- **Close** (merah): tutup window
- **Double-click titlebar**: toggle maximize
- **Drag titlebar**: pindahkan window (mouse + touch)
- **Drag corner kanan-bawah**: resize window

### Taskbar (bottom)
- Toggle launchpad (kiri, ikon grid ungu)
- Open windows list: klik = focus, klik lagi = minimize
- Clock + date (kanan)

### Menubar (top)
- KJ Apps brand: toggle launchpad
- Apps: show launchpad
- Close All: tutup semua window
- Clock (kanan)

### Keyboard shortcuts
- `Cmd/Ctrl + W`: close focused window
- `Cmd/Ctrl + M`: minimize focused window
- `Escape`: toggle launchpad

### Mobile behavior
- Windows otomatis fullscreen (no drag/resize)
- Tap tile → window terbuka full
- Taskbar compact (52px height)

## Apps yang tersedia

| App | URL | Type |
|---|---|---|
| WordCraft | `./wordcraft/` | Same-origin (WASM 59MB) |
| Nobar | `https://kj564.github.io/Nobar/` | Cross-origin (Next.js static) |
| VuCall | `https://kj564.github.io/VuCall/` | Cross-origin (Next.js static) |
| Coming Soon | — | Disabled placeholder |

## Teknis

- **Pure vanilla JS** — no framework, no build step
- **iframe sandbox**: `allow-scripts allow-same-origin allow-downloads allow-popups allow-forms allow-pointer-lock allow-modals allow-presentation`
- **Loading spinner**: hidden on iframe `load` event + 8s fallback (cross-origin load event unreliable)
- **z-index management**: increment counter on focus, max 10000+ for chrome
- **Touch support**: drag + resize bekerja di mobile (tapi windows fullscreen di mobile)
- **CSS variables**: dark theme dengan glass morphism + backdrop-filter

## Browser support

- Chrome/Edge 88+ (backdrop-filter)
- Safari 14+ (webkit-backdrop-filter)
- Firefox 103+ (backdrop-filter)
- Mobile Safari iOS 14+
- Android Chrome

## Roadmap

- [ ] Window snapping (drag to edge → half screen)
- [ ] Persist window state (localStorage)
- [ ] Multi-monitor support (virtual desktops)
- [ ] Right-click context menu on desktop
- [ ] Resize from all 8 edges (not just corner)
