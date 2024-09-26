use winapi::um::winuser::{GetForegroundWindow, GetWindowRect, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use winapi::shared::windef::{RECT, HWND__};
use winapi::um::dwmapi::DwmGetWindowAttribute;
use winapi::um::dwmapi::DWMWA_EXTENDED_FRAME_BOUNDS;

pub fn deny_quicknote() -> bool {
    unsafe  {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return false;
        }
        is_fullscreen(hwnd)
        // TODO - allow users to add a list of window names to disable quicknote
    }
}

fn is_fullscreen(hwnd: *mut HWND__) -> bool {
    unsafe {
        // Get the screen resolution
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);

        // Get the window rectangle
        let mut rect: RECT = std::mem::zeroed();
        if GetWindowRect(hwnd, &mut rect) == 0 {
            return false;
        }

        // Compare window dimensions to screen size
        let window_width = rect.right - rect.left;
        let window_height = rect.bottom - rect.top;

        if window_width == screen_width && window_height == screen_height {
            // Further check if the window has no decorations (i.e., it's borderless)
            let mut frame_rect: RECT = std::mem::zeroed();
            if DwmGetWindowAttribute(
                hwnd, 
                DWMWA_EXTENDED_FRAME_BOUNDS, 
                &mut frame_rect as *mut RECT as *mut _, 
                std::mem::size_of::<RECT>() as u32
            ) == 0 {
                return frame_rect.left == rect.left && frame_rect.right == rect.right &&
                       frame_rect.top == rect.top && frame_rect.bottom == rect.bottom;
            }
        }
        false
    }
}

