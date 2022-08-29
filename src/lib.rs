use std::io::{Bytes, Read, Write};

pub fn parse_into_base64(a: &[u8]) -> String {
    let mut c = unsafe { std::mem::MaybeUninit::<[u8; 3]>::uninit().assume_init() }; 
    let mut index = 0; 
    let mut ans = Vec::<u8>::with_capacity(a.len()); 
    for &v in a {
        c[index] = v; 
        if index == 2 {
            index = 0; 
            let v1 = parse_6bit(c[0] >> 2).unwrap(); 
            ans.push(v1); 
            let v1 = parse_6bit(((c[0] & 0x3) << 4) | (c[1] >> 4)).unwrap(); 
            ans.push(v1); 
            let v1 = parse_6bit(((c[1] & 0xf) << 2) | (c[2] >> 6)).unwrap(); 
            ans.push(v1); 
            let v1 = parse_6bit(c[2] & 0x3f).unwrap(); 
            ans.push(v1); 
        } else {
            index += 1; 
        }
    }
    match index {
        1 => {
            let v1 = parse_6bit(c[0] >> 2).unwrap(); 
            ans.push(v1); 
            let v1 = parse_6bit((c[0] & 0x3) << 4).unwrap(); 
            ans.push(v1); 
            ans.push('=' as u8); 
            ans.push('=' as u8); 
        }
        2 => {
            let v1 = parse_6bit(c[0] >> 2).unwrap(); 
            ans.push(v1); 
            let v1 = parse_6bit(((c[0] & 0x3) << 4) | ((c[1] & 0xf0) >> 4)).unwrap(); 
            ans.push(v1); 
            let v1 = parse_6bit((c[1] & 0xf) << 2).unwrap(); 
            ans.push(v1); 
            ans.push('=' as u8); 
        }
        _ => (), 
    }
    String::from_utf8(ans).unwrap()
}

pub fn parse_from_base64(a: &[u8]) -> Vec<u8> {
    let mut c: [u8; 4] = unsafe { std::mem::MaybeUninit::uninit().assume_init() }; 
    let mut index = 0; 
    let mut ans = Vec::<u8>::new();
    for &v in a {
        if v == '=' as u8 {
            break; 
        }
        c[index] = match v {
            v if v >= 'A' as u8 && v <= 'Z' as u8 => v - 'A' as u8, 
            v if v >= 'a' as u8 && v <= 'z' as u8 => v - 'a' as u8 + 26, 
            v if v >= '0' as u8 && v <= '9' as u8 => v - '0' as u8 + 52, 
            v if v == '+' as u8 => 62, 
            v if v == '/' as u8 => 63, 
            _ => panic!("Invalid Base64 Character"), 
        };
        if index == 3 {
            index = 0; 
            ans.push((c[0] << 2) | (c[1] >> 4)); 
            ans.push((c[1] << 4) | (c[2] >> 2)); 
            ans.push((c[2] << 6) | c[3]); 
        } else {
            index += 1; 
        }
    }
    match index {
        0 => (), 
        1 => panic!("Why you get an error base64 str? "), 
        2 => ans.push((c[0] << 2) | (c[1] >> 4)), 
        3 => {
            ans.push((c[0] << 2) | (c[1] >> 4)); 
            ans.push((c[1] << 4) | (c[2] >> 2)); 
        }
        _ => panic!("Amazing val!"), 
    }
    // String::from_utf8(ans).ok() 
    ans
}

// #[allow(arithmetic_overflow)]
fn parse_6bit(a: u8) -> Option<u8> {
    Some (match a {
        a if a < 26 => a + 'A' as u8, 
        a if a < 52 => a + ('a' as u8 - 26), 
        a if a < 62 => a + '0' as u8 - 52, 
        62 => '+' as u8, 
        63 => '/' as u8, 
        _ => return None,
    })
}

