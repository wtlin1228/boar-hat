use super::module::{Import, ImportType, Symbol, DEFAULT_EXPORT};

use std::collections::{HashMap, HashSet};
use swc_core::ecma::{ast, visit::Visit};

#[derive(Debug)]
pub struct ModuleSymbolsVisitor {
    pub has_namespace_import: bool,
    pub symbols: HashMap<String, Symbol>,

    // Can't just use `Symbol.name` to build the dependency graph.
    // We need the `SyntaxContext` in `Id.1` when looking into each symbol declaration.
    //
    // ```js
    // let a#1 = 5
    // {
    //     let a#2 = 3;
    // }
    // ```
    //
    // ref: https://rustdoc.swc.rs/swc_core/ecma/ast/struct.Ident.html
    pub tracked_ids: HashSet<ast::Id>,

    // In order to trace one level more for namespaces later
    pub namespace_ids: HashSet<ast::Id>,
}

impl ModuleSymbolsVisitor {
    pub fn new() -> Self {
        Self {
            has_namespace_import: false,
            symbols: HashMap::new(),
            tracked_ids: HashSet::new(),
            namespace_ids: HashSet::new(),
        }
    }

    fn track_id(&mut self, ident: &ast::Ident, is_namespace_id: bool) {
        let id = ident.to_id();
        assert!(
            !self.tracked_ids.contains(&id),
            "It's impossible to track the same id {} twice. There is high possibility that your JS/TS program has bug.",
            id.0
        );
        self.tracked_ids.insert(id);
        if is_namespace_id {
            self.namespace_ids.insert(ident.to_id());
        }
    }

    fn add_symbol_dependency(&mut self, symbol_name: &str, depend_on_name: &str) {
        assert!(self.symbols.contains_key(symbol_name));
        let symbol = self.symbols.get_mut(symbol_name).unwrap();
        match symbol.depend_on {
            Some(ref mut depend_on) => {
                depend_on.insert(depend_on_name.to_string());
            }
            None => symbol.depend_on = Some(HashSet::from([depend_on_name.to_string()])),
        }
    }

    fn add_symbol(&mut self, name: &str) {
        assert!(!self.symbols.contains_key(name));
        self.symbols.insert(
            name.to_string(),
            Symbol {
                name: name.to_string(),
                is_named_exported: false,
                import_from: None,
                depend_on: None,
            },
        );
    }

    fn add_named_exported_symbol(&mut self, name: &str) {
        assert_ne!(
            name, DEFAULT_EXPORT,
            "Please use `add_symbol()`. The default export is special, it's is_named_exported must be false."
        );
        match self.symbols.get_mut(name) {
            Some(symbol) => {
                symbol.is_named_exported = true;
            }
            None => {
                self.symbols.insert(
                    name.to_string(),
                    Symbol {
                        name: name.to_string(),
                        is_named_exported: true,
                        import_from: None,
                        depend_on: None,
                    },
                );
            }
        };
    }

    fn add_default_import_symbol(&mut self, name: &str, src: &str) {
        assert!(!self.symbols.contains_key(name));
        self.symbols.insert(
            name.to_string(),
            Symbol {
                name: name.to_string(),
                is_named_exported: true,
                import_from: Some(Import {
                    from: src.to_string(),
                    import_type: ImportType::DefaultImport,
                }),
                depend_on: None,
            },
        );
    }

    fn add_named_import_symbol(&mut self, name: &str, src: &str) {
        assert!(!self.symbols.contains_key(name));
        self.symbols.insert(
            name.to_string(),
            Symbol {
                name: name.to_string(),
                is_named_exported: true,
                import_from: Some(Import {
                    from: src.to_string(),
                    import_type: ImportType::NamedImport(name.to_string()),
                }),
                depend_on: None,
            },
        );
    }

    fn add_namespace_import_symbol(&mut self, name: &str, src: &str) {
        assert!(!self.symbols.contains_key(name));
        self.has_namespace_import = true;
        self.symbols.insert(
            name.to_string(),
            Symbol {
                name: name.to_string(),
                is_named_exported: true,
                import_from: Some(Import {
                    from: src.to_string(),
                    import_type: ImportType::NamespaceImport(vec![]),
                }),
                depend_on: None,
            },
        );
    }
}

