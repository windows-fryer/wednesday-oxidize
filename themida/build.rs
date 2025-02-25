fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if we're in release mode
    if std::env::var("PROFILE")? != "release" {
        return Ok(());
    }

    println!(
        "cargo:rustc-link-search={}/lib",
        std::env::var("CARGO_MANIFEST_DIR")?
    );

    Ok(())
}
