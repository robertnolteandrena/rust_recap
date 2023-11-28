use clone_copy::{Point2D, PointVariableD};

#[test]
fn test_integer_copy() {
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
#[test]
fn copy_point2d() {
    let a = Point2D { x: 1f32, y: 10f32 };
    let b = a;
    println!("a = {:?}, b = {:?}", a, b);
}
#[test]
fn no_copy_point_variable_d() {
    let a = PointVariableD {
        components: vec![1f32, 2f32],
    };
    let b = a;
    //If this line is uncommented the tests will not compile
    //println!("a = {:?}, b = {:?}", a, b);
}
#[test]
fn clone_point_variable_d() {
    let a = PointVariableD {
        components: vec![1f32, 2f32],
    };
    let b = a.clone();
    println!("a = {:?}, b = {:?}", a, b);
}
