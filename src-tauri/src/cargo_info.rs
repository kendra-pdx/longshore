#![allow(dead_code)]

use anyhow::anyhow;
use derive_more::derive::{Deref, From, Into};
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use pest_derive::Parser;
use tokio::process::Command;

use crate::BoxError;

fn span_to_string(span: Span) -> String {
    span.as_str().to_string()
}

#[derive(Debug, FromPest, From, Into, Deref)]
#[pest_ast(rule(Rule::crate_name))]
pub struct CrateName(#[pest_ast(outer(with(span_to_string)))] String);

#[derive(Debug, FromPest, From, Into, Deref)]
#[pest_ast(rule(Rule::tag))]
pub struct Tag(#[pest_ast(outer(with(span_to_string)))] String);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::title))]
pub struct Title {
    pub crate_name: CrateName,
    pub tags: Vec<Tag>,
}

#[derive(Debug, FromPest, From, Into, Deref)]
#[pest_ast(rule(Rule::description))]
pub struct Description(#[pest_ast(outer(with(span_to_string)))] String);

#[derive(Debug, FromPest, From, Into, Deref)]
#[pest_ast(rule(Rule::key))]
pub struct Key(#[pest_ast(outer(with(span_to_string)))] String);

#[derive(Debug, FromPest, From, Into, Deref)]
#[pest_ast(rule(Rule::value))]
pub struct Value(#[pest_ast(outer(with(span_to_string)))] String);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::property))]
pub struct Property {
    pub key: Key,
    pub value: Value,
}

#[derive(Debug, FromPest, From, Into, Deref)]
#[pest_ast(rule(Rule::flag))]
pub struct Flag(#[pest_ast(outer(with(span_to_string)))] String);

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::flags))]
pub struct Flags {
    pub flags: Vec<Flag>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::feature))]
pub struct Feature {
    pub crate_name: CrateName,
    pub flags: Option<Flags>,
    _eoi: Option<Eoi>,
}

#[derive(Debug, Default, FromPest)]
#[pest_ast(rule(Rule::features))]
pub struct Features {
    pub features: Vec<Feature>,
    _eoi: Option<Eoi>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::note))]
pub struct Note {
    #[pest_ast(outer(with(span_to_string)))]
    pub value: String,
    _eoi: Option<Eoi>,
}

#[derive(Debug, Parser, FromPest)]
#[grammar = "cargo_info.pest"]
#[pest_ast(rule(Rule::info))]
pub struct CargoInfo {
    pub title: Title,
    pub description: Vec<Description>,
    pub properties: Vec<Property>,
    pub features: Option<Features>,
    pub note: Option<Note>,
    _eoi: Eoi,
}

impl CargoInfo {
    pub async fn for_crate(crate_name: &str) -> Result<Self, BoxError> {
        println!("looking up info for crate: {crate_name}");

        let cmd_out = Command::new("cargo")
            .arg("info")
            .arg("--quiet")
            .arg(crate_name)
            .output()
            .await?;

        if cmd_out.status.success() {
            let stdout = String::from_utf8(cmd_out.stdout)?;
            let mut rules = CargoInfo::parse(Rule::info, &stdout)?;
            let ast = CargoInfo::from_pest(&mut rules)?;
            Ok(ast)
        } else {
            let stderr = String::from_utf8(cmd_out.stderr)?;
            Err(anyhow!(stderr).into())
        }
    }

    pub fn version(&self) -> Option<&str> {
        fn for_version(p: &Property) -> Option<&str> {
            if *p.key == "version" {
                Some(&p.value)
            } else {
                None
            }
        }
        self.properties.iter().find_map(for_version)
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn parsing() -> anyhow::Result<()> {
        let crates = ["tokio", "uuid", "futures", "derive-new", "thiserror"];
        for name in crates {
            let info = CargoInfo::for_crate(name).await.unwrap();
            println!("{info:#?}");
        }
        Ok(())
    }
}
