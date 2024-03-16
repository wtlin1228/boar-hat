//! I18nKeyVisitor filters out the i18n keys defined in the first argument
//! of the following form only. If your i18n keys are applied in different
//! ways, please implement your own.
//!
//! const LABELS = translation({ /*...*/ });
//!
//! The filtered out translations can be accessed by I18nKeyVisitor.contents

use swc_core::ecma::{
    ast::*,
    visit::{Visit, VisitWith},
};

use crate::SingleTranslation;

/// I18nKeyVisitor filters out `const LABELS = translate({ /*...*/ })`
pub struct I18nKeyVisitor {
    pub contents: Vec<SingleTranslation>,
}

impl I18nKeyVisitor {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

impl Visit for I18nKeyVisitor {
    /// It filters out the statement `const LABELS = ...`.
    /// Then keep visiting the children of this variable declarator.
    fn visit_stmt(&mut self, n: &Stmt) {
        match n {
            Stmt::Decl(decl) => match decl {
                Decl::Var(var) if var.kind == VarDeclKind::Const => match var.decls.get(0) {
                    Some(VarDeclarator {
                        name:
                            Pat::Ident(BindingIdent {
                                id: Ident { sym, .. },
                                ..
                            }),
                        ..
                    }) if sym == "LABELS" => {
                        var.visit_children_with(self);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    /// It filters out the call expression `translate({ /*...*/ })`.
    /// Then keep visiting the children of its first argument with `TranslateArgumentVisitor`.
    fn visit_call_expr(&mut self, n: &CallExpr) {
        match &n.callee {
            Callee::Expr(expr) => match &**expr {
                Expr::Ident(Ident { sym, .. }) if sym == "translate" => match n.args.get(0) {
                    Some(arg) => {
                        let mut visitor = TranslateArgumentVisitor {
                            contents: &mut self.contents,
                        };
                        arg.visit_children_with(&mut visitor);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }
}

struct TranslateArgumentVisitor<'contents> {
    contents: &'contents mut Vec<SingleTranslation>,
}

impl<'contents> TranslateArgumentVisitor<'contents> {
    fn add_translate(&mut self, key: &str, value: &str) {
        self.contents.push(SingleTranslation::new(key, value));
    }
}

impl<'contents> Visit for TranslateArgumentVisitor<'contents> {
    fn visit_key_value_prop(&mut self, n: &KeyValueProp) {
        let key = n.key.as_ident().unwrap().sym.as_str();

        // extract value
        match &*n.value {
            // "bar.bar.bar.bar",
            Expr::Lit(expr) => match expr {
                Lit::Str(s) => {
                    self.add_translate(key, s.value.as_str());
                }
                _ => (),
            },
            // ["bar.bar.bar.bar", "lazy"],
            Expr::Array(expr) if expr.elems.len() == 2 => match &expr.elems[0] {
                Some(expr_or_spread) => match &*expr_or_spread.expr {
                    Expr::Lit(expr) => match expr {
                        Lit::Str(s) => {
                            self.add_translate(key, s.value.as_str());
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }

        n.visit_children_with(self);
    }
}