impl Visit for ModuleSymbolsVisitor {
    fn visit_module(&mut self, n: &ast::Module) {
        for module_item in &n.body {
            match module_item {
                ast::ModuleItem::ModuleDecl(module_decl) => {
                    match module_decl {
                        ast::ModuleDecl::Import(import_decl) => {
                            for specifier in &import_decl.specifiers {
                                match specifier {
                                    // import { A } from 'module-a';
                                    ast::ImportSpecifier::Named(ast::ImportNamedSpecifier {
                                        local,
                                        ..
                                    }) => {
                                        self.track_id(local, false);
                                        self.add_named_import_symbol(
                                            local.to_id().0.as_str(),
                                            import_decl.src.value.as_str(),
                                        );
                                    }
                                    // import A from 'module-a';
                                    ast::ImportSpecifier::Default(
                                        ast::ImportDefaultSpecifier { local, .. },
                                    ) => {
                                        self.track_id(local, false);
                                        self.add_default_import_symbol(
                                            local.to_id().0.as_str(),
                                            import_decl.src.value.as_str(),
                                        )
                                    }
                                    // import * as A from 'module-a';
                                    ast::ImportSpecifier::Namespace(
                                        ast::ImportStarAsSpecifier { local, .. },
                                    ) => {
                                        self.track_id(local, true);
                                        self.add_namespace_import_symbol(
                                            local.to_id().0.as_str(),
                                            import_decl.src.value.as_str(),
                                        )
                                    }
                                }
                            }
                        }
                        ast::ModuleDecl::ExportDecl(ast::ExportDecl { decl, .. }) => match decl {
                            // export class A {}
                            ast::Decl::Class(ast::ClassDecl { ident, .. }) => {
                                self.track_id(ident, false);
                                self.add_named_exported_symbol(ident.to_id().0.as_str());
                            }
                            // export function A() {}
                            ast::Decl::Fn(ast::FnDecl { ident, .. }) => {
                                self.track_id(ident, false);
                                self.add_named_exported_symbol(ident.to_id().0.as_str());
                            }
                            // export const A = init
                            // export const A = () => {}
                            ast::Decl::Var(var_decl) => {
                                let first_var_decl = var_decl.decls.get(0).unwrap();
                                match &first_var_decl.name {
                                    ast::Pat::Ident(ast::BindingIdent { id, .. }) => {
                                        self.track_id(id, false);
                                        self.add_named_exported_symbol(id.to_id().0.as_str());
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        },
                        ast::ModuleDecl::ExportNamed(ast::NamedExport { specifiers, .. }) => {
                            for specifier in specifiers {
                                match specifier {
                                    ast::ExportSpecifier::Namespace(_) => todo!(),
                                    ast::ExportSpecifier::Default(_) => todo!(),
                                    ast::ExportSpecifier::Named(ast::ExportNamedSpecifier {
                                        orig,
                                        exported,
                                        ..
                                    }) => match orig {
                                        ast::ModuleExportName::Ident(ident) => match exported {
                                            Some(module_exported_name) => {
                                                match module_exported_name {
                                                    ast::ModuleExportName::Ident(
                                                        exported_ident,
                                                    ) => match exported_ident.to_id().0.as_str() {
                                                        // export { A as Default }
                                                        "Default" => {
                                                            self.add_symbol(DEFAULT_EXPORT);
                                                            self.add_symbol_dependency(
                                                                DEFAULT_EXPORT,
                                                                ident.to_id().0.as_str(),
                                                            )
                                                        }
                                                        // export { A as B }
                                                        _ => {
                                                            self.add_named_exported_symbol(
                                                                exported_ident.to_id().0.as_str(),
                                                            );
                                                            self.add_symbol_dependency(
                                                                exported_ident.to_id().0.as_str(),
                                                                ident.to_id().0.as_str(),
                                                            )
                                                        }
                                                    },
                                                    _ => (),
                                                }
                                            }
                                            // export { A }
                                            None => {
                                                self.add_named_exported_symbol(
                                                    ident.to_id().0.as_str(),
                                                );
                                            }
                                        },
                                        _ => (),
                                    },
                                }
                            }
                        }
                        ast::ModuleDecl::ExportDefaultDecl(ast::ExportDefaultDecl {
                            decl, ..
                        }) => {
                            match decl {
                                ast::DefaultDecl::Class(ast::ClassExpr { ident, .. }) => {
                                    if let Some(ident) = ident {
                                        // export default class A {}
                                        // We need to track the class A since it could be used inside the same module.
                                        // In this case we set the symbol "DEFAULT_EXPORT" to depend on symbol "A",
                                        // so if other symbol depends on "A", they can follow "A"'s dependency.
                                        self.track_id(ident, false);
                                        self.add_symbol(ident.to_id().0.as_str());
                                        self.add_symbol(DEFAULT_EXPORT);
                                        self.add_symbol_dependency(
                                            DEFAULT_EXPORT,
                                            ident.to_id().0.as_str(),
                                        );
                                    } else {
                                        // export default class {}
                                        // We don't need to track the anonymous class since it won't be used inside the same module.
                                        // But in this case we need to put the dependency directly on the symbol "DEFAULT_EXPORT",
                                        // And this will need us to pay additional attention to handle it properly later.
                                        self.add_symbol(DEFAULT_EXPORT);
                                    }
                                }
                                ast::DefaultDecl::Fn(ast::FnExpr { ident, .. }) => {
                                    if let Some(ident) = ident {
                                        // export default function A() {}
                                        // We need to track the function A since it could be used inside the same module.
                                        // In this case we set the symbol "DEFAULT_EXPORT" to depend on symbol "A",
                                        // so if other symbol depends on "A", they can follow "A"'s dependency.
                                        self.track_id(ident, false);
                                        self.add_symbol(ident.to_id().0.as_str());
                                        self.add_symbol(DEFAULT_EXPORT);
                                        self.add_symbol_dependency(
                                            DEFAULT_EXPORT,
                                            ident.to_id().0.as_str(),
                                        );
                                    } else {
                                        // export default function() {}
                                        // We don't need to track the anonymous function since it won't be used inside the same module.
                                        // But in this case we need to put the dependency directly on the symbol "DEFAULT_EXPORT",
                                        // And this will need us to pay additional attention to handle it properly later.
                                        self.add_symbol(DEFAULT_EXPORT);
                                    }
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
                ast::ModuleItem::Stmt(_) => todo!(),
            }
        }
    }
}

#[cfg(test)]
macro_rules! parse_with_visitors {
    ($input:expr, $($visitor:expr),*) => {
        let cm: Lrc<SourceMap> = Default::default();
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

        let fm = cm.new_source_file(FileName::Custom("test.js".into()), $input.into());

        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: false,
                dts: false,
                no_early_errors: true,
                disallow_ambiguous_jsx_like: true,
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        for e in parser.take_errors() {
            e.into_diagnostic(&handler).emit();
        }

        let module = parser
            .parse_module()
            .map_err(|e| {
                // Unrecoverable fatal error occurred
                e.into_diagnostic(&handler).emit()
            })
            .expect("failed to parse module");

        GLOBALS.set(&Globals::new(), || {
            let module = module.fold_with(&mut resolver(Mark::new(), Mark::new(), true));
            $(module.visit_with(&mut $visitor);)*
        });
    };

    ($input:expr, $($visitor:expr,)*) => {
        $crate::parse_with_visitors![$input:expr, $($visitor),*]
    };
}

#[cfg(test)]
macro_rules! assert_ast_ids {
    ($ids:expr, $expect:expr) => {
        let tracked_ids: HashSet<&str> = $ids.iter().map(|(atom, _)| atom.as_str()).collect();
        assert_eq!(tracked_ids, HashSet::from($expect));
    };
}

#[cfg(test)]
mod tests {
    use super::{Import, ImportType, ModuleSymbolsVisitor, Symbol, DEFAULT_EXPORT};
    use std::collections::HashSet;
    use swc_core::{
        common::{
            errors::{ColorConfig, Handler},
            sync::Lrc,
            FileName, Globals, Mark, SourceMap, GLOBALS,
        },
        ecma::{
            transforms::base::resolver,
            visit::{FoldWith, VisitWith},
        },
    };
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    #[test]
    fn test_default_import() {
        let input = "
            import A from 'module-a';
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: true,
                import_from: Some(Import {
                    from: "module-a".to_string(),
                    import_type: ImportType::DefaultImport,
                }),
                depend_on: None
            }
        );
    }

    #[test]
    fn test_named_import() {
        let input = "
            import { A } from 'module-a';
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: true,
                import_from: Some(Import {
                    from: "module-a".to_string(),
                    import_type: ImportType::NamedImport("A".to_string()),
                }),
                depend_on: None
            }
        );
    }

    #[test]
    fn test_namespace_import() {
        let input = "
            import * as A from 'module-a';
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, ["A"]);
        assert!(visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: true,
                import_from: Some(Import {
                    from: "module-a".to_string(),
                    import_type: ImportType::NamespaceImport(vec![]),
                }),
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_class_declaration() {
        let input = "
            export class A {}
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: true,
                import_from: None,
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_function_declaration() {
        let input = "
            export function A() {}
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: true,
                import_from: None,
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_variable_declaration() {
        let input = "
            export const A = init;
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: true,
                import_from: None,
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_variable_declaration_2() {
        let input = "
            export const A = () => {}
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: true,
                import_from: None,
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_named() {
        let input = "
            export { A }
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, []);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: true,
                import_from: None,
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_named_as() {
        let input = "
            export { A as B }
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, []);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("B").unwrap(),
            &Symbol {
                name: "B".to_string(),
                is_named_exported: true,
                import_from: None,
                depend_on: Some(HashSet::from(["A".to_string()]))
            }
        );
    }

    #[test]
    fn test_export_named_as_another_name() {
        let input = "
            export { A as B }
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, []);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get("B").unwrap(),
            &Symbol {
                name: "B".to_string(),
                is_named_exported: true,
                import_from: None,
                depend_on: Some(HashSet::from(["A".to_string()]))
            }
        );
    }

    #[test]
    fn test_export_named_as_default() {
        let input = "
            export { A as Default }
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, []);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get(DEFAULT_EXPORT).unwrap(),
            &Symbol {
                name: DEFAULT_EXPORT.to_string(),
                is_named_exported: false,
                import_from: None,
                depend_on: Some(HashSet::from(["A".to_string()]))
            }
        );
    }

    #[test]
    fn test_export_default_class_declaration() {
        let input = "
            export default class A {}
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get(DEFAULT_EXPORT).unwrap(),
            &Symbol {
                name: DEFAULT_EXPORT.to_string(),
                is_named_exported: false,
                import_from: None,
                depend_on: Some(HashSet::from(["A".to_string()]))
            }
        );
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: false,
                import_from: None,
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_default_class_declaration_anonymous() {
        let input = "
            export default class {}
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, []);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get(DEFAULT_EXPORT).unwrap(),
            &Symbol {
                name: DEFAULT_EXPORT.to_string(),
                is_named_exported: false,
                import_from: None,
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_default_function_declaration() {
        let input = "
            export default function A() {}
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, ["A"]);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get(DEFAULT_EXPORT).unwrap(),
            &Symbol {
                name: DEFAULT_EXPORT.to_string(),
                is_named_exported: false,
                import_from: None,
                depend_on: Some(HashSet::from(["A".to_string()]))
            }
        );
        assert_eq!(
            visitor.symbols.get("A").unwrap(),
            &Symbol {
                name: "A".to_string(),
                is_named_exported: false,
                import_from: None,
                depend_on: None
            }
        );
    }

    #[test]
    fn test_export_default_function_declaration_anonymous() {
        let input = "
            export default function() {}
        ";
        let mut visitor = ModuleSymbolsVisitor::new();
        parse_with_visitors![input, visitor];
        assert_ast_ids!(visitor.tracked_ids, []);
        assert_ast_ids!(visitor.namespace_ids, []);
        assert!(!visitor.has_namespace_import);
        assert_eq!(
            visitor.symbols.get(DEFAULT_EXPORT).unwrap(),
            &Symbol {
                name: DEFAULT_EXPORT.to_string(),
                is_named_exported: false,
                import_from: None,
                depend_on: None
            }
        );
    }
}
