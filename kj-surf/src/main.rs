// KJ Surf — Browser orisinil berbasis Gecko engine
// Arsitektur: Rust + egui + WASM (sama seperti ArtCraft)
//
// Browser shell dengan:
// - Tab bar (multi-tab support)
// - Address bar (URL input + navigation)
// - Back/Forward/Reload buttons
// - Gecko engine rendering ke canvas
// - Bookmarks
// - History
//
// Gecko engine diload via gecko.js (pre-compiled WASM dari HeyPuter/firefox-wasm)
// Browser UI sendiri ditulis murni Rust + egui (bukan iframe, bukan JS)

#![cfg(target_arch = "wasm32")]

use eframe::egui;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use web_sys::{self, HtmlCanvasElement};

// ─── Browser State ──────────────────────────────────────────────
#[derive(Clone)]
struct Tab {
    id: usize,
    title: String,
    url: String,
    loading: bool,
    history: VecDeque<String>,
    history_index: usize,
}

impl Tab {
    fn new(id: usize, url: &str) -> Self {
        let mut history = VecDeque::new();
        history.push_back(url.to_string());
        Self {
            id,
            title: "New Tab".to_string(),
            url: url.to_string(),
            loading: false,
            history,
            history_index: 0,
        }
    }

    fn navigate(&mut self, url: &str) {
        // Normalize URL
        let url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else if url.contains('.') && !url.contains(' ') {
            format!("https://{}", url)
        } else {
            format!("https://duckduckgo.com/?q={}", url.replace(' ', "+"))
        };

        // Truncate history forward
        while self.history.len() > self.history_index + 1 {
            self.history.pop_back();
        }
        self.history.push_back(url.clone());
        self.history_index = self.history.len() - 1;
        self.url = url;
        self.loading = true;
    }

    fn back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.url = self.history[self.history_index].clone();
            self.loading = true;
        }
    }

    fn forward(&mut self) {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.url = self.history[self.history_index].clone();
            self.loading = true;
        }
    }

    fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    fn can_go_forward(&self) -> bool {
        self.history_index < self.history.len() - 1
    }
}

struct KjSurf {
    tabs: Vec<Tab>,
    active_tab: usize,
    next_tab_id: usize,
    address_bar: String,
    bookmarks: Vec<(String, String)>,  // (title, url)
    show_bookmarks: bool,
    gecko_ready: bool,
    gecko_loading: bool,
    status_message: String,
}

impl KjSurf {
    fn new() -> Self {
        let initial_tab = Tab::new(0, "about:home");
        Self {
            tabs: vec![initial_tab],
            active_tab: 0,
            next_tab_id: 1,
            address_bar: String::new(),
            bookmarks: vec![
                ("DuckDuckGo".to_string(), "https://duckduckgo.com".to_string()),
                ("Wikipedia".to_string(), "https://wikipedia.org".to_string()),
                ("GitHub".to_string(), "https://github.com".to_string()),
                ("KJ Apps".to_string(), "https://kj564.github.io/kj-apps/".to_string()),
            ],
            show_bookmarks: false,
            gecko_ready: false,
            gecko_loading: false,
            status_message: "KJ Surf — Browser berbasis Gecko".to_string(),
        }
    }

    fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab)
    }

    fn new_tab(&mut self) {
        let tab = Tab::new(self.next_tab_id, "about:home");
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
        self.next_tab_id += 1;
        self.address_bar.clear();
    }

    fn close_tab(&mut self, index: usize) {
        if self.tabs.len() > 1 {
            self.tabs.remove(index);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }
}

impl eframe::App for KjSurf {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ─── Top: Tab bar ───────────────────────────────
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Tabs
                let mut to_close: Option<usize> = None;
                for (i, tab) in self.tabs.iter().enumerate() {
                    let is_active = i == self.active_tab;
                    let title = if tab.loading {
                        format!("⟳ {}...", tab.title)
                    } else {
                        tab.title.clone()
                    };

                    let tab_resp = ui.selectable_label(is_active, &title);
                    if tab_resp.clicked() {
                        self.active_tab = i;
                        self.address_bar = self.tabs[i].url.clone();
                    }

                    // Close button
                    if self.tabs.len() > 1 {
                        if ui.small_button("✕").clicked() {
                            to_close = Some(i);
                        }
                    }
                    ui.separator();
                }

