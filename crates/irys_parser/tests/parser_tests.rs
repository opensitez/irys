
use irys_parser::parser::parse_program;
// use irys_parser::ast::*;

// Test for single argument implicit call
#[test]
fn test_implicit_call_one_arg() {
    let code = r#"
        Sub Test()
            MsgBox "Hello"
        End Sub
    "#;
    let result = parse_program(code);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test for multiple arguments implicit call
#[test]
fn test_implicit_call_multi_args() {
    let code = r#"
        Sub Test()
            Foo Arg1, 42
        End Sub
    "#;
    let result = parse_program(code);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test for call with parens (explicit call)
#[test]
fn test_explicit_call_parens() {
    let code = r#"
        Sub Test()
            Call MsgBox("Hello")
        End Sub
    "#;
    let result = parse_program(code);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test call without args
#[test]
fn test_call_no_args() {
    let code = r#"
        Sub Test()
            DoSomething
        End Sub
    "#;
    let result = parse_program(code);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

#[test]
fn test_assignment() {
    let code = r#"
        Sub Test()
            x = 1
        End Sub
    "#;
    let result = parse_program(code);
    assert!(result.is_ok(), "Failed to parse assignment: {:?}", result.err());
}

#[test]
fn test_reproduction_empty_lines() {
    let input = r#"
Sub Test()
    MsgBox "Hello"
End Sub

"#; 
    // The input above has a trailing newline which might be line 5 if counted.
    // Line 1: empty (if starts with newline) or Sub Test
    // Let's try to match the user's "5:1" error.
    // 1: Sub Test()
    // 2:     MsgBox "Hello"
    // 3: End Sub
    // 4: 
    // 5: <EOF>
    
    let result = irys_parser::parse_program(input);
    assert!(result.is_ok(), "Failed to parse program with trailing newlines: {:?}", result.err());
}

#[test]
fn test_trailing_newlines() {
    let code = "Private Sub btn_Click()\n    MsgBox \"Hi\"\nEnd Sub\n\n\n";
    let result = parse_program(code);
    assert!(result.is_ok(), "Failed to parse with trailing newlines: {:?}", result.err());
}
#[test]
fn test_repro_user_errors() {
    // Case 1: Empty line between sub header and body
    let code1 = "Sub btn1_Click()\n\n    MsgBox \"Hi\"\nEnd Sub";
    assert!(parse_program(code1).is_ok(), "Failed code1");

    // Case 2: Empty line before End Sub
    let code2 = "Sub btn1_Click()\n    MsgBox \"Hi\"\n\nEnd Sub";
    assert!(parse_program(code2).is_ok(), "Failed code2");

    // Case 3: Spaces and newlines everywhere
    let code3 = " \n\nSub Test()  \n  \n  x = 1 \n \nEnd Sub \n \n ";
    assert!(parse_program(code3).is_ok(), "Failed code3");
}

#[test]
fn test_fifty_empty_lines() {
    let mut code = String::from("Sub Test()\n");
    for _ in 0..50 {
        code.push_str("   \n\t\n\n");
    }
    code.push_str("    MsgBox \"Done\"\n");
    for _ in 0..50 {
        code.push_str("   \n\t\n\n");
    }
    code.push_str("End Sub");
    
    let result = parse_program(&code);
    assert!(result.is_ok(), "Failed to parse with 50+ empty lines: {:?}", result.err());
}
