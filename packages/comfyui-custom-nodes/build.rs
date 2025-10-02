use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use toml::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");

    // Read the Cargo.toml file
    let mut cargo_toml_content = String::new();
    let mut file = File::open(&cargo_toml_path)?;
    file.read_to_string(&mut cargo_toml_content)?;

    // Parse the TOML content
    let parsed: Value = toml::from_str(&cargo_toml_content)?;

    // Extract the crate name
    let crate_name = parsed
        .get("lib")
        .or_else(|| parsed.get("package"))
        .and_then(|package| package.get("name"))
        .and_then(|name| name.as_str())
        .ok_or("Failed to extract crate name from Cargo.toml. Please ensure your Cargo.toml has a valid [package] name field.")?
        .replace("-", "_");

    // Create __init__.py in the project root
    let init_file_path = Path::new(&manifest_dir).join("__init__.py");

    let mut init_file = File::create(&init_file_path)?;
    writeln!(init_file, "# Auto-generated __init__.py")?;
    writeln!(init_file, "from {} import *", crate_name)?;

    println!("Created __init__.py at: {}", init_file_path.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    Ok(())
}
