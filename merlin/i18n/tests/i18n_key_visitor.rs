use i18n::{i18n_key_visitor::I18nKeyVisitor, SingleTranslation};

use swc_core::{
    common::{
        errors::{ColorConfig, Handler},
        sync::Lrc,
        FileName, SourceMap,
    },
    ecma::visit::VisitWith,
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

        let mut i18n_key_visitor = I18nKeyVisitor::new();
        module.visit_with(&mut i18n_key_visitor);
        assert_eq!(i18n_key_visitor.contents, $expect);
    };
}

#[test]
fn test_empty_input() {
    test!("", []);
}

#[test]
fn test_nested_input() {
    test!(
        "
const LABELS = translate({
  title: 'abc.def.title',
  description: 'abc.def.description',
  payment: {
    method: 'abc.def.payment.method',
    price: ['abc.def.payment.formatted-price', 'lazy'],
  },
});

        ",
        [
            SingleTranslation::new("title", "abc.def.title"),
            SingleTranslation::new("description", "abc.def.description"),
            SingleTranslation::new("method", "abc.def.payment.method"),
            SingleTranslation::new("price", "abc.def.payment.formatted-price"),
        ]
    );
}

#[test]
fn test_should_ignore() {
    test!(
        "
const LABELS = 'foo';

const foo = translate({ foo: 'foo' });

function foo() {
  const LABELS = 'foo';

  const foo = translate({ foo: 'foo' });

  const LABELS = translate({
    title: 'abc.def.title',
    description: 'abc.def.description',
    payment: {
      method: 'abc.def.payment.method',
      price: ['abc.def.payment.formatted-price', 'lazy'],
    },
  });
}
        ",
        []
    );
}
