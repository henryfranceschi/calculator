use calculator::{
    codegen,
    diagnostics::report_error,
    lexer::{span::Span, Lexer},
    parser::Parser,
    vm::Vm,
};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <expression>", env!("CARGO_BIN_NAME"));
        return;
    }

    let source = args[1].as_str();
    let mut parser = Parser::new(Lexer::new(source));
    match parser.parse() {
        Ok(expr) => {
            let mut vm = Vm::new(codegen::generate(&expr));
            match vm.run() {
                Ok(value) => println!("{}", value),
                Err(err) => eprintln!("runtime error: {:?}", err),
            }
        }
        Err(err) => {
            report_error(err.message(), err.span(), source);
        }
    }
}
