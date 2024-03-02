use dependency_tracker::TopVisitor;
use std::path::Path;

use swc_core::{
    common::{
        errors::{ColorConfig, Handler},
        sync::Lrc,
        Globals, Mark, SourceMap, GLOBALS,
    },
    ecma::{
        transforms::base::resolver,
        visit::{FoldWith, VisitWith},
    },
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm
        .load_file(Path::new("./test-inputs/foo.jsx"))
        .expect("failed to load ./test-inputs/foo.jsx");

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

    let module = parser
        .parse_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parser module");

    GLOBALS.set(&Globals::new(), || {
        module
            .fold_with(&mut resolver(Mark::new(), Mark::new(), true))
            .visit_with(&mut TopVisitor {});
    });
}
