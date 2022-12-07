#![allow(non_camel_case_types, unused)]

mod bindings;

use bindings::*;

const PATTERN: &str = r"\d{4,}([^\d\s\n\r\f\t\v]{3,13}).+?";
const SUBJECT: &str = "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)(^!@#$%^&())9999999";

fn main() {
    let (start, end) = find(SUBJECT).unwrap();
    dbg!(&SUBJECT[start..end]);
}

fn find(subject: &str) -> Option<(usize, usize)> {
    let mut error_code = 0;
    let mut error_offset = 0;
    let pattern = PATTERN;
    // TODO: SAFETY
    let code = unsafe {
        pcre2_compile_8(
            pattern.as_ptr(),
            pattern.len(),
            PCRE2_UCP | PCRE2_UTF,
            &mut error_code,
            &mut error_offset,
            std::ptr::null_mut(),
        )
    };

    let match_data = unsafe { pcre2_match_data_create_from_pattern_8(code, std::ptr::null_mut()) };

    let ovector = unsafe { pcre2_get_ovector_pointer_8(match_data) };
    if ovector.is_null() {
        unsafe {
            pcre2_match_data_free_8(match_data);
            pcre2_code_free_8(code);
        }
        println!("could not get ovector");
        return None;
    }

    let rc = unsafe {
        pcre2_match_8(
            code,
            subject.as_ptr(),
            subject.len(),
            0,
            0,
            match_data,
            std::ptr::null_mut(),
        )
    };

    if rc <= 0 {
        unsafe {
            pcre2_match_data_free_8(match_data);
            pcre2_code_free_8(code);
        }
        println!("error executing match");
        return None;
    }
    let (s, e) = unsafe { (*ovector.offset(2), *ovector.offset(3)) };
    unsafe {
        pcre2_match_data_free_8(match_data);
        pcre2_code_free_8(code);
    }
    Some((s, e))
}
