fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ::topcoat::tailwind::BuildConfig::new()
        .input("styles.css")
        .render()?;
    Ok(())
}
