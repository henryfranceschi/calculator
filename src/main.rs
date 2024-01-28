use calculator::lexer::span::Span;

fn main() {
    println!("Hello, world!");
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
