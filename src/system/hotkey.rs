// src/system/hotkey.rs
// Hotkey global Ctrl+Shift+F pour restaurer la fenêtre

use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use std::sync::mpsc::{channel, Receiver};

pub struct HotkeyManager {
    _manager: GlobalHotKeyManager,
    _hotkey: HotKey,
    event_rx: Receiver<GlobalHotKeyEvent>,
}

impl HotkeyManager {
    /// Enregistre le hotkey global Ctrl+Shift+F
    pub fn new() -> anyhow::Result<Self> {
        let manager = GlobalHotKeyManager::new()?;

        // Ctrl+Shift+F
        let hotkey = HotKey::new(
            Some(Modifiers::CONTROL | Modifiers::SHIFT),
            Code::KeyF,
        );

        manager.register(hotkey)?;

        // Channel pour les événements
        let (tx, rx) = channel();

        // Thread pour écouter les événements
        std::thread::spawn(move || {
            loop {
                if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                    let _ = tx.send(event);
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });

        Ok(Self {
            _manager: manager,
            _hotkey: hotkey,
            event_rx: rx,
        })
    }

    /// Vérifie si le hotkey a été pressé (non-bloquant)
    pub fn is_triggered(&self) -> bool {
        while let Ok(event) = self.event_rx.try_recv() {
            if event.state == HotKeyState::Pressed {
                return true;
            }
        }
        false
    }
}
