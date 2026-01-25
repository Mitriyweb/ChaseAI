mod platform;

// Re-export app module so it's accessible from main
pub use app::App;

fn main() -> anyhow::Result<()> {
    platform::run()
}
