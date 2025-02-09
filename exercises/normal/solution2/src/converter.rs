use regex::Regex;

// let num_str = "9(10)";
// let base_to: u32 = 8;
pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let mut result = String::new();
    let num_map = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
    ];

    let reg = Regex::new(r"(\d+)\((\d+)\)").unwrap();
    let caps = reg.captures(num_str).unwrap();
    // if let Some((num, base)) = caps {
    // 如果直接使用 caps[1] 会报错, 因为 caps[1] 默认是 str
    // 这里的 str 的意思就是，&str 中的 str。。。。
    let num = caps[1].to_string();
    let base = caps[2].to_string().parse::<u32>().unwrap();
    // println!("caps:\n {:?}", num);

    // 我的妈呀，很难想象 rust 没有 regex 标准库却有进制转换标准库
    let mut num_base_10 = i32::from_str_radix(&num, base).unwrap();
    // println!("num_base_10: {}   {}", num_base_10, to_base);

    if num_base_10 == 0 {
        return "0".to_string();
    }

    let mut temp = String::new();
    while num_base_10 > 0 {
        let digit = num_base_10 % to_base as i32;
        num_base_10 /= to_base as i32;
        temp.push_str(num_map[digit as usize]);
    }

    temp.chars().rev().collect()
}
