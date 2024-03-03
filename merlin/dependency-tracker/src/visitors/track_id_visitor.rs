use std::collections::HashSet;
use swc_core::{ecma::ast::*, ecma::visit::Visit};

pub struct TrackIdVisitor {
    pub tracked_ids: HashSet<Id>,
}

impl TrackIdVisitor {
    pub fn new() -> Self {
        Self {
            tracked_ids: HashSet::new(),
        }
    }

    fn track_id(&mut self, ident: &Ident) {
        let id = ident.to_id();
        assert_eq!(
            self.tracked_ids.contains(&id),
            false,
            "{} should not be tracked twice",
            id.0
        );
        self.tracked_ids.insert(id);
    }
}

impl Visit for TrackIdVisitor {
    fn visit_module(&mut self, n: &Module) {
        for module_item in &n.body {
            match module_item {
                ModuleItem::ModuleDecl(module_decl) => match module_decl {
                    ModuleDecl::Import(import_decl) => {
                        for specifier in &import_decl.specifiers {
                            match specifier {
                                // import { foo } from 'foo';
                                ImportSpecifier::Named(ImportNamedSpecifier { local, .. }) => {
                                    self.track_id(local);
                                }
                                // import foo from 'foo';
                                ImportSpecifier::Default(ImportDefaultSpecifier {
                                    local, ..
                                }) => {
                                    self.track_id(local);
                                }
                                // import * as foo from 'foo';
                                ImportSpecifier::Namespace(ImportStarAsSpecifier {
                                    local, ..
                                }) => {
                                    self.track_id(local);
                                }
                            }
                        }
                    }
                    ModuleDecl::ExportDecl(ExportDecl { decl, .. }) => match decl {
                        // export class Foo {}
                        Decl::Class(ClassDecl { ident, .. }) => {
                            self.track_id(ident);
                        }
                        // export function foo() {}
                        Decl::Fn(FnDecl { ident, .. }) => {
                            self.track_id(ident);
                        }
                        // export const foo = init
                        Decl::Var(var_decl) => {
                            let first_var_decl = var_decl.decls.get(0).unwrap();
                            match &first_var_decl.name {
                                Pat::Ident(BindingIdent { id, .. }) => {
                                    self.track_id(id);
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    },
                    ModuleDecl::ExportDefaultDecl(ExportDefaultDecl { decl, .. }) => {
                        match decl {
                            DefaultDecl::Class(ClassExpr { ident, .. }) => {
                                if let Some(ident) = ident {
                                    // export default class Foo {}
                                    // Foo could be used somewhere in this module
                                    self.track_id(ident);
                                }
                            }
                            DefaultDecl::Fn(FnExpr { ident, .. }) => {
                                if let Some(ident) = ident {
                                    // export default function foo() {}
                                    // foo could be used somewhere in this module
                                    self.track_id(ident);
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                },
                ModuleItem::Stmt(stmt) => match stmt {
                    Stmt::Decl(decl) => match decl {
                        // class Foo {}
                        Decl::Class(ClassDecl { ident, .. }) => {
                            self.track_id(ident);
                        }
                        // function foo() {}
                        Decl::Fn(FnDecl { ident, .. }) => {
                            self.track_id(ident);
                        }
                        // const foo = init, bar = init;
                        Decl::Var(var_decl) => {
                            for var_decl in &var_decl.decls {
                                match &var_decl.name {
                                    Pat::Ident(BindingIdent { id, .. }) => {
                                        self.track_id(id);
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                },
            }
        }
    }
}
