//! Dependency Tracker.
//!
//! [WIP] Introduction ...
//!
//! # Symbol
//!
//! Symbol is the basic unit used internally in `DependencyTracker`. We can get the
//! information about "Is it exported?", "Does it depends on other symbols in the
//! same module?", "Is it imported from other module?".
//!
//! ## Examples
//!
//! ### Default Import
//!
//! ```js
//! import A from "module-a";
//! ```
//!
//! In symbol representation:
//!
//! ```rs
//! Symbol {
//!   name: "A",
//!   is_named_exported: false,
//!   import_from: Some(
//!     Import {
//!       from: "module-a",
//!       import_type: ImportType::DefaultImport
//!   }),
//!   depend_on: None
//! }
//! ```
//!
//! ### Named Import
//!
//! ```js
//! import { A as B } from "module-a";
//! ```
//!
//! In symbol representation:
//!
//! ```rs
//! Symbol {
//!   name: "B",
//!   is_named_exported: false,
//!   import_from: Some(
//!     Import {
//!       from: "module-a",
//!       import_type: ImportType::NamedImport("A")
//!   }),
//!   depend_on: None
//! }
//! ```
//!
//! ### Namespace Import
//!
//! ```js
//! import * as A from "module-a";
//! ```
//!
//! In symbol representation:
//!
//! ```rs
//! Symbol {
//!   name: "A",
//!   is_named_exported: false,
//!   import_from: Some(
//!     Import {
//!       from: "module-a",
//!       import_type: ImportType::NamespaceImport("A")
//!   }),
//!   depend_on: None
//! }
//! ```
//!
//! ### Named Export
//!
//! ```js
//! export A;
//! ```
//!
//! In symbol representation:
//!
//! ```rs
//! Symbol {
//!   name: "A",
//!   is_named_exported: true,
//!   import_from: None,
//!   depend_on: None
//! }
//! ```
//!
//! ### Default Export
//!
//! ```js
//! export default A;
//! ```
//!
//! In symbol representation:
//!
//! ```rs
//! Symbol {
//!   name: "____DEFAULT__EXPORT____",
//!   is_named_exported: false,
//!   import_from: None,
//!   depend_on: None
//! }
//! ```
//!
//! ### Rename Export
//!
//! ```js
//! export { A as B };
//! ```
//!
//! In symbol representation:
//!
//! ```rs
//! Symbol {
//!   name: "B",
//!   is_named_exported: true,
//!   import_from: None,
//!   depend_on: Some(HashSet(["A"]))
//! }
//! ```
//!
//! ### Re-exporting
//!
//! ```js
//! export { A as B } from "module-a";
//! ```
//!
//! In symbol representation:
//!
//! ```rs
//! Symbol {
//!   name: "B",
//!   is_named_exported: true,
//!   import_from: Some(
//!     Import {
//!       from: "module-a",
//!       import_type: ImportType::NamedImport("A")
//!   }),
//!   depend_on: None
//! }
//! ```
//!
//! ### Re-exporting Default
//!
//! ```js
//! export { Default as A } from "module-a";
//! ```
//!
//! In symbol representation:
//!
//! ```rs
//! Symbol {
//!   name: "A",
//!   is_named_exported: true,
//!   import_from: Some(
//!     Import {
//!       from: "module-a",
//!       import_type: ImportType::DefaultExport
//!   }),
//!   depend_on: None
//! }
//! ```
//!
//! # Parsing Order
//!
//! The parsing order for JavaScript modules `module-a` and `module-b` below
//! will be determined by the `Scheduler`. `Scheduler` will parse the `module-b`
//! first then `module-a` because `module-a` imports the namespace of `module-b`.
//!
//! ```js
//! // module-b.js
//! export Header;
//! export Body;
//! export Footer;
//!
//! // module-a.js
//! import * as UI from "module-b";
//! const A = UI;
//! ```
//!
//! # Expansion of the Namespace Import
//!
//! The goal of "expansion" is to replace the all the namespace imports with named exports.
//!
//! Let's continue with the "module-a" and "module-b" example in the parsing order section.
//!
//! "module-b" will be parsed into the symbol representation like this:
//!
//! ```rs
//! Symbol { name: "Header", is_named_exported: true, import_from: None, depend_on: None }
//! Symbol { name: "Body", is_named_exported: true, import_from: None, depend_on: None }
//! Symbol { name: "Footer", is_named_exported: true, import_from: None, depend_on: None }
//! ```
//!
//! And "module-a" will be parsed into the symbol representation like this:
//!
//! ```rs
//! Symbol {
//!   name: "A",
//!   is_named_exported: false,
//!   import_from: None,
//!   depend_on: Some(HashSet(["UI"]))
//! }
//!
//! Symbol {
//!   name: "UI",
//!   is_named_exported: false,
//!   import_from: Some(
//!     Import {
//!       from: "module-name"
//!       import_type: ImportType::NamespaceImport(vec!["A"])
//!   }),
//!   depend_on: None
//! }
//! ```
//!
//! After the expansion of "module-a", the new symbol representation becomes:
//!
//! ```rs
//! Symbol {
//!   name: "A",
//!   is_named_exported: false,
//!   import_from: None,
//!   depend_on: Some(HashSet(["Header", "Body", "Footer"]))
//! }
//!
//! Symbol {
//!   name: "Header",
//!   is_named_exported: false,
//!   import_from: Some(
//!     Import {
//!       from: "module-name"
//!       import_type: ImportType::NamedImport("Header")
//!   }),
//!   depend_on: None
//! }
//!
//! Symbol {
//!   name: "Body",
//!   is_named_exported: false,
//!   import_from: Some(
//!     Import {
//!       from: "module-name"
//!       import_type: ImportType::NamedImport("Body")
//!   }),
//!   depend_on: None
//! }
//!
//! Symbol {
//!   name: "Footer",
//!   is_named_exported: false,
//!   import_from: Some(
//!     Import {
//!       from: "module-name"
//!       import_type: ImportType::NamedImport("Footer")
//!   }),
//!   depend_on: None
//! }
//! ```
//!
//! You should notice that the `Symbol UI` in "module-a" is removed. Instead, all the
//! named exported symbols `Symbol Header`, `Symbol Body` and `Symbol Footer` are added
//! into "module-a". Another important thing is `Symbol A`'s dependency is also updated.
//!

