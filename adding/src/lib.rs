pub fn add_curry(x: i32) -> Box<dyn Fn(i32) -> i32> {
    // The closure takes an argument y and adds it to the pre-bound value x
    Box::new(move |y: i32| x + y)
}
