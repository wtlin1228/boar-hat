use anyhow::{self, bail, Ok};
use std::collections::{HashMap, HashSet};

pub const DEFAULT_EXPORT: &'static str = "____DEFAULT__EXPORT____";

#[derive(Debug, PartialEq, Eq)]
pub enum ImportType {
    NamedImport(String),
    DefaultImport,
    NamespaceImport(Vec<String>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub from: String,
    pub import_type: ImportType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub is_named_exported: bool,
    pub import_from: Option<Import>,
    // a symbol can only depend on the symbols in the same module
    pub depend_on: Option<HashSet<String>>,
}

impl Symbol {
    pub fn is_namespace_import(&self) -> bool {
        match self.import_from {
            Some(ref import_type) => match import_type.import_type {
                ImportType::NamespaceImport(_) => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn get_symbol_names_depend_on_the_namespace(&self) -> anyhow::Result<Vec<&str>> {
        match self.import_from {
            Some(ref import_type) => match import_type.import_type {
                ImportType::NamespaceImport(ref names) => {
                    Ok(names.iter().map(|name| name.as_str()).collect())
                }
                _ => bail!("This method is only available for ImportType::NamespaceImport"),
            },
            None => bail!("This method is only available for ImportType::NamespaceImport"),
        }
    }
}

#[derive(Debug)]
pub struct Module {
    pub has_namespace_import: bool,
    pub symbols: HashMap<String, Symbol>,
}
