use std::collections::BTreeMap;

use anyhow::anyhow;
use derive_more::derive::{Deref, Display};
use serde::{Deserialize, Serialize};

use crate::{cargo_info::CargoInfo, BoxError};

#[derive(Debug, derive_new::new, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CategoryName(#[new(into)] String);

#[derive(
    Debug,
    derive_new::new,
    Serialize,
    Deserialize,
    Deref,
    Display,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[display("{_0}")]
pub struct PackageName(#[new(into)] String);

#[derive(
    Debug, derive_new::new, Serialize, Deserialize, Deref, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Feature(#[new(into)] String);

#[derive(Debug, derive_new::new, Serialize, Deserialize)]
pub struct PackageDetails {
    #[new(into)]
    pub version: String,
    #[new(default)]
    pub features: BTreeMap<Feature, bool>,
}

pub type _BasicCategories = BTreeMap<CategoryName, BTreeMap<PackageName, Vec<Feature>>>;
pub type _FullCategories = BTreeMap<CategoryName, BTreeMap<PackageName, PackageDetails>>;

#[derive(Debug, Clone, Serialize, Deserialize, derive_new::new, Deref)]
pub struct BasicCategories(#[new(default)] pub BTreeMap<CategoryName, BTreeMap<PackageName, Vec<Feature>>>);

#[derive(Debug, Serialize, Deserialize, derive_new::new, Deref)]
pub struct FullCategories(#[new(default)] BTreeMap<CategoryName, BTreeMap<PackageName, PackageDetails>>);


#[tauri::command]
pub async fn lookup_packages(categories: BasicCategories) -> Result<FullCategories, String> {
    let mut full = FullCategories::new();
    for (category, packages) in &categories.0 {
        let mut cat_pkgs = BTreeMap::new();
        for (package_name, enabled_features) in packages {
            let pkg = lookup_package(&package_name, &enabled_features).await.map_err(|e| e.to_string())?;
            cat_pkgs.insert(package_name.clone(), pkg);
        }
        full.0.insert(category.clone(), cat_pkgs);
    }

    Ok(full)
}

async fn lookup_package(
    package_name: &PackageName,
    enabled: &[Feature],
) -> Result<PackageDetails, BoxError> {
    let info = CargoInfo::for_crate(&package_name).await?;
    let version = info.version().ok_or(anyhow!(
        "crate {} must have a version. none found.",
        package_name
    ))?;
    let mut pkg_details = PackageDetails::new(version);
    for feature in info.features.unwrap_or_default().features {
        let enabled = enabled.iter().any(|f| **f == *feature.crate_name);
        pkg_details
            .features
            .insert(Feature(feature.crate_name.into()), enabled);
    }
    Ok(pkg_details)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map;

    #[tokio::test]
    async fn usage() {
        let categories = BasicCategories(map!(
            CategoryName::new("utility") => map!(
                PackageName::new("tokio") => vec![Feature::new("full")]
            ),
            CategoryName::new("data") => map!(
                PackageName::new("uuid") => vec![Feature::new("derive")],
                PackageName::new("chrono") => vec![]
            )
        ));
        println!("{categories:?}");
        let full = lookup_packages(categories.clone()).await.unwrap();
        println!("{full:#?}");

        let json = serde_json::to_string_pretty(&categories).unwrap();
        println!("{json}");

        let toml = toml::to_string_pretty(&categories).unwrap();
        println!("{toml}");

        let toml: &str = r#"
        [test]
        foo = []
        bar = ["a"]

        [tset]
        baz = ["b", "c"]
        "#;
        let categories: BasicCategories = toml::from_str(&toml).unwrap();
        println!("{categories:?}");
    }
}