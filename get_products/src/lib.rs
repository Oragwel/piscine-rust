pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let total_product: usize = arr.iter().product(); // Calculate the total product of all elements in the vector

    arr.iter().map(|&num| total_product / num).collect() // For each element, calculate the product excluding that element
}
