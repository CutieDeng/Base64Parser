use std::{env, io::{Read, Write}, fs::File};

const MANUAL_INFO: &'static str = include_str!("../../res/manual-info.txt"); 
const PROJECT_VERSION: &'static str = env!("CARGO_PKG_VERSION"); 

fn main() -> Result<(), String> {
    let args = env::args(); 
    let args: Vec<_> = args.collect(); 
    let mut new_line_op = true; 
    assert!(args.len() >= 1, "The program arguments must be larger than 1. "); 
    if args.len() == 1 || (args.len() == 2 && (args[1] == "-help" || args[1] == "-h")) {
        print!("{}", MANUAL_INFO); 
        return Ok(()); 
    }
    if args.len() == 2 && (args[1] == "-v" || args[1] == "--version") {
        print!("Base64Parser Manual\nVersion: {}\n", PROJECT_VERSION); 
        return Ok(()); 
    }
    let input_stream: Box<dyn Read>; 
    let mut output_stream: Box<dyn Write>; 
    let mut encode = None; 
    { 
        let mut file_name: Option<&String> = None; 
        let mut output_file_name: Option<&String> = None; 
        let mut it = args.iter().skip(1); 
        let mut raw_mode = false; 
        loop {
            let rs; 
            if let Some(v) = it.next() {
                rs = v; 
            } else {
                break; 
            }
            if rs == "-o" || rs == "--output" {
                // escape the next object. 
                let ofn = it.next().ok_or_else(|| format!("Missing the output file to redirect. "))?; 
                if output_file_name.is_some() {
                    return Err(format!("Redirect the output stream to more than one file. {{{}, {} ..}}", 
                        output_file_name.unwrap(), ofn));   
                }
                output_file_name = Some(ofn); 
            } else if rs.starts_with('-') {
                const EN_AND_DE_ERR_INFO: &'static str = "Invalid Program arguments: conflicted -e & -d";
                if rs == "-e" || rs == "--encode" {
                    if let Some(_b @ false) = encode {
                        return Err(format!("{}", EN_AND_DE_ERR_INFO)); 
                    }
                    encode = Some(true); 
                } else if rs == "-d" || rs == "--decode" {
                    if let Some(_b @ true) = encode {
                        return Err(format!("{}", EN_AND_DE_ERR_INFO)); 
                    }
                    encode = Some(false); 
                } else if rs == "--no-newline" {
                    new_line_op = false; 
                } else if rs == "-r" {
                    raw_mode = true; 
                }
            } else {
                if file_name.is_some() {
                    return Err(format!("Redirect the input stream from more than one file. {{{}, {} ..}}.", 
                        file_name.unwrap(), rs)); 
                }
                file_name = Some(rs); 
            }
        }
        if raw_mode {
            if file_name.is_some() {
                return Err(format!("Raw mode doesn't need a input file '{}'..", file_name.unwrap())); 
            }     
            input_stream = Box::new(std::io::stdin()); 
        } else {
            let file_name = file_name.ok_or("No file to deal. ").map_err(String::from)?;
            input_stream = Box::new(File::open(file_name).map_err(|e| format!("{}", e))?);  
        }
        let os = output_file_name.map(File::create).map(|r| r.map_err(|e| e.to_string())); 
        if let Some(os) = os {
            let os = os?; 
            output_stream = Box::new(os); 
        } else {
            output_stream = Box::new(std::io::stdout()); 
        }
    }
    let b = input_stream.bytes(); 
    let encode = encode.ok_or_else(|| format!("Missing the encoding operators: '-e' or '-d'. "))?; 
    if encode {
        base64parse::encode_base64(b, &mut output_stream)?; 
    } else {
        base64parse::decode_base64(b, &mut output_stream)?; 
    }
    if new_line_op {
        let _ = output_stream.write(b"\n"); 
    } 
    Ok(())
}