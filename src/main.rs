fn main() {
    println!("Hello, world!");
}

pub struct Item {
    pub name: String,
    pub weight: usize,
    pub value: usize,
}

fn knapsack(items: Vec<Item>, capacity: usize) -> Vec<Item> {
    let mut table = vec![vec![0; capacity + 1]; items.len() + 1];

    items.iter().zip(0..capacity).for_each(|(item, i)| {
        (1..capacity).for_each(|j| {
            table[i][j] = if item.weight > j {
                table[i - 1][j]
            } else {
                table[i - 1][j].max(table[i - 1][j - item.weight] + item.value)
            }
        })
    });

    let mut result = Vec::new();
    let mut i = items.len();
    let mut j = capacity;

    while i > 0 {
        if table[i][j] != table[i - 1][j] {
            result.push(items[i - 1].clone());
            j -= items[i - 1].weight;
        }
        i -= 1;
    }

    result
}