                // New tab button
                if ui.button("+").clicked() {
                    self.new_tab();
                }
            });

            if let Some(idx) = to_close {
                self.close_tab(idx);
            }
        });

        // ─── Navigation bar ──────────────────────────────
        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Back button
                let can_back = self.tabs.get(self.active_tab).map(|t| t.can_go_back()).unwrap_or(false);
                if ui.add_enabled(can_back, egui::Button::new("←")).clicked() {
                    if let Some(tab) = self.active_tab_mut() {
                        tab.back();
                        self.address_bar = tab.url.clone();
                    }
                }

                // Forward button
                let can_fwd = self.tabs.get(self.active_tab).map(|t| t.can_go_forward()).unwrap_or(false);
                if ui.add_enabled(can_fwd, egui::Button::new("→")).clicked() {
                    if let Some(tab) = self.active_tab_mut() {
                        tab.forward();
                        self.address_bar = tab.url.clone();
                    }
                }

                // Reload button
                if ui.button("⟳").clicked() {
                    if let Some(tab) = self.active_tab_mut() {
                        tab.loading = true;
                        self.status_message = format!("Reloading {}...", tab.url);
                    }
                }

                // Home button
                if ui.button("🏠").clicked() {
                    if let Some(tab) = self.active_tab_mut() {
                        tab.navigate("about:home");
                        self.address_bar = String::new();
                    }
                }

                // Address bar
                let resp = ui.add_sized(
                    [ui.available_width() - 100.0, 20.0],
                    egui::TextEdit::singleline(&mut self.address_bar)
                        .hint_text("Search or enter URL…")
                        .interactive(true),
                );

                if resp.lost_focus() && ui.input(|r| r.key_pressed(egui::Key::Enter)) {
                    if !self.address_bar.is_empty() {
                        if let Some(tab) = self.active_tab_mut() {
                            tab.navigate(&self.address_bar.clone());
                            self.status_message = format!("Loading {}...", tab.url);
                        }
                    }
                }

                // Bookmarks button
                if ui.button("★").clicked() {
                    self.show_bookmarks = !self.show_bookmarks;
                }

                // Bookmark current page
                if ui.button("⭐").clicked() {
                    if let Some(tab) = self.tabs.get(self.active_tab) {
                        self.bookmarks.push((tab.title.clone(), tab.url.clone()));
                    }
                }
            });
        });

        // ─── Bookmarks dropdown ──────────────────────────
        if self.show_bookmarks {
            egui::TopBottomPanel::top("bookmarks").show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for (title, url) in &self.bookmarks {
                        if ui.button(title).clicked() {
                            if let Some(tab) = self.active_tab_mut() {
                                tab.navigate(url);
                                self.address_bar = url.clone();
                            }
                            self.show_bookmarks = false;
                        }
                    }
                });
            });
        }

        // ─── Status bar ──────────────────────────────────
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status = if let Some(tab) = self.tabs.get(self.active_tab) {
                    if tab.loading {
                        format!("⟳ Loading: {}", tab.url)
                    } else {
                        format!("✓ {}", tab.url)
                    }
                } else {
                    self.status_message.clone()
                };
                ui.label(&status);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Gecko Engine: {}", if self.gecko_ready { "✓ Ready" } else { "⟳ Loading" }));
                    ui.label(format!("Tabs: {}", self.tabs.len()));
                    ui.label("KJ Surf v0.1.0");
                });
            });
        });

        // ─── Main content area (Gecko canvas) ───────────
        egui::CentralPanel::default().show(ctx, |ui| {
            // Gecko engine renders here
            // The canvas is managed by eframe/egui
            // Gecko paints to canvas via JS bridge

            let active_url = self.tabs.get(self.active_tab).map(|t| t.url.clone()).unwrap_or_default();

            if active_url == "about:home" {
                // Home page
                ui.vertical_centered(|ui| {
                    ui.add_space(80.0);
                    ui.heading("🌐 KJ Surf");
                    ui.label("Browser orisinil berbasis Gecko engine");
                    ui.add_space(20.0);

                    // Search box
                    let mut search = String::new();
                    let resp = ui.add_sized(
                        [400.0, 30.0],
                        egui::TextEdit::singleline(&mut search)
                            .hint_text("Search the web…"),
                    );
                    if resp.lost_focus() && ui.input(|r| r.key_pressed(egui::Key::Enter)) {
                        if !search.is_empty() {
                            if let Some(tab) = self.active_tab_mut() {
                                let url = format!("https://duckduckgo.com/?q={}", search.replace(' ', "+"));
                                tab.navigate(&url);
                                self.address_bar = url;
                            }
                        }
                    }

                    ui.add_space(10.0);
                    ui.label("Quick links:");
                    ui.horizontal(|ui| {
                        for (title, url) in &self.bookmarks {
                            if ui.button(title).clicked() {
                                if let Some(tab) = self.active_tab_mut() {
                                    tab.navigate(url);
                                    self.address_bar = url.clone();
                                }
                            }
                        }
                    });

                    ui.add_space(40.0);
                    ui.label(
                        egui::RichText::new("Powered by Gecko engine (Firefox WASM) · Built with Rust + egui")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                });
            } else {
                // Show Gecko canvas area
                // Gecko engine renders to canvas via JS bridge
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() / 3.0);
                    if self.gecko_ready {
                        ui.label("🦎 Gecko engine active");
                        ui.label(format!("URL: {}", active_url));
                    } else if self.gecko_loading {
                        ui.spinner();
                        ui.label("Loading Gecko engine (34MB WASM)…");
                    } else {
                        ui.label("⚠ Gecko engine not loaded");
                        ui.label(format!("Would load: {}", active_url));
                        if ui.button("Initialize Gecko").clicked() {
                            self.gecko_loading = true;
                            self.status_message = "Loading Gecko WASM…".to_string();
                            // Gecko init would happen here via JS bridge
                        }
                    }
                });
            }

            // Simulate loading completion
            if let Some(tab) = self.active_tab_mut() {
                if tab.loading {
                    tab.loading = false;
                    // Update title from URL
                    let url = tab.url.clone();
                    if url.contains("duckduckgo.com") {
                        tab.title = "DuckDuckGo".to_string();
                    } else if url.contains("wikipedia.org") {
                        tab.title = "Wikipedia".to_string();
                    } else if url.contains("github.com") {
                        tab.title = "GitHub".to_string();
                    } else if url.starts_with("about:home") {
                        tab.title = "New Tab".to_string();
                    } else {
                        // Extract domain
                        if let Some(start) = url.find("://") {
                            let domain = &url[start + 3..];
                            let end = domain.find('/').unwrap_or(domain.len());
                            tab.title = domain[..end].to_string();
                        } else {
                            tab.title = url.clone();
                        }
                    }
                }
            }
        });

        // Request repaint for loading animations
        if self.tabs.iter().any(|t| t.loading) {
            ctx.request_repaint();
        }
    }
}

// ─── WASM entry point ──────────────────────────────────────────
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let canvas = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("kj_surf_canvas"))
        .and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok())
        .expect("canvas #kj_surf_canvas not found");

    // Hide loading screen
    if let Some(window) = web_sys::window() {
        if let Some(doc) = window.document() {
            if let Some(loading) = doc.get_element_by_id("kj_surf_loading") {
                loading.set_attribute("style", "display:none").ok();
            }
        }
    }

    let viewport = eframe::WebViewport::Fixed(canvas);
    eframe::WebRunner::new()
        .start(
            "kj_surf_canvas",
            eframe::WebOptions {
                viewport,
                ..Default::default()
            },
            Box::new(|cc| Ok(Box::new(KjSurf::new()))),
        )
        .expect("failed to start eframe");
}

// Console panic hook
mod console_error_panic_hook {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "export function hook() { console.error_panic_hook = true; }")]
    extern "C" {
        pub fn hook();
    }

    pub fn set_once() {
        // Simple panic hook for WASM
        std::panic::set_hook(Box::new(|info| {
            web_sys::console::error_1(&format!("KJ Surf panic: {}", info).into());
        }));
    }
}
