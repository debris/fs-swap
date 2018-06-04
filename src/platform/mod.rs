#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use self::linux::swap;
#[cfg(target_os = "macos")]
pub use self::macos::swap;
#[cfg(windows)]
pub use self::windows::swap;
