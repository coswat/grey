#[cfg(test)]
use grey::vars;

#[test]
fn get_cmd_test() {
    // test 1
    let result: String = vars::get_cmd();
    let expected: String = String::new();
    assert_eq!(expected, result);
}

#[test]
fn get_args_test() {
    let result: Vec<String> = vars::get_args();
    let expected: Vec<String> = vec![];
    assert_eq!(expected, result);
}
