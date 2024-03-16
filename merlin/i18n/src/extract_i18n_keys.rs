use crate::{i18n_key_visitor::I18nKeyVisitor, SingleTranslation};

use std::path::Path;
use swc_core::{
    common::{
        errors::{ColorConfig, Handler},
        sync::Lrc,
        SourceMap,
    },
    ecma::visit::VisitWith,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

pub fn extract_i18n_keys(filepath: &str) -> Vec<SingleTranslation> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm
        .load_file(Path::new(filepath))
        .expect(&format!("failed to load {}", filepath));

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
        .expect("failed to parser module");

    let mut i18n_key_visitor = I18nKeyVisitor::new();
    module.visit_with(&mut i18n_key_visitor);
    i18n_key_visitor.contents
}
