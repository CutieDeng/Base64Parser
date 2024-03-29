fn main() -> Result<(), &'static str>{
    let mut input = String::new(); 
    println!("Base64 decoder: please input your base64 string. "); 
    std::io::stdin().read_line(&mut input).unwrap(); 
    println!("The base64 origin val: {}", 
        String::from_utf8_lossy(&base64parse::parse_from_base64(input.trim_end().as_bytes()))); 
    Ok(())
}