pub fn decode_base64(input: Bytes<Box<dyn Read>>, output: &mut Box<dyn Write>) -> Result<(), String> {
    let mut c = [0u8; 4]; 
    let mut index = 0; 
    let mut ans = Vec::<u8>::with_capacity(200); 
    for v in input {
        let v = v.map_err(|e| e.to_string())?; 
        if v == '=' as u8 || v == '\n' as u8 || v == '\r' as u8 {
            // '=' would hold up the error infomation after it... Because it would ends the program and 
            // request a answer. 
            
            // update, now '\r' & '\n' is also included to the escape character. 
            break; 
        }
        c[index] = match v {
            v if v >= 'A' as u8 && v <= 'Z' as u8 => v - 'A' as u8, 
            v if v >= 'a' as u8 && v <= 'z' as u8 => v - 'a' as u8 + 26, 
            v if v >= '0' as u8 && v <= '9' as u8 => v - '0' as u8 + 52, 
            v if v == '+' as u8 => 62, 
            v if v == '/' as u8 => 63, 
            _ => return Err(format!("Invalid Base64 Character: '{}'", v as char)), 
        }; 
        index += 1;
        if index == 4 {
            index = 0; 
            ans.push((c[0] << 2) | (c[1] >> 4)); 
            ans.push((c[1] << 4) | (c[2] >> 2)); 
            ans.push((c[2] << 6) | c[3]); 
        } 
    }
    match index {
        0 => (), 
        1 => return Err(format!("Why you get an error base64 str? ")), 
        2 => ans.push((c[0] << 2) | (c[1] >> 4)), 
        3 => {
            ans.push((c[0] << 2) | (c[1] >> 4)); 
            ans.push((c[1] << 4) | (c[2] >> 2)); 
        }
        _ => panic!("Invalid index value: {}", index), 
    }
    output.write_all(&ans).map_err(|e| e.to_string())?; 
    output.flush().map_err(|e| e.to_string())?; 
    Ok(())
}

pub fn encode_base64(input: Bytes<Box<dyn Read>>, output: &mut Box<dyn Write>) -> Result<(), String> {
    let mut cache = [0u8; 3]; 
    let mut index = 0; 
    let mut ans: Vec<u8> = Vec::with_capacity(200); 
    for v in input {
        let v = v.map_err(|e| e.to_string())?; 
        cache[index] = v; 
        index += 1; 
        if index == 3 {
            index = 0; 
            ans.push(parse6bit(cache[0] >> 2)?); 
            ans.push(parse6bit(((cache[0] & 0x3) << 4) | (cache[1] >> 4))?); 
            ans.push(parse6bit(((cache[1] & 0xf) << 2) | (cache[2] >> 6))?); 
            ans.push(parse6bit(cache[2] & 0x3f)?);
        }
    }
    match index {
        1 => {
            let v1 = parse6bit(cache[0] >> 2)?; 
            ans.push(v1); 
            let v1 = parse6bit((cache[0] & 0x3) << 4)?; 
            ans.push(v1); 
            ans.push('=' as u8); 
            ans.push('=' as u8); 
        }
        2 => {
            let v1 = parse6bit(cache[0] >> 2)?; 
            ans.push(v1); 
            let v1 = parse6bit(((cache[0] & 0x3) << 4) | ((cache[1] & 0xf0) >> 4))?; 
            ans.push(v1); 
            let v1 = parse6bit((cache[1] & 0xf) << 2)?; 
            ans.push(v1); 
            ans.push('=' as u8); 
        }
        0 => (), 
        o => panic!("Meet an unexpected index value of cache array: {}", o),
    }
    output.write_all(&ans).map_err(|e| e.to_string())?; 
    output.flush().map_err(|e| e.to_string())?; 
    Ok(())
}

fn parse6bit (a: u8) -> Result<u8, String> {
    Ok (match a {
        a if a < 26 => a + 'A' as u8, 
        a if a < 52 => a + ('a' as u8 - 26), 
        a if a < 62 => a + '0' as u8 - 52, 
        62 => '+' as u8, 
        63 => '/' as u8, 
        other => return Err(format!("Unexpected value {} in the func 'parse6bit'. ", other)), 
    })
}