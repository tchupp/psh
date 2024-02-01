use crate::Parse;

#[test]
fn repl_line() {
    psh_test_harness::run_test_dir("repl_line", |_path, input| {
        run_parser_test(input, crate::parse_repl_line)
    });
}

fn run_parser_test(input: &str, parsing_fn: fn(&str) -> Parse) -> String {
    let actual_parse = parsing_fn(input);

    actual_parse.debug_tree().to_string()
}
