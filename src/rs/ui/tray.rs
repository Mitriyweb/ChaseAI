use crate::config::network_config::NetworkConfig;
use crate::ui::tray_menu;
use std::path::Path;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

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

    pub fn setup(&mut self, config: &NetworkConfig) -> anyhow::Result<()> {
        println!("Setting up tray icon...");
        let tray_menu = tray_menu::build_menu(config)?;
        println!("Menu built successfully");

        // Get the executable path and construct paths relative to it
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap();

        // Try multiple possible icon paths (for both dev and app bundle)
        // For menu bar, we want a larger icon that scales well (64x64 or higher)
        let mut possible_paths = vec![];

        // For .app bundle: Contents/MacOS/../Resources/icon_menubar_64.png
        if let Ok(bundle_icon) = exe_dir
            .join("../Resources/icon_menubar_64.png")
            .canonicalize()
        {
            possible_paths.push(bundle_icon);
        }

        // Fallback to regular icon
        if let Ok(bundle_icon) = exe_dir.join("../Resources/icon_menubar.png").canonicalize() {
            possible_paths.push(bundle_icon);
        }

        // For development: from project root
        possible_paths.push(Path::new("resources/icon_menubar_64.png").to_path_buf());
        possible_paths.push(Path::new("resources/icon_menubar.png").to_path_buf());
        possible_paths.push(Path::new("./resources/icon_menubar.png").to_path_buf());

        let icon = possible_paths.iter().find_map(|path| {
            println!("Trying icon path: {}", path.display());
            Self::load_icon(path).ok()
        });

        if icon.is_some() {
            println!("Icon loaded successfully");
        } else {
            println!("Warning: Could not load icon from any path, using default");
        }

        let mut builder = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu) as Box<dyn tray_icon::menu::ContextMenu>)
            .with_tooltip("ChaseAI - Network Management");

        if let Some(icon) = icon {
            builder = builder.with_icon(icon);
        }

        let tray_icon = builder.build()?;

        #[cfg(target_os = "macos")]
        {
            tray_icon.set_icon_as_template(true);
        }

        self.tray_icon = Some(tray_icon);
        println!("Tray icon created successfully");

        Ok(())
    }

    pub fn update_menu(&mut self, config: &NetworkConfig) -> anyhow::Result<()> {
        if let Some(tray_icon) = &mut self.tray_icon {
            let new_menu = tray_menu::build_menu(config)?;
            tray_icon.set_menu(Some(Box::new(new_menu) as Box<dyn tray_icon::menu::ContextMenu>));
        }
        Ok(())
    }
}
