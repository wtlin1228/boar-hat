use std::collections::{HashMap, HashSet};

use dependency_tracker::visitors::{
    dependency_visitor::DependencyVisitor, track_id_visitor::TrackIdVisitor,
};

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

macro_rules! test {
    ($input:expr, $expect:expr) => {
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

            let mut track_id_visitor = TrackIdVisitor::new();
            module.visit_with(&mut track_id_visitor);

            let mut dependency_visitor = DependencyVisitor::new(track_id_visitor.tracked_ids);
            module.visit_with(&mut dependency_visitor);

            let mut output: HashMap<&str, HashSet<&str>> = HashMap::new();
            for (key, value) in &dependency_visitor.dependency {
                let set: HashSet<&str> = value.iter().map(|(atom, _)| atom.as_str()).collect();
                output.insert(key.0.as_str(), set);
            }

            assert_eq!(output, HashMap::from($expect));
        });
    };
}

macro_rules! dependency {
    ($($key:expr, $value:expr),*) => {{
        $(($key, HashSet::from($value)))*
    }};
}

#[test]
fn test_empty_input() {
    test!("", []);
}

#[test]
fn test_export_declaration_class() {
    test!(
        "
import A, { B, _C as C } from 'some/where';
import * as D from 'some/where/else';

let bar = 'bar';

export class Foo {
  constructor() {
    this.a = A;
    this.b = B;
    this.c = C;
    this.d = D;
    this.bar = bar;
    this.foo = Foo;
  }
}

        ",
        [
            dependency!("A", []),
            dependency!("B", []),
            dependency!("C", []),
            dependency!("D", []),
            dependency!("bar", []),
            dependency!("Foo", ["A", "B", "C", "D", "bar"]),
        ]
    );
}

#[test]
fn test_export_declaration_class_arguments() {
    test!(
        "
import A, { B, _C as C } from 'some/where';
import * as D from 'some/where/else';

let bar = 'bar';

export class Foo {
  constructor(A, B, C, D, bar, Foo) {}
}

        ",
        [
            dependency!("A", []),
            dependency!("B", []),
            dependency!("C", []),
            dependency!("D", []),
            dependency!("bar", []),
            dependency!("Foo", []),
        ]
    );
}

#[test]
fn test_export_declaration_class_shadowed() {
    test!(
        "
import A, { B, _C as C } from 'some/where';
import * as D from 'some/where/else';

let bar = 'bar';

export class Foo {
  constructor(A) {
    const B = '';
    this.a = A;
    this.b = B;
    this.c = C;
    this.d = D;
    this.bar = bar;
    this.foo = Foo;
  }
}

        ",
        [
            dependency!("A", []),
            dependency!("B", []),
            dependency!("C", []),
            dependency!("D", []),
            dependency!("bar", []),
            dependency!("Foo", ["C", "D", "bar"]),
        ]
    );
}

#[test]
fn test_complex_case() {
    test!(
        "
import A, { B, _C as C } from 'some/where';
import * as D from 'some/where/else';

let E = {
  A,
  B,
  C: D.x.y.z,
  F,
};

export class F {
  constructor() {
    const B = '';
    this.a = A;
    this.b = B;
    this.c = C;
    this.d = D;
    this.bar = bar;
    this.foo = Foo;
  }
}

function G() {
  const D = {
    a: { b: B },
  };

  const E = () => {
    let A = 'a' + E.F;
    return {
      a: A,
    };
  };

  return [B, D, E];
}

export const H = [E.C];

const I = () => {
  return (
    <D.Page>
      <D.Header>
        <div>
          <C
            a={A}
            b={B}
            others={{
              E: { e: 'E' },
              F: {
                f: (F) => <div>{F.abcd}</div>,
              },
            }}
          />
        </div>
      </D.Header>
    </D.Page>
  );
};

export default () => ({
  A,
  B,
  C,
});

        ",
        [
            dependency!("A", []),
            dependency!("B", []),
            dependency!("C", []),
            dependency!("D", []),
            dependency!("E", ["A", "B", "D", "F"]),
            dependency!("F", ["A", "C", "D"]),
            dependency!("G", ["B"]),
            dependency!("H", ["E"]),
            dependency!("I", ["A", "B", "C", "D"]),
        ]
    );
}
