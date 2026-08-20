use std::process::Command;

#[test]
fn run_all_cases() {
    let output = Command::new("./run_tests.sh")
        .current_dir("../../tests/parsing")
        .arg("../../target/debug/yajl_test")
        .output()
        .expect("successful run");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    assert!(output.status.success());
}
