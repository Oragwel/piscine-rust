use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

/// Rounds to 2 decimal places, then formats to drop trailing zero if needed.
fn round(value: f64) -> f64 {
    let rounded = (value * 100.0).round() / 100.0;
    let formatted = format!("{:.2}", rounded);
    formatted.parse::<f64>().unwrap()
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Extract kcal string, remove "kcal", and parse to f64
        let kcal_str = &food.calories[1];
        let kcal_val: f64 = kcal_str
            .trim_end_matches("kcal")
            .parse()
            .unwrap_or(0.0);

        let portions = food.nbr_of_portions;

        total_cals += kcal_val * portions;
        total_carbs += food.carbs * portions;
        total_proteins += food.proteins * portions;
        total_fats += food.fats * portions;
    }

    json::object! {
        "cals" => round(total_cals),
        "carbs" => round(total_carbs),
        "proteins" => round(total_proteins),
        "fats" => round(total_fats),
    }
}
