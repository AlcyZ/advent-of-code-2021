mod part_a;
mod part_b;

fn bin_str_to_decimal(value: &str) -> i32 {
    i32::from_str_radix(value, 2).unwrap()
}
