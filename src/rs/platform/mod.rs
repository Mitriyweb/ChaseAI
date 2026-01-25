#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub use macos::run;

#[cfg(target_os = "windows")]
pub use windows::run;

#[cfg(target_os = "linux")]
pub use linux::run;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn run() -> anyhow::Result<()> {
    eprintln!("Unsupported operating system");
    std::process::exit(1);
}
