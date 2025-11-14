// src/system/window_restore.rs
// Restauration de fenêtre Windows native

#[cfg(windows)]
pub fn restore_window() {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        FindWindowW, ShowWindow, SetForegroundWindow, SW_RESTORE
    };

    unsafe {
        // Trouver la fenêtre par son titre
        let title = "xfinder - Recherche intelligente\0"
            .encode_utf16()
            .collect::<Vec<u16>>();

        let hwnd: HWND = FindWindowW(std::ptr::null(), title.as_ptr());

        if hwnd != 0 {
            // Restaurer la fenêtre depuis minimisée
            ShowWindow(hwnd, SW_RESTORE);
            // Mettre au premier plan
            SetForegroundWindow(hwnd);
        }
    }
}

#[cfg(not(windows))]
pub fn restore_window() {
    // Rien à faire sur non-Windows
}
