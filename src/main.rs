use calculator::{codegen::CodeGenerator, parser::Parser, vm::Vm};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <expression>", env!("CARGO_BIN_NAME"));
        return;
    }

    let source = args[1].as_str();
    let mut parser = Parser::new(source);
    let ast = parser.parse();
    if ast.complete() {
        let bytecode = CodeGenerator::default().generate(&ast);
        let mut vm = Vm::new(bytecode);
        match vm.run() {
            Ok(value) => println!("{}", value),
            Err(err) => eprintln!("runtime error: {:?}", err),
        }
    }
}
