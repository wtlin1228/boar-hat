use super::{
    dependency_visitor::SymbolDependencyVisitor, module::Module,
    symbol_visitor::ModuleSymbolsVisitor,
};

use anyhow::{self};
use std::collections::HashMap;
use swc_core::{
    common::{
        errors::{ColorConfig, Handler},
        sync::Lrc,
        Globals, Mark, SourceFile, SourceMap, GLOBALS,
    },
    ecma::{
        transforms::base::resolver,
        visit::{FoldWith, VisitWith},
    },
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

pub fn parse_module(
    module_src: &str,
    cm: Lrc<SourceMap>,
    fm: Lrc<SourceFile>,
) -> anyhow::Result<Module> {
    let handler: Handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: false,
            dts: false,
            no_early_errors: true,
            disallow_ambiguous_jsx_like: true,
        }),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let ast_module: swc_core::ecma::ast::Module = parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parser module");

    let mut parsed_module = Module {
        has_namespace_import: false,
        symbols: HashMap::new(),
    };

    GLOBALS.set(&Globals::new(), || {
        // ref: https://rustdoc.swc.rs/swc_ecma_transforms_base/fn.resolver.html
        let ast_module: swc_core::ecma::ast::Module =
            ast_module.fold_with(&mut resolver(Mark::new(), Mark::new(), true));

        let mut symbol_visitor = ModuleSymbolsVisitor::new();
        ast_module.visit_with(&mut symbol_visitor);

        println!("{:#?}", symbol_visitor);

        let mut dependency_visitor =
            SymbolDependencyVisitor::new(symbol_visitor.namespace_ids, symbol_visitor.tracked_ids);
        ast_module.visit_with(&mut dependency_visitor);

        // println!("{:#?}", dependency_visitor);
        for (id, dependency) in dependency_visitor.dependency {
            println!(
                "{} depends on {:?}",
                id.0.as_str(),
                dependency
                    .iter()
                    .map(|x| x.0.as_str())
                    .collect::<Vec<&str>>()
            )
        }

        parsed_module.has_namespace_import = symbol_visitor.has_namespace_import;
    });

    Ok(parsed_module)
}

#[cfg(test)]
mod tests {
    use swc_core::common::FileName;

    use super::*;

    #[test]
    fn test_parse_module() {
        let cm: Lrc<SourceMap> = Default::default();
        let fm: Lrc<SourceFile> = cm.new_source_file(
            FileName::Custom("test.js".into()),
            "
                import Header from 'header';
                import * as UI from 'my-ui-lib'; // UI.Header, UI.Body, UI.Footer

                export Body = '';
                
                export const a = [
                    UI,        // before expansion: `Symbol UI`
                               // after expansion:  `Symbol UI.Header`, `Symbol UI.Body`, `Symbol UI.Footer` 
                    Header,    // `Symbol Header`
                    Body       // `Symbol Body`
                ];
                export const b = [
                    UI.Header, // before expansion: `Symbol UI.Header`
                               // after expansion:  `Symbol UI.Header`
                    Header,    // `Symbol Header`   
                    Body       // `Symbol Body`
                ];
                export const c = [
                    UI.Header.getTitle(), // before expansion: `Symbol UI.Header`
                                          // after expansion:  `Symbol UI.Header`
                ];
                export const d = [
                    UI.Header(),          // before expansion: `Symbol UI.Header`
                                          // after expansion:  `Symbol UI.Header`
                ];
            "
            .into(),
        );
        let module: Module = parse_module("test.js", cm, fm).unwrap();

        println!("{:#?}", module);
    }
}
