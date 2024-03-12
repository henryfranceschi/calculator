use crate::lexer::span::Span;

pub fn report_error(message: &str, span: Span, source: &str) {
    let line = span.starting_line_number(source);
    let column = span.starting_column_number(source);
    let margin_width = 3 + digit_count(line as u32) + digit_count(column as u32);

    eprintln!("error: {message}");
    if !span.is_empty() {
        let src_line = source.lines().nth(line - 1).expect("invalid line");
        eprintln!("[{}:{}] {}", line, column, src_line);
        eprintln!(
            "{} {}",
            str::repeat(" ", margin_width as usize + (column - 1)),
            str::repeat("^", span.len()),
        );
    } else {
        // The only kind of token that has an empty span is eof which is always the last token
        // yielded by the lexer.
        eprintln!("[eof]");
    }
}

fn digit_count(n: u32) -> u32 {
    n.ilog10() + 1
}
