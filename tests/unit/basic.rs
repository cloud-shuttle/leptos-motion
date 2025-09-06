#[test]
fn test_basic_math() {
    assert_eq!(2 + 2, 4);
    assert_eq!(10 - 5, 5);
    assert_eq!(3 * 4, 12);
    assert_eq!(15 / 3, 5);
}

#[test]
fn test_string_operations() {
    let hello = "Hello";
    let world = "World";
    let combined = format!("{} {}", hello, world);
    assert_eq!(combined, "Hello World");
}

#[test]
fn test_vector_operations() {
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec[3], 4);
}

#[test]
fn test_option_operations() {
    let some_value = Some(42);
    let none_value: Option<i32> = None;

    assert_eq!(some_value.unwrap(), 42);
    assert_eq!(none_value.unwrap_or(0), 0);
}

#[test]
fn test_result_operations() {
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");

    assert_eq!(ok_result.unwrap(), 42);
    assert_eq!(err_result.unwrap_or(0), 0);
}
