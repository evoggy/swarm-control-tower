fn main() {
    slint_build::compile("ui/app.slint").expect("Slint build failed");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let config_path = std::path::Path::new(&manifest_dir).join("config.toml");

    println!("cargo:rerun-if-changed={}", config_path.display());

    let text = std::fs::read_to_string(&config_path)
        .expect("Failed to read config.toml");

    let config: toml::Value = toml::from_str(&text).expect("Failed to parse config.toml");

    let lighthouse = config
        .get("lighthouse_geometry")
        .and_then(|v| v.as_str())
        .unwrap_or("Lighthouse_Cage.yaml");
    println!("cargo:rustc-env=LIGHTHOUSE_YAML={}", lighthouse);

    let area = config.get("active_area").expect("Missing [active_area] in config.toml");
    for (toml_key, env_key) in &[
        ("min_x", "SETTINGS_MIN_X_BOUND"),
        ("max_x", "SETTINGS_MAX_X_BOUND"),
        ("min_y", "SETTINGS_MIN_Y_BOUND"),
        ("max_y", "SETTINGS_MAX_Y_BOUND"),
    ] {
        let value = area.get(toml_key)
            .and_then(|v| v.as_float())
            .unwrap_or_else(|| panic!("Missing active_area.{} in config.toml", toml_key));
        println!("cargo:rustc-env={}={}", env_key, value);
    }
}
