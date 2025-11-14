// src/system/tray.rs
// System tray icon avec menu contextuel

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIcon, TrayIconBuilder, Icon,
};
use std::sync::mpsc::{channel, Receiver};

#[derive(Debug)]
pub enum TrayEvent {
    Show,
    Settings,
    StartIndexing,
    Quit,
}

pub struct SystemTray {
    _tray_icon: TrayIcon,
    event_rx: Receiver<TrayEvent>,
}

impl SystemTray {
    pub fn new() -> anyhow::Result<Self> {
        // Créer le menu contextuel
        let tray_menu = Menu::new();

        let show_item = MenuItem::new("📂 Ouvrir xfinder", true, None);
        let start_indexing_item = MenuItem::new("🔄 Lancer l'indexation", true, None);
        let settings_item = MenuItem::new("⚙️ Paramètres", true, None);
        let separator = PredefinedMenuItem::separator();
        let quit_item = MenuItem::new("❌ Quitter", true, None);

        tray_menu.append(&show_item)?;
        tray_menu.append(&start_indexing_item)?;
        tray_menu.append(&separator)?;
        tray_menu.append(&settings_item)?;
        tray_menu.append(&separator)?;
        tray_menu.append(&quit_item)?;

        // Créer l'icône (utiliser une icône par défaut pour l'instant)
        // TODO: Ajouter une vraie icône depuis assets/
        let icon = Self::create_default_icon();

        // Créer le tray icon
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("xfinder - Recherche de fichiers")
            .with_icon(icon)
            .build()?;

        // Channel pour les événements
        let (tx, rx) = channel();

        // Écouter les événements du menu
        let show_id = show_item.id().clone();
        let start_indexing_id = start_indexing_item.id().clone();
        let settings_id = settings_item.id().clone();
        let quit_id = quit_item.id().clone();

        std::thread::spawn(move || {
            let menu_event_rx = MenuEvent::receiver();
            loop {
                if let Ok(event) = menu_event_rx.recv() {
                    if event.id == show_id {
                        // Restaurer immédiatement (remettre dans taskbar + restaurer)
                        crate::system::window_restore::show_in_taskbar();
                        crate::system::window_restore::restore_window();
                        let _ = tx.send(TrayEvent::Show);
                    } else if event.id == start_indexing_id {
                        let _ = tx.send(TrayEvent::StartIndexing);
                    } else if event.id == settings_id {
                        // Restaurer immédiatement (remettre dans taskbar + restaurer)
                        crate::system::window_restore::show_in_taskbar();
                        crate::system::window_restore::restore_window();
                        let _ = tx.send(TrayEvent::Settings);
                    } else if event.id == quit_id {
                        let _ = tx.send(TrayEvent::Quit);
                    }
                }
            }
        });

        Ok(Self {
            _tray_icon: tray_icon,
            event_rx: rx,
        })
    }

    /// Créer une icône par défaut (simple carré pour l'instant)
    fn create_default_icon() -> Icon {
        // Créer une icône 32x32 simple
        let width = 32;
        let height = 32;
        let mut rgba = Vec::with_capacity(width * height * 4);

        for y in 0..height {
            for x in 0..width {
                // Dessiner un carré bleu avec bordure blanche
                if x < 2 || x >= width - 2 || y < 2 || y >= height - 2 {
                    // Bordure blanche
                    rgba.extend_from_slice(&[255, 255, 255, 255]);
                } else {
                    // Fond bleu
                    rgba.extend_from_slice(&[0, 120, 215, 255]);
                }
            }
        }

        Icon::from_rgba(rgba, width as u32, height as u32)
            .expect("Failed to create icon")
    }

    /// Récupérer les événements du tray (non-bloquant)
    pub fn poll_events(&self) -> Vec<TrayEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_rx.try_recv() {
            events.push(event);
        }
        events
    }
}
