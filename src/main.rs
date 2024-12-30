fn main() {
    let budget = 556.90;
    let items = vec!
    [
        ("Gift A".to_string(), 170.50),
        ("Gift B".to_string(), 37.83),
        ("Gift C".to_string(), 127.0),
        ("Gift D".to_string(), 351.0),
        ("Gift E".to_string(), 50.0),
        ("Gift F".to_string(), 166.25),
        ("Gift G".to_string(), 35.36),
        ("Gift H".to_string(), 15.50),
        ("Gift I".to_string(), 510.19),
        ("Gift J".to_string(), 262.5),
        ("Gift K".to_string(), 150.00),
        ("Gift L".to_string(), 48.50),
        ("Gift M".to_string(), 265.30),
        ("Gift N".to_string(), 552.39),
    ];
    let (spent, selected) = knapsack(budget, items);
    println!("Max spent: {}, Selected items: {:?}", spent, selected);
}
fn knapsack(budget: f32, items: Vec<(String, f32)>) -> (f32, Vec<String>) {
    let budget_scaled = (budget * 100.0).round() as usize;
    let mut dp = vec![0; budget_scaled + 1];
    let mut item_selection = vec![vec![]; budget_scaled + 1];

    for (name, cost) in items {
        let cost_scaled = (cost * 100.0).round() as usize;

        for j in (cost_scaled..=budget_scaled).rev() {
            if dp[j] < dp[j - cost_scaled] + cost_scaled {
                dp[j] = dp[j - cost_scaled] + cost_scaled;
                item_selection[j] = item_selection[j - cost_scaled].clone();
                item_selection[j].push(name.clone());
            }
        }
    }

    let max_spent = dp[budget_scaled] as f32 / 100.0;
    let selected_items = item_selection[budget_scaled].clone();

    (max_spent, selected_items)
}

