mod converter;

fn main() {
    let num_str = "12(10)";
    let base_to: u32 = 16;
    let result = converter::convert_base(num_str, base_to);
    println!("{} -> {}", num_str, result);
}
