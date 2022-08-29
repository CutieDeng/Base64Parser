use std::{env::args, fs::File, io::Read};

fn main() -> Result<(), &'static str> {
    {
        let args: Vec<_> = args().collect(); 
        let result = args.iter().enumerate().find(|&(_i, s)| -> bool {
            *s == "-f"
        }); 
        if let Some((result, _)) = result {
            let file_name; 
            if args.len() > result + 1 {
                file_name = args[result + 1].clone(); 
            } else {
                return Err("Need a file name. The correct format is [program name] -f [file name]"); 
            }
            let t = File::open(file_name).map_err(|_e| -> &'static str {
                "File open error"
            })?; 
            let bytes: Vec<u8> = t.bytes().filter_map(Result::ok).collect(); 
            let result = base64parse::parse_into_base64(&bytes); 
            println!("File input with size [{}], the base64 val: \n", bytes.len()); 
            println!("{}", result); 
            return Ok(()); 
        } 
    }
    let mut input = String::new(); 
    println!("Please input your info in one line, then I would parse it as base64 encodes. "); 
    if std::io::stdin().read_line(&mut input).unwrap() == 0 {
        return Err("Err with the empty input. "); 
    } 
    let input = input.trim_end(); 
    if input.chars().count() == 0 {
        return Err("Err with the empty input. "); 
    }
    println!("Correctly input(Trim the end '\\n' char). [input size = {}] The base64 val: \n{}", 
        input.chars().count(), 
        base64parse::parse_into_base64(input.as_bytes())); 
    Ok(())
}
