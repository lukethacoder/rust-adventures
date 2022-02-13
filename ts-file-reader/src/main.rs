use std::path::Path;
use colored::*;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false,
        Some(cm.clone()));

    // Real usage
    let fm = cm
        .load_file(Path::new("test.ts"))
        .expect("failed to load test.ts");

    // Inline string usage
    // let fm = cm.new_source_file(
    //     FileName::Custom("test.ts".into()),
    //     "function foo() {}".into(),
    // );

    let lexer = Lexer::new(
        // We want to parse typescript
        Syntax::Typescript(Default::default()),
        // EsVersion defaults to es5
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let _module = parser
        .parse_typescript_module()
        .map_err(|e| {
            // Unrecoverable fatal error occurred
            e.into_diagnostic(&handler).emit()
        })
        .expect("failed to parser module");

    println!("{} {:#?}", "Typescript as AST:".green(), _module)
}