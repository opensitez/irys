use std::path::PathBuf;

#[test]
fn test_threading_and_tasks() {
    let test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().parent().unwrap()
        .join("tests/test_threading.vb");
    
    let source = std::fs::read_to_string(&test_file)
        .expect("Failed to read test_threading.vb");
    
    let parsed = vybe_parser::parse_vb(&source)
        .expect("Failed to parse test_threading.vb");
    
    let mut interp = vybe_runtime::Interpreter::new();
    let console_output = interp.get_console_output();
    
    interp.run(&parsed).expect("Runtime error in test_threading.vb");
    
    let output = console_output.borrow().join("");
    println!("{}", output);
    
    assert!(output.contains("SUCCESS"), "Threading test did not pass:\n{}", output);
    assert!(!output.contains("FAIL:"), "Threading test had failures:\n{}", output);
}
