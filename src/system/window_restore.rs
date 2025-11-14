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

#[cfg(windows)]
pub fn hide_from_taskbar() {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        FindWindowW, GetWindowLongPtrW, SetWindowLongPtrW, ShowWindow,
        GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_APPWINDOW, SW_HIDE,
    };

    unsafe {
        let title = "xfinder - Recherche intelligente\0"
            .encode_utf16()
            .collect::<Vec<u16>>();

        let hwnd: HWND = FindWindowW(std::ptr::null(), title.as_ptr());

        if hwnd != 0 {
            // Cacher la fenêtre
            ShowWindow(hwnd, SW_HIDE);

            // Retirer WS_EX_APPWINDOW et ajouter WS_EX_TOOLWINDOW pour enlever de la taskbar
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let new_style = (ex_style & !(WS_EX_APPWINDOW as isize)) | WS_EX_TOOLWINDOW as isize;
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);
        }
    }
}

#[cfg(windows)]
pub fn show_in_taskbar() {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        FindWindowW, GetWindowLongPtrW, SetWindowLongPtrW, ShowWindow,
        GWL_EXSTYLE, WS_EX_TOOLWINDOW, WS_EX_APPWINDOW, SW_SHOW,
    };

    unsafe {
        let title = "xfinder - Recherche intelligente\0"
            .encode_utf16()
            .collect::<Vec<u16>>();

        let hwnd: HWND = FindWindowW(std::ptr::null(), title.as_ptr());

        if hwnd != 0 {
            // Remettre le style normal (visible dans taskbar)
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let new_style = (ex_style & !(WS_EX_TOOLWINDOW as isize)) | WS_EX_APPWINDOW as isize;
            SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);

            // Ré-afficher la fenêtre
            ShowWindow(hwnd, SW_SHOW);
        }
    }
}

#[cfg(not(windows))]
pub fn restore_window() {
    // Rien à faire sur non-Windows
}

#[cfg(not(windows))]
pub fn hide_from_taskbar() {
    // Rien à faire sur non-Windows
}

#[cfg(not(windows))]
pub fn show_in_taskbar() {
    // Rien à faire sur non-Windows
}
