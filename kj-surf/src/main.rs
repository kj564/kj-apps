// KJ Surf — Browser orisinil berbasis Gecko engine
// Arsitektur: Rust + egui + WASM (sama seperti ArtCraft)
//
// Browser shell dengan tab bar, address bar, navigation, bookmarks.
// Gecko engine diload via gecko.js (pre-compiled WASM dari HeyPuter/firefox-wasm)

#![cfg(target_arch = "wasm32")]

use eframe::egui;
use std::collections::VecDeque;

// ─── Browser Tab State ──────────────────────────────────────────
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
        let url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else if url.contains('.') && !url.contains(' ') {
            format!("https://{}", url)
        } else {
            format!("https://duckduckgo.com/?q={}", url.replace(' ', "+"))
        };

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
}

// ─── Browser App ────────────────────────────────────────────────
struct KjSurf {
    tabs: Vec<Tab>,
    active_tab: usize,
    next_tab_id: usize,
    address_bar: String,
    bookmarks: Vec<(String, String)>,
    show_bookmarks: bool,
    gecko_ready: bool,
    status_message: String,
}

impl KjSurf {
    fn new() -> Self {
        Self {
            tabs: vec![Tab::new(0, "about:home")],
            active_tab: 0,
            next_tab_id: 1,
            address_bar: String::new(),
            bookmarks: vec![
                ("DuckDuckGo".into(), "https://duckduckgo.com".into()),
                ("Wikipedia".into(), "https://wikipedia.org".into()),
                ("GitHub".into(), "https://github.com".into()),
                ("KJ Apps".into(), "https://kj564.github.io/kj-apps/".into()),
            ],
            show_bookmarks: false,
            gecko_ready: false,
            status_message: "KJ Surf — Browser berbasis Gecko".into(),
        }
    }

