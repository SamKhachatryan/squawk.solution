pub fn decode(data: &[u32]) -> Vec<u32> {
    let emergency_code = 7700;
    let xor_coef = emergency_code / 100;

    data.iter()
        .map(|&num| num ^ xor_coef)
        .collect()
}