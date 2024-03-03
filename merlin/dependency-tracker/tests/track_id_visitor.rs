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

use dependency_tracker::visitors::track_id_visitor::TrackIdVisitor;

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
          .expect("failed to parser module");

      GLOBALS.set(&Globals::new(), || {
          module
              .fold_with(&mut resolver(Mark::new(), Mark::new(), true))
              $(.visit_with(&mut $visitor))*
              ;
      });
    };

    ($input:expr, $($visitor:expr,)*) => {
      $crate::parse_with_visitors![$input:expr, $($visitor),*]
    };

}

#[test]
fn empty_input() {
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors!["", visitor];
    assert_eq!(visitor.tracked_ids.len(), 0);
}