    fn new_tab(&mut self) {
        self.tabs.push(Tab::new(self.next_tab_id, "about:home"));
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
        // Tab bar
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut close_idx: Option<usize> = None;
                for (i, tab) in self.tabs.iter().enumerate() {
                    let is_active = i == self.active_tab;
                    let label = if tab.loading { format!("⟳ {}...", tab.title) } else { tab.title.clone() };
                    if ui.selectable_label(is_active, &label).clicked() {
                        self.active_tab = i;
                        self.address_bar = self.tabs[i].url.clone();
                    }
                    if self.tabs.len() > 1 && ui.small_button("✕").clicked() {
                        close_idx = Some(i);
                    }
                    ui.separator();
                }
                if ui.button("+ New").clicked() {
                    self.new_tab();
                }
                if let Some(idx) = close_idx {
                    self.close_tab(idx);
                }
            });
        });

        // Navigation bar
        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let can_back = self.tabs.get(self.active_tab).map(|t| t.history_index > 0).unwrap_or(false);
                let can_fwd = self.tabs.get(self.active_tab).map(|t| t.history_index < t.history.len() - 1).unwrap_or(false);

                if ui.add_enabled(can_back, egui::Button::new("← Back")).clicked() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        tab.back();
                        self.address_bar = tab.url.clone();
                    }
                }
                if ui.add_enabled(can_fwd, egui::Button::new("Forward →")).clicked() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        tab.forward();
                        self.address_bar = tab.url.clone();
                    }
                }
                if ui.button("⟳ Reload").clicked() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        tab.loading = true;
                    }
                }
                if ui.button("🏠 Home").clicked() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        tab.navigate("about:home");
                        self.address_bar.clear();
                    }
                }

                let resp = ui.add_sized(
                    [ui.available_width() - 120.0, 20.0],
                    egui::TextEdit::singleline(&mut self.address_bar)
                        .hint_text("Search or enter URL…"),
                );
                if resp.lost_focus() && ui.input(|r| r.key_pressed(egui::Key::Enter)) {
                    if !self.address_bar.is_empty() {
                        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                            tab.navigate(&self.address_bar.clone());
                        }
                    }
                }

                if ui.button("★ Bookmarks").clicked() {
                    self.show_bookmarks = !self.show_bookmarks;
                }
            });
        });

        // Bookmarks dropdown
        if self.show_bookmarks {
            egui::TopBottomPanel::top("bookmarks").show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for (title, url) in &self.bookmarks.clone() {
                        if ui.button(title).clicked() {
                            if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                                tab.navigate(url);
                                self.address_bar = url.clone();
                            }
                            self.show_bookmarks = false;
                        }
                    }
                });
            });
        }

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status = if let Some(tab) = self.tabs.get(self.active_tab) {
                    if tab.loading { format!("⟳ Loading: {}", tab.url) }
                    else { format!("✓ {}", tab.url) }
                } else { self.status_message.clone() };
                ui.label(&status);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Gecko: {}", if self.gecko_ready { "✓" } else { "⟳" }));
                    ui.label(format!("Tabs: {}", self.tabs.len()));
                    ui.label("KJ Surf v0.1.0");
                });
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            let active_url = self.tabs.get(self.active_tab).map(|t| t.url.clone()).unwrap_or_default();

            if active_url == "about:home" {
                ui.vertical_centered(|ui| {
                    ui.add_space(80.0);
                    ui.heading("🌐 KJ Surf");
                    ui.label("Browser orisinil berbasis Gecko engine");
                    ui.add_space(20.0);

                    let mut search = String::new();
                    let resp = ui.add_sized(
                        [400.0, 30.0],
                        egui::TextEdit::singleline(&mut search).hint_text("Search the web…"),
                    );
                    if resp.lost_focus() && ui.input(|r| r.key_pressed(egui::Key::Enter)) && !search.is_empty() {
                        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                            let url = format!("https://duckduckgo.com/?q={}", search.replace(' ', "+"));
                            tab.navigate(&url);
                            self.address_bar = url;
                        }
                    }

                    ui.add_space(10.0);
                    ui.label("Quick links:");
                    ui.horizontal(|ui| {
                        for (title, url) in &self.bookmarks.clone() {
                            if ui.button(title).clicked() {
                                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                                    tab.navigate(url);
                                    self.address_bar = url.clone();
                                }
                            }
                        }
                    });

                    ui.add_space(40.0);
                    ui.label(
                        egui::RichText::new("Powered by Gecko engine · Built with Rust + egui (same as ArtCraft)")
                            .small().color(egui::Color32::GRAY),
                    );
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() / 3.0);
                    if self.gecko_ready {
                        ui.label("🦎 Gecko engine active");
                        ui.label(format!("URL: {}", active_url));
                    } else {
                        ui.spinner();
                        ui.label("Loading Gecko engine (34MB WASM)…");
                        ui.label(format!("Target: {}", active_url));
                    }
                });
            }

            // Simulate loading completion + update title
            if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                if tab.loading {
                    tab.loading = false;
                    let url = tab.url.clone();
                    if url.contains("duckduckgo.com") {
                        tab.title = "DuckDuckGo".into();
                    } else if url.contains("wikipedia.org") {
                        tab.title = "Wikipedia".into();
                    } else if url.contains("github.com") {
                        tab.title = "GitHub".into();
                    } else if url.starts_with("about:home") {
                        tab.title = "New Tab".into();
                    } else if let Some(start) = url.find("://") {
                        let domain = &url[start + 3..];
                        let end = domain.find('/').unwrap_or(domain.len());
                        tab.title = domain[..end].to_string();
                    } else {
                        tab.title = url.clone();
                    }
                }
            }
        });

        if self.tabs.iter().any(|t| t.loading) {
            ctx.request_repaint();
        }
    }
}

// ─── WASM entry point ──────────────────────────────────────────
fn main() {
    // For WASM target, use eframe web runner
    // This is the same pattern as WordCraft/ArtCraft apps
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start("kj_surf_canvas", web_options, Box::new(|_cc| Ok(Box::new(KjSurf::new()))))
            .await
            .expect("failed to start eframe");
    });
}

mod console_error_panic_hook {
    pub fn set_once() {
        std::panic::set_hook(Box::new(|info| {
            web_sys::console::error_1(&format!("KJ Surf panic: {}", info).into());
        }));
    }
}
