use std::collections::BTreeMap;
use std::path::Path;

#[derive(serde::Deserialize)]
pub struct Config {
    pub lighthouse_geometry: Option<String>,
    pub active_area: Option<ActiveArea>,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct ActiveArea {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

#[derive(Clone, Copy)]
pub struct BaseStation {
    pub origin: [f32; 3],
}

pub fn load_config(path: &str) -> Config {
    match std::fs::read_to_string(path) {
        Ok(text) => match toml::from_str(&text) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to parse {}: {}", path, e);
                Config { lighthouse_geometry: None, active_area: None }
            }
        },
        Err(e) => {
            eprintln!("No config file at {}: {}", path, e);
            Config { lighthouse_geometry: None, active_area: None }
        }
    }
}

pub fn load_lighthouse_geometry(yaml_path: &str, config_dir: &Path) -> Vec<BaseStation> {
    let full_path = if Path::new(yaml_path).is_absolute() {
        yaml_path.to_string()
    } else {
        config_dir.join(yaml_path).to_string_lossy().to_string()
    };

    let text = match std::fs::read_to_string(&full_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to read lighthouse geometry {}: {}", full_path, e);
            return Vec::new();
        }
    };

    // Parse just the geos section
    #[derive(serde::Deserialize)]
    struct GeoEntry {
        origin: [f64; 3],
    }

    #[derive(serde::Deserialize)]
    struct LighthouseFile {
        #[serde(default)]
        geos: BTreeMap<u32, GeoEntry>,
    }

    match serde_yaml::from_str::<LighthouseFile>(&text) {
        Ok(lh) => {
            let stations: Vec<BaseStation> = lh.geos.values().map(|g| {
                BaseStation {
                    origin: [g.origin[0] as f32, g.origin[1] as f32, g.origin[2] as f32],
                }
            }).collect();
            eprintln!("Loaded {} lighthouse base stations from {}", stations.len(), full_path);
            stations
        }
        Err(e) => {
            eprintln!("Failed to parse lighthouse geometry {}: {}", full_path, e);
            Vec::new()
        }
    }
}
