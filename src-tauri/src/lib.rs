use std::error::Error;

use anyhow::anyhow;
use packages::{lookup_packages, BasicCategories};
use tauri::{App, Manager};

mod cargo_info;
mod packages;

type BoxError = Box<dyn Error + Send + Sync>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            packages::lookup_packages,
            packages::get_categories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    tauri::async_runtime::block_on(async {
        let packages: BasicCategories = toml::from_str(include_str!("crates.toml"))?;
        app.manage(packages.clone());

        let packages = lookup_packages(packages).await.map_err(|e| anyhow!(e))?;
        app.manage(packages);

        Ok(())
    })
}

