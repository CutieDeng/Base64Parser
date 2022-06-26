fn main() {
    let mut input = String::new(); 
    std::io::stdin().read_line(&mut input).unwrap(); 
    println!("The base64 val: {}", 
        base64parse::parse_into_base64(input.trim_end().as_bytes())); 
}
