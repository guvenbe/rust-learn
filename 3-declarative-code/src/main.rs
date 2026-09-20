use parser_state_demo::ParserState;

fn main() {
    let mut state = ParserState::new();
    println!("START: {state:?}");

    // Make the long String live for the whole scope
    let big = "x".repeat(2000);

    let lines = [
        "not-a-header",
        "still-not-a-header",
        "---",
        "hello body line 1",
        "hello body line 2",
        big.as_str(), // now this &str points to a live String
        "",
        "",
    ];

    for (i, l) in lines.iter().enumerate() {
        let _ = state.process_line(l.to_string());
        println!("after line {i}: {state:?}");
    }
}
