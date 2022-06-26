// #![feature(exclusive_range_pattern)]

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
            let v1 = parse_6bit(((c[0] & 0x3) << 4) | ((c[1] & 0xf0) >> 4)).unwrap(); 
            ans.push(v1); 
            let v1 = parse_6bit(((c[1] & 0xf) << 2) | ((c[2] & 0xc) >> 6)).unwrap(); 
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
        c[index] = v; 
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
        1 => panic!("Why you get an error base64 str? "), 
        2 => ans.push((c[0] << 2) | (c[1] >> 4)), 
        3 => {
            ans.push((c[0] << 2) | (c[1] >> 4)); 
            ans.push((c[1] << 4) | (c[2] >> 2)); 
        }
        // 4 => {
        //     ans.push((c[0] << 2) | (c[1] >> 4)); 
        //     ans.push((c[1] << 4) | (c[2] >> 2)); 
        //     ans.push((c[2] << 6) | c[3]); 
        // }
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