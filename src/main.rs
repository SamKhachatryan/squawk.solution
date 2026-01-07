mod decode;

fn main() {
    let data: Vec<u32> = vec![
        35, 44, 47, 36, 33, 13, 44, 36, 63, 62, 61, 44, 46, 40, 96, 36, 35, 57, 40, 33, 33, 36, 42,
        40, 35, 46, 40, 99, 46, 34, 32,
    ];

    let decoded = decode::decode(&data);

    println!(
        "{:?}",
        decoded
            .iter().map(|n| char::from_u32(*n).expect("Invalid character found"))
            .collect::<String>()
    );
}
