#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        // Try to find the product in the store
        if let Some(pos) = s.products.iter().position(|(x, _)| *x == ele) {
            self.items.push((ele, s.products[pos].1)); // Add the product to the cart
        }
    }

    pub fn get_prices(&self) -> Vec<f32> {
        self.items.iter().map(|(_, v)| *v).collect::<Vec<f32>>()
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.get_prices();
        let num_items = prices.len();
        
        // Sort prices to apply the promotion correctly
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Calculate how many full groups of 3 items there are
        let cal = num_items / 3;

        // Discount ratio: the sum of the items after applying "buy 3, get 1 free"
        let v: Vec<f32> = prices[cal..].to_vec(); // Remove the first "group" of items (those that are free)
        let percentage: f32 = (v.iter().sum::<f32>() * 100.0) / prices.iter().sum::<f32>();

        // Apply the discount to each price in the cart and round to 2 decimal places
        self.receipt = prices
            .iter()
            .map(|price| round_two(price * percentage / 100.0))
            .collect::<Vec<f32>>();

        self.receipt.clone()
    }
}

// Helper function to round numbers to two decimal places
fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}
