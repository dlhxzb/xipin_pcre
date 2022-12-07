#![allow(non_camel_case_types, unused)]
#![feature(result_option_inspect)]

mod bindings;

use std::net::UdpSocket;
use std::sync::mpsc::channel;
use std::time::Duration;

use bindings::*;

const PATTERN: &str = r"\d{4,}([^\d\s\n\r\f\t\v]{3,13}).+?";
const SUBJECT: &str = "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)(^!@#$%^&())9999999";
const UDP_ADDR: &str = "127.0.0.1:34254";

fn main() {
    let socket = UdpSocket::bind(UDP_ADDR).expect("Failed to bind Udp port");
    println!("Udp Server started at {UDP_ADDR}, may run client.sh now");

    let socket_recv = socket.try_clone().expect("Failed to clone socket");
    let (tx, rx) = channel();

    std::thread::spawn(move || loop {
        let mut buf = [0; 10];
        let (amt, src) = socket_recv
            .recv_from(&mut buf)
            .expect("Didn't receive data");
        println!("{src} connected {}", String::from_utf8_lossy(&buf[..amt]));
        tx.send(src);
    });

    let mut peers = vec![];
    loop {
        while let Ok(src) = rx.try_recv() {
            peers.push(src);
        }
        if let Some((start, end)) = find(SUBJECT) {
            peers.iter().for_each(|src| {
                println!("Send `{}` to {src}", &SUBJECT[start..end]);
                socket
                    .send_to(SUBJECT[start..end].as_bytes(), src)
                    .inspect_err(|e| println!("Udp send failed {e:?}"));
            });
        }
        std::thread::sleep(Duration::from_secs(2))
    }
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
