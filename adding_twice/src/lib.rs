// The add_curry function that returns a closure to add a given value to the input.
pub fn add_curry(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y: i32| x + y)
}

// The twice function that takes a closure f(x) and returns a closure f(f(x))
pub fn twice<F>(f: F) -> Box<dyn Fn(i32) -> i32>
where
    F: Fn(i32) -> i32 + 'static,
{
    Box::new(move |x| f(f(x)))  // Applies f twice: f(f(x))
}
