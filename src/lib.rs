struct MyI32;

#[my_macro::transform_function]
fn something(a: i32, b: i32) -> i32 {
    a + b
}
