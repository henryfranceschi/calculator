use calculator::{
    codegen,
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

fn report_error(message: &str, span: Span, source: &str) {
    let line = span.starting_line_number(source);
    let column = span.starting_column_number(source);

    // Lines will not yield empty lines, so we have to handle it like this for the case where the
    // source ends with a newline.
    if let Some(src_line) = source.lines().nth(line - 1) {
        eprintln!("[{}:{}] {}", line, column, src_line);
    }

    eprintln!("{}", message);
}
