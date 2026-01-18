fn main() -> anyhow::Result<()> {
    let app = app::App::new();
    app.run()?;
    Ok(())
}