mod js_module_parser;
mod path_resolver;
mod scheduler;
pub mod visitors;

use anyhow::{self, bail, Context, Ok};
use js_module_parser::module::{Import, ImportType, Module, Symbol};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DependencyTracker {
    root: String,
    parsed_modules: HashMap<String, Module>,
}

impl DependencyTracker {
    fn expand_namespace_import(&mut self, module_name: &str) -> anyhow::Result<()> {
        let module = self
            .parsed_modules
            .get_mut(module_name)
            .context(format!("Module {} not found", module_name))?;

        if !module.has_namespace_import {
            return Ok(());
        }

        let namespace_import_symbol_names: Vec<String> = module
            .symbols
            .iter()
            .filter(|&(_, s)| s.is_namespace_import())
            .map(|(name, _)| name.clone())
            .collect();
        let mut namespace_import_symbols: Vec<Symbol> = vec![];
        for symbol_name in namespace_import_symbol_names.iter() {
            namespace_import_symbols.push(module.symbols.remove(symbol_name).unwrap());
        }

        for namespace_import_symbol in namespace_import_symbols.iter() {
            let import = namespace_import_symbol
                .import_from
                .as_ref()
                .context("Namespace import must import from some module")?;

            let import_from_module = self
                .parsed_modules
                .get(&import.from)
                .context(format!("The imported module {} not found", &import.from))?;

            if import_from_module.has_namespace_import {
                bail!(
                    "The imported module {} hasn't expand its own namespace import yet",
                    &import.from
                )
            }

            let mut collect_exported_symbols: Vec<Symbol> = vec![];
            let mut collect_exported_symbol_names: Vec<String> = vec![];
            for (symbol_name, _) in import_from_module
                .symbols
                .iter()
                // The reason we can just filter out the symbol whose `is_named_exported`
                // is true is because the `is_named_exported` of symbol "DEFAULT_EXPORT"
                // is always `false`.
                .filter(|&(_, symbol)| symbol.is_named_exported)
            {
                collect_exported_symbol_names.push(symbol_name.clone());
                collect_exported_symbols.push(Symbol {
                    name: symbol_name.clone(),
                    is_named_exported: false,
                    import_from: Some(Import {
                        from: import.from.clone(),
                        import_type: ImportType::NamedImport(symbol_name.clone()),
                    }),
                    depend_on: None,
                })
            }

            let module = self
                .parsed_modules
                .get_mut(module_name)
                .context(format!("Module {} not found", module_name))?;

            for to_update_symbol_name in
                namespace_import_symbol.get_symbol_names_depend_on_the_namespace()?
            {
                let depend_on = module
                    .symbols
                    .get_mut(to_update_symbol_name)
                    .context(format!(
                        "Symbol {} not found in Module {}",
                        to_update_symbol_name, module_name
                    ))?
                    .depend_on
                    .as_mut()
                    .context(format!(
                        "Symbol {} will at least depend on the namespace {}",
                        to_update_symbol_name, namespace_import_symbol.name
                    ))?;
                depend_on.remove(&namespace_import_symbol.name);
                collect_exported_symbol_names.iter().for_each(|name| {
                    depend_on.insert(name.clone());
                });
            }

            collect_exported_symbols.into_iter().for_each(|symbol| {
                let module_has_symbol_already = module.symbols.get(&symbol.name);
                if module_has_symbol_already.is_none() {
                    module.symbols.insert(symbol.name.clone(), symbol);
                }
            })
        }

        self.parsed_modules
            .get_mut(module_name)
            .context(format!("Module {} not found", module_name))?
            .has_namespace_import = false;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::js_module_parser::module::{Import, ImportType, Module, Symbol, DEFAULT_EXPORT};
    use super::DependencyTracker;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_namespace_import_expansion() {
        let mut d = DependencyTracker {
            root: "depend/dency/track/ker".to_string(),
            parsed_modules: HashMap::from([
                (
                    "Module X".to_string(),
                    Module {
                        has_namespace_import: false,
                        symbols: HashMap::from([
                            (
                                DEFAULT_EXPORT.to_string(),
                                Symbol {
                                    name: DEFAULT_EXPORT.to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: Some(HashSet::from(["x1".to_string()])),
                                },
                            ),
                            (
                                "x1".to_string(),
                                Symbol {
                                    name: "x1".to_string(),
                                    is_named_exported: true,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                            (
                                "x2".to_string(),
                                Symbol {
                                    name: "x2".to_string(),
                                    is_named_exported: true,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                            (
                                "x3".to_string(),
                                Symbol {
                                    name: "x3".to_string(),
                                    is_named_exported: true,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                        ]),
                    },
                ),
                (
                    "Module Y".to_string(),
                    Module {
                        has_namespace_import: false,
                        symbols: HashMap::from([
                            (
                                DEFAULT_EXPORT.to_string(),
                                Symbol {
                                    name: DEFAULT_EXPORT.to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: Some(HashSet::from(["y1".to_string()])),
                                },
                            ),
                            (
                                "y1".to_string(),
                                Symbol {
                                    name: "y1".to_string(),
                                    is_named_exported: true,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                            (
                                "y2".to_string(),
                                Symbol {
                                    name: "y2".to_string(),
                                    is_named_exported: true,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                            (
                                "y3".to_string(),
                                Symbol {
                                    name: "y3".to_string(),
                                    is_named_exported: true,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                        ]),
                    },
                ),
                (
                    "Module Z".to_string(),
                    Module {
                        has_namespace_import: false,
                        symbols: HashMap::from([
                            (
                                DEFAULT_EXPORT.to_string(),
                                Symbol {
                                    name: DEFAULT_EXPORT.to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: Some(HashSet::from(["z1".to_string()])),
                                },
                            ),
                            (
                                "z1".to_string(),
                                Symbol {
                                    name: "z1".to_string(),
                                    is_named_exported: true,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                            (
                                "z2".to_string(),
                                Symbol {
                                    name: "z2".to_string(),
                                    is_named_exported: true,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                            (
                                "z3".to_string(),
                                Symbol {
                                    name: "z3".to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: None,
                                },
                            ),
                        ]),
                    },
                ),
                (
                    "Module A".to_string(),
                    Module {
                        has_namespace_import: true,
                        symbols: HashMap::from([
                            (
                                DEFAULT_EXPORT.to_string(),
                                Symbol {
                                    name: DEFAULT_EXPORT.to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: Some(HashSet::from(["A".to_string()])),
                                },
                            ),
                            (
                                "A".to_string(),
                                Symbol {
                                    name: "A".to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: Some(HashSet::from(["C".to_string()])),
                                },
                            ),
                            (
                                "B".to_string(),
                                Symbol {
                                    name: "B".to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: Some(HashSet::from([
                                        "C".to_string(),
                                        "z".to_string(),
                                    ])),
                                },
                            ),
                            (
                                "C".to_string(),
                                Symbol {
                                    name: "C".to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: Some(HashSet::from([
                                        "x".to_string(),
                                        "y".to_string(),
                                    ])),
                                },
                            ),
                            (
                                "D".to_string(),
                                Symbol {
                                    name: "D".to_string(),
                                    is_named_exported: false,
                                    import_from: None,
                                    depend_on: Some(HashSet::from(["z1".to_string()])),
                                },
                            ),
                            (
                                "x".to_string(),
                                Symbol {
                                    name: "x".to_string(),
                                    is_named_exported: false,
                                    import_from: Some(Import {
                                        from: "Module X".to_string(),
                                        import_type: ImportType::DefaultImport,
                                    }),
                                    depend_on: None,
                                },
                            ),
                            (
                                "y".to_string(),
                                Symbol {
                                    name: "y".to_string(),
                                    is_named_exported: false,
                                    import_from: Some(Import {
                                        from: "Module Y".to_string(),
                                        import_type: ImportType::NamedImport("y1".to_string()),
                                    }),
                                    depend_on: None,
                                },
                            ),
                            (
                                "z1".to_string(),
                                Symbol {
                                    name: "z1".to_string(),
                                    is_named_exported: true,
                                    import_from: Some(Import {
                                        from: "Module Z".to_string(),
                                        import_type: ImportType::NamedImport("z1".to_string()),
                                    }),
                                    depend_on: None,
                                },
                            ),
                            // It's a NamespaceImport, so it will be expanded to z1, z2, z3.
                            // Then each symbol depends on z will then depend on z1, z2, z3 instead.
                            // Don't need to update those depend on z1, z2, z3 directly.
                            (
                                "z".to_string(),
                                Symbol {
                                    name: "z".to_string(),
                                    is_named_exported: false,
                                    import_from: Some(Import {
                                        from: "Module Z".to_string(),
                                        import_type: ImportType::NamespaceImport(vec![
                                            "B".to_string()
                                        ]),
                                    }),
                                    depend_on: None,
                                },
                            ),
                        ]),
                    },
                ),
            ]),
        };

        d.expand_namespace_import("Module A").unwrap();

        let module_a = d.parsed_modules.get("Module A").unwrap();
        assert!(
            !module_a.has_namespace_import,
            "Module A should have no more namespace imports"
        );
        assert!(
            module_a.symbols.get("z1").unwrap().is_named_exported,
            "z1 shouldn't be override during the expansion of namespace import of z"
        );
        assert!(
            module_a.symbols.contains_key("z2"),
            "z2 should be expanded during the expansion of namespace import of z"
        );
        assert!(
            !module_a.symbols.contains_key("z3"),
            "z3 is not exported by Module Z"
        );

        let module_a_symbol_b_depend_on = module_a
            .symbols
            .get("B")
            .as_ref()
            .unwrap()
            .depend_on
            .as_ref()
            .unwrap();
        assert!(
            module_a_symbol_b_depend_on.contains("C"),
            "The original dependency of Symbol B should not be affected"
        );
        assert!(
            !module_a_symbol_b_depend_on.contains("z"),
            "Symbol B shouldn't depend on the whole namespace z anymore"
        );
        assert!(
            module_a_symbol_b_depend_on.contains("z1"),
            "Symbol B should depend on the z1 directly"
        );
        assert!(
            module_a_symbol_b_depend_on.contains("z2"),
            "Symbol B should depend on the z2 directly"
        );

        let module_a_symbol_d_depend_on = module_a
            .symbols
            .get("D")
            .as_ref()
            .unwrap()
            .depend_on
            .as_ref()
            .unwrap();
        assert!(
            module_a_symbol_d_depend_on.contains("z1"),
            "Symbol D's dependency shouldn't be affected during the expansion of namespace import of z"
        );
        assert!(
            !module_a_symbol_d_depend_on.contains("z2"),
            "Symbol D's dependency shouldn't be affected during the expansion of namespace import of z"
        );
    }
}
