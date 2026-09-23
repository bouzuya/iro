mod app;
mod hsl;
mod named_colors;
mod rgb;

pub use self::hsl::Hsl;
pub use self::named_colors::NamedColors;
pub use self::rgb::Rgb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ::topcoat::start(self::app::router()?).await?;
    Ok(())
}
