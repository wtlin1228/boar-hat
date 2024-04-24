use anyhow::{self, bail, Ok};
use std::collections::{HashMap, HashSet};

pub const DEFAULT_EXPORT: &'static str = "____DEFAULT__EXPORT____";

#[derive(Debug, PartialEq, Eq)]
pub enum ImportType {
    NamedImport(String), // import { A } from 'some-module'
    DefaultImport,       // import A from 'some-module'
    NamespaceImport,     // import * as A from 'some-module'
    ReExportingAllAs,    // export * as A from 'some-module'
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
    // Important: a symbol can only depend on the symbols in the same module
    pub depend_on: Option<HashSet<String>>,
}

// impl Symbol {
//     pub fn is_namespace_import(&self) -> bool {
//         match self.import_from {
//             Some(ref import_type) => match import_type.import_type {
//                 ImportType::NamespaceImport => true,
//                 _ => false,
//             },
//             None => false,
//         }
//     }

//     pub fn get_symbol_names_depend_on_the_namespace(&self) -> anyhow::Result<Vec<&str>> {
//         match self.import_from {
//             Some(ref import_type) => match import_type.import_type {
//                 ImportType::NamespaceImport(ref names) => {
//                     Ok(names.iter().map(|name| name.as_str()).collect())
//                 }
//                 _ => bail!("This method is only available for ImportType::NamespaceImport"),
//             },
//             None => bail!("This method is only available for ImportType::NamespaceImport"),
//         }
//     }
// }

#[derive(Debug)]
pub struct Module {
    // Step2.1 will collect `export * from 'some-module'` into `re_exporting_all_from` field.
    // Step2.4 will expand all the `re_exporting_all_from` into local symbols.
    //         It's ok that the expansion happens after finishing tracing the symbols'
    //         dependency since local symbols are not able to use `re_exporting_all_from`'s
    //         symbols.
    pub re_exporting_all_from: Option<Vec<String>>,

    pub symbols: HashMap<String, Symbol>,
}
