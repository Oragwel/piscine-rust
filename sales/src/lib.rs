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
        let pos = s.products.iter().position(|(x, _)| *x == ele).unwrap();
        self.items.push((ele, s.products[pos].1));
    }

    pub fn get_prices(&self) -> Vec<f32> {
        self.items.iter().map(|(_, v)| *v).collect::<Vec<f32>>()
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.get_prices();
        let cal = self.items.len() / 3;
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let v: Vec<f32> = pri#[derive(Debug, Clone, PartialEq)]
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
                    items: Vec::new(),
                    receipt: Vec::new(),
                }
            }
        
            pub fn insert_item(&mut self, store: &Store, ele: String) {
                // Look up item in store and add (name, price) to the cart
                if let Some((name, price)) = store.products.iter().find(|(name, _)| name == &ele) {
                    self.items.push((name.clone(), *price));
                }
            }
        
            pub fn generate_receipt(&mut self) -> Vec<f32> {
                let prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
            
                let mut receipt = vec![];
                let mut index = 0;
            
                while index < prices.len() {
                    let group = &prices[index..(index + 3).min(prices.len())];
                    let  group_vec = group.to_vec();
            
                    if group_vec.len() == 3 {
                        let min = group_vec.iter().cloned().fold(f32::INFINITY, f32::min);
                        let group_sum: f32 = group_vec.iter().sum();
                        let discount_ratio = min / group_sum;
            
                        for &price in group_vec.iter() {
                            let adjusted = price * (1.0 - discount_ratio);
                            receipt.push((adjusted * 100.0).round() / 100.0);
                        }
                    } else {
                        for &price in group_vec.iter() {
                            receipt.push((price * 100.0).round() / 100.0);
                        }
                    }
            
                    index += 3;
                }
            
                receipt.sort_by(|a, b| a.partial_cmp(b).unwrap()); // ✅ sort for test comparison
                self.receipt = receipt.clone();
                receipt
            }
        }   
            2>>();

        self.receipt.clone()
    }
}

fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}