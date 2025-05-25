/// Підраховує мінімальну кількість переміщень для вирівнювання вантажу
fn count_moves(shipments: &[u32]) -> isize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    if total % (n as u32) != 0 {
        return -1; // Неможливо розподілити рівномірно
    }

    let average = total / (n as u32);
    let mut balance: i32 = 0;
    let mut total_moves: i32 = 0;

    for &load in shipments {
        balance += load as i32 - average as i32;
        total_moves += balance.abs();
    }

    total_moves as isize
}

/// Генерує вектор вантажу, який вже збалансований
fn generate_balanced_shipments(n: usize, average: u32) -> Vec<u32> {
    vec![average; n]
}

fn main() {
    let test_cases = vec![
        vec![1, 1, 1, 1, 6],        // => 4
        vec![8, 2, 2, 4, 4],        // => 4
        vec![9, 3, 7, 2, 9],        // => 7
        vec![5, 5, 5, 5],           // => 0
        vec![10, 1, 1, 1],          // => -1 (неможливо)
    ];

    for (i, shipments) in test_cases.iter().enumerate() {
        let result = count_moves(shipments);
        println!("Example {}: {:?} → Moves = {}", i + 1, shipments, result);
    }

    let balanced = generate_balanced_shipments(6, 10);
    println!("\nGenerated balanced shipments: {:?}", balanced);
    println!("Moves required: {}", count_moves(&balanced));
}
