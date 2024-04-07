pub mod parser_candidate_scheduler;
pub mod visitors;

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

pub type ModulePath = String;
pub type SymbolName = String;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct ModuleSymbol {
    module_path: String,
    symbol_name: String,
}

impl std::fmt::Debug for ModuleSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.module_path, self.symbol_name))
    }
}

impl ModuleSymbol {
    pub fn new(m: &str, s: &str) -> Self {
        Self {
            module_path: m.to_string(),
            symbol_name: s.to_string(),
        }
    }
}

const MODULE_DEFAULT_EXPORT: &'static str = "MODULE_DEFAULT_EXPORT";

#[derive(Debug)]
pub struct DependencyTracker {
    root: PathBuf,

    // This table will only be used when we encounter those cases when we're building
    // the `module_symbols_table`
    //     - `import * as Foo from 'somewhere'`
    //     - `export * from 'elsewhere'`
    module_exports_table: HashMap<ModulePath, Vec<SymbolName>>,

    module_symbol_depends_on_table: HashMap<ModulePath, HashMap<SymbolName, HashSet<ModuleSymbol>>>,

    // This table stores the reversed pointers from `module_symbol_depends_on_table`.
    // It can be built by calling `Self::build_depended_by_table(&Self)`.
    module_symbol_depended_by_table:
        HashMap<ModulePath, HashMap<SymbolName, HashSet<ModuleSymbol>>>,
}

impl DependencyTracker {
    pub fn debug_depended_by(&self, ms: &ModuleSymbol) -> Vec<Vec<ModuleSymbol>> {
        let depended_by = &self
            .module_symbol_depended_by_table
            .get(&ms.module_path)
            .unwrap()
            .get(&ms.symbol_name)
            .unwrap();

        let mut res = vec![];
        for depended_by_ms in depended_by.iter() {
            for mut r in self.debug_depended_by(depended_by_ms).into_iter() {
                r.push(ms.clone());
                res.push(r);
            }
        }

        res.push(vec![ms.clone()]);
        res
    }

