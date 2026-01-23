use crate::network::interface_detector::InterfaceDetector;
use std::path::Path;
use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

pub struct TrayManager {
    tray_icon: Option<TrayIcon>,
}

impl TrayManager {
    pub fn new() -> Self {
        Self { tray_icon: None }
    }
}

impl Default for TrayManager {
    fn default() -> Self {
        Self::new()
    }
}
impl TrayManager {
    pub fn load_icon(path: &Path) -> anyhow::Result<Icon> {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(path)?.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        Ok(Icon::from_rgba(icon_rgba, icon_width, icon_height)?)
    }

    pub fn setup(&mut self) -> anyhow::Result<()> {
        let tray_menu = Menu::new();

        // Add interfaces
        tray_menu.append(&MenuItem::new("Interfaces:", false, None))?;
        let interfaces = InterfaceDetector::detect_all()?;
        for interface in interfaces {
            let label = format!("  {} ({})", interface.name, interface.ip_address);
            tray_menu.append(&MenuItem::new(&label, true, None))?;
        }

        tray_menu.append(&PredefinedMenuItem::separator())?;
        tray_menu.append(&MenuItem::new("Settings...", true, None))?;
        tray_menu.append(&PredefinedMenuItem::quit(None))?;

        let icon_path = Path::new("resources/icon.png");
        let icon = Self::load_icon(icon_path).ok(); // Fallback to no icon if loading fails

        let mut builder = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("ChaseAI - Network Management");

        if let Some(icon) = icon {
            builder = builder.with_icon(icon);
        }

        self.tray_icon = Some(builder.build()?);

        Ok(())
    }
}
