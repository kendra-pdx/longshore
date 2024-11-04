use std::error::Error;

use anyhow::anyhow;
use packages::{lookup_packages, BasicCategories};
use tauri::{App, Manager};

mod packages;
mod cargo_info;

type BoxError = Box<dyn Error + Send + Sync>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![packages::lookup_packages])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    tauri::async_runtime::block_on(async {
        let packages: BasicCategories = toml::from_str(include_str!("crates.toml"))?;

        let packages = lookup_packages(packages).await.map_err(|e| anyhow!(e))?;
        app.manage(packages);

        Ok(())
    })
}

macro_rules! map {
    ($($k:expr => $v:expr),*) => ({
        let mut map = std::collections::BTreeMap::new();
        $(
            map.insert($k, $v);
        )*
        map
    });
}
use map;