    pub fn build_depended_by_table(&mut self) {
        for (module_path, symbol_table) in self.module_symbol_depends_on_table.iter() {
            for (symbol_name, dependency) in symbol_table.iter() {
                for ms in dependency.iter() {
                    self.module_symbol_depended_by_table
                        .get_mut(&ms.module_path)
                        .unwrap()
                        .get_mut(&ms.symbol_name)
                        .unwrap()
                        .insert(ModuleSymbol::new(&module_path, &symbol_name));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! ms {
        ($x:expr) => {{
            let mut x = $x.split(".");
            ModuleSymbol::new(x.next().unwrap(), x.next().unwrap())
        }};
    }

    //
    //
    //  A  ┌────a1────────────►a2
    //     │     │              │
    //     │     ├──────┐┌──────┤
    //     │     │      ││      │
    //     │     ▼      ▼▼      ▼
    //  B  │    b1      b2     b3
    //     │             │      │
    //     │             │      │
    //     │             │      │
    //  C  └───►c1◄──────┴──────┘
    //
    //
    #[test]
    fn test_build_depended_by_table() {
        let mut dt = DependencyTracker {
            root: Path::new("some/where").into(),
            module_exports_table: HashMap::new(),
            module_symbol_depends_on_table: HashMap::from([
                (
                    "A".into(),
                    HashMap::from([
                        (
                            "a1".into(),
                            HashSet::from([ms!("A.a2"), ms!("B.b1"), ms!("B.b2"), ms!("C.c1")]),
                        ),
                        ("a2".into(), HashSet::from([ms!("B.b2"), ms!("B.b3")])),
                    ]),
                ),
                (
                    "B".into(),
                    HashMap::from([
                        ("b1".into(), HashSet::new()),
                        ("b2".into(), HashSet::from([ms!("C.c1")])),
                        ("b3".into(), HashSet::from([ms!("C.c1")])),
                    ]),
                ),
                ("C".into(), HashMap::from([("c1".into(), HashSet::new())])),
            ]),
            module_symbol_depended_by_table: HashMap::from([
                (
                    "A".into(),
                    HashMap::from([("a1".into(), HashSet::new()), ("a2".into(), HashSet::new())]),
                ),
                (
                    "B".into(),
                    HashMap::from([
                        ("b1".into(), HashSet::new()),
                        ("b2".into(), HashSet::new()),
                        ("b3".into(), HashSet::new()),
                    ]),
                ),
                ("C".into(), HashMap::from([("c1".into(), HashSet::new())])),
            ]),
        };

        dt.build_depended_by_table();
        let graph = dt.debug_depended_by(&ms!("C.c1"));
        let expect_graph = vec![
            vec![ms!("A.a1"), ms!("A.a2"), ms!("B.b2"), ms!("C.c1")],
            vec![ms!("A.a2"), ms!("B.b2"), ms!("C.c1")],
            vec![ms!("A.a1"), ms!("B.b2"), ms!("C.c1")],
            vec![ms!("B.b2"), ms!("C.c1")],
            vec![ms!("A.a1"), ms!("A.a2"), ms!("B.b3"), ms!("C.c1")],
            vec![ms!("A.a2"), ms!("B.b3"), ms!("C.c1")],
            vec![ms!("B.b3"), ms!("C.c1")],
            vec![ms!("A.a1"), ms!("C.c1")],
            vec![ms!("C.c1")],
        ];

        assert_eq!(graph.len(), expect_graph.len());
        expect_graph.iter().for_each(|p| assert!(graph.contains(p)))
    }

    #[test]
    fn test_depends_on_module_default_export() {
        let mut dt = DependencyTracker {
            root: Path::new("some/where").into(),
            module_exports_table: HashMap::new(),
            module_symbol_depends_on_table: HashMap::from([
                (
                    "App.tsx".into(),
                    HashMap::from([(
                        MODULE_DEFAULT_EXPORT.into(),
                        HashSet::from([
                            ModuleSymbol::new("components/Header", MODULE_DEFAULT_EXPORT),
                            ModuleSymbol::new("components/Main", MODULE_DEFAULT_EXPORT),
                            ModuleSymbol::new("components/Footer", MODULE_DEFAULT_EXPORT),
                        ]),
                    )]),
                ),
                (
                    "components/Header".into(),
                    HashMap::from([(MODULE_DEFAULT_EXPORT.into(), HashSet::new())]),
                ),
                (
                    "components/Main".into(),
                    HashMap::from([(MODULE_DEFAULT_EXPORT.into(), HashSet::new())]),
                ),
                (
                    "components/Footer".into(),
                    HashMap::from([(MODULE_DEFAULT_EXPORT.into(), HashSet::new())]),
                ),
            ]),
            module_symbol_depended_by_table: HashMap::from([
                (
                    "App.tsx".into(),
                    HashMap::from([(MODULE_DEFAULT_EXPORT.into(), HashSet::new())]),
                ),
                (
                    "components/Header".into(),
                    HashMap::from([(MODULE_DEFAULT_EXPORT.into(), HashSet::new())]),
                ),
                (
                    "components/Main".into(),
                    HashMap::from([(MODULE_DEFAULT_EXPORT.into(), HashSet::new())]),
                ),
                (
                    "components/Footer".into(),
                    HashMap::from([(MODULE_DEFAULT_EXPORT.into(), HashSet::new())]),
                ),
            ]),
        };

        dt.build_depended_by_table();
        let graph = dt.debug_depended_by(&ModuleSymbol::new(
            "components/Footer",
            MODULE_DEFAULT_EXPORT,
        ));
        let expect_graph = vec![
            vec![ModuleSymbol::new(
                "components/Footer",
                MODULE_DEFAULT_EXPORT,
            )],
            vec![
                ModuleSymbol::new("App.tsx", MODULE_DEFAULT_EXPORT),
                ModuleSymbol::new("components/Footer", MODULE_DEFAULT_EXPORT),
            ],
        ];

        assert_eq!(graph.len(), expect_graph.len());
        expect_graph.iter().for_each(|p| assert!(graph.contains(p)));
    }
}
