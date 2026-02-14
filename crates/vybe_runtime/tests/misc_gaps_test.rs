use std::path::PathBuf;

#[test]
fn test_misc_gaps() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests/test_misc_gaps.vb");
    
    let source = std::fs::read_to_string(&test_file)
        .expect("Failed to read test_misc_gaps.vb");
    
    let parsed = vybe_parser::parse_vb(&source)
        .expect("Failed to parse test_misc_gaps.vb");
    
    let mut interp = vybe_runtime::Interpreter::new();
    let console_output = interp.get_console_output();
    
    interp.run(&parsed).expect("Runtime error in test_misc_gaps.vb");
    
    let output = console_output.borrow().join("");
    println!("{}", output);
    
    assert!(output.contains("SUCCESS"), "Misc gaps test did not pass:\n{}", output);
    assert!(!output.contains("FAIL:"), "Misc gaps test had failures:\n{}", output);
}
