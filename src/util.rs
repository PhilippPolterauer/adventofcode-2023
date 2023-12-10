pub fn load_file(day: i32, part: i32, runtest: bool, data_path: &str) -> String {
    let teststr = if runtest { "test_" } else { "" };

    let path = std::format!("{data_path}/day{day}/{teststr}input{part}.txt");
    println!("loading data from '{}'", path);
    std::fs::read_to_string(path).unwrap()
}
