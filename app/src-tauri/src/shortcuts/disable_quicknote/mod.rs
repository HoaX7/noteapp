#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;

pub fn deny_quicknote() -> bool {
    #[cfg(target_os = "windows")]
    {
        windows::deny_quicknote()
    }

    #[cfg(target_os = "macos")]
    {
        macos::deny_quicknote()
    }
}
