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
          let module = module.fold_with(&mut resolver(Mark::new(), Mark::new(), true));
          $(module.visit_with(&mut $visitor);)*
      });
    };

    ($input:expr, $($visitor:expr,)*) => {
      $crate::parse_with_visitors![$input:expr, $($visitor),*]
    };
}

macro_rules! assert_tracked_ids {
    ($visitor:expr, $expect:expr) => {
        let tracked_ids: HashSet<&str> = $visitor
            .tracked_ids
            .iter()
            .map(|(atom, _)| atom.as_str())
            .collect();
        assert_eq!(tracked_ids, HashSet::from($expect));
    };
}

#[test]
fn test_empty_input() {
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors!["", visitor];
    assert_tracked_ids!(visitor, []);
}

#[test]
fn test_import() {
    let input = "
import A, { B, _C as C } from 'some/where';
import * as D from 'some/where/else';
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, ["A", "B", "C", "D"]);
}

#[test]
fn test_export_declaration() {
    let input = "
export class A {}
export function B() {}
export const C = () => {};
export const D = [];
export const E = {};
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, ["A", "B", "C", "D", "E"]);
}

#[test]
fn test_export_named_declaration() {
    let input = "
export { A };
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, []);
}

#[test]
fn test_export_default_declaration_class() {
    let input = "
export default class A {}
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, ["A"]);
}

#[test]
fn test_export_default_declaration_function() {
    let input = "
export default function A() {}
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, ["A"]);
}

#[test]
fn test_export_default_declaration_class_no_ident() {
    let input = "
export default class {}
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, []);
}

#[test]
fn test_export_default_declaration_function_no_ident() {
    let input = "
export default function () {}
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, []);
}

#[test]
fn test_export_default_expression() {
    let input = "
export default [1, 2, 3];
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, []);
}

#[test]
fn test_export_all_declaration() {
    let input = "
export * from 'some/where';
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, []);
}

#[test]
fn test_statement() {
    let input = "
class A {}
function B() {}
const C = () => {};
const D = [];
const E = {};
";
    let mut visitor = TrackIdVisitor::new();
    parse_with_visitors![input, visitor];
    assert_tracked_ids!(visitor, ["A", "B", "C", "D", "E"]);
}
