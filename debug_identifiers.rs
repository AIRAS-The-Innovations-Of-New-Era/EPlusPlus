use pest::Parser;
use crate::parser::{EppParser, Rule};

fn main() {
    let test_cases = vec![
        "print",    // This should work
        "myvar",    // This should work
        "int",      // This is failing
        "float",    // Let's test this too
        "str",      // And this
        "bool",     // And this
    ];

    for test in test_cases {
        println!("Testing identifier: '{}'", test);
        match EppParser::parse(Rule::identifier, test) {
            Ok(_) => println!("  ✅ SUCCESS"),
            Err(e) => println!("  ❌ FAILED: {}", e),
        }
    }
}
