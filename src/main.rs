use chrono::offset::{TimeZone, Utc};
use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    let mut term = false;
    let mut conky = false;
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let query = &args[1];
            match &query[..] {
                "term" | "terminal" => term = true,
                "conky" => conky = true,
                _ => {}
            }
        }
        _ => {}
    }
    let mut events: Vec<&str> = Vec::new();
    let mut data = [0u8; 1512];
    match TcpStream::connect("localhost:9977") {
        Ok(mut stream) => {
            let msg = b"events";
            stream.write(msg).unwrap();
            match stream.read(&mut data) {
                Ok(n) => {
                    if n == 0 {
                        std::process::exit(0x0000);
                    }
                    let mut reply: Vec<&str> = from_utf8(&data[0..n]).unwrap().lines().collect();
                    events.append(&mut reply);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect:{}", e);
        }
    }
    for event in events.iter() {
        let mut splitter = event.splitn(2, ',');
        let first = splitter.next().unwrap();
        let mut second: String = splitter.next().unwrap().to_owned();
        let now = Utc::now();
        second.push_str(&now.format(" %Y").to_string());
        let date = Utc
            .datetime_from_str(second.trim(), "%d %B, %I:%M %p %Y")
            .unwrap();
        let duration = date - now;
        if duration.num_days() >= 2 {
            if term {
                println!(
                "\x1b[38;2;250;169;22m{},\x1b[0m remaining days to submit are \x1b[38;2;104;161;223m{} days\x1b[0m",
                first,
                duration.num_days()
            );
            } else if conky {
                println!(
                "${{#FAA916}}{},${{#FBFFFE}} remaining days to submit are ${{#68A1DF}}{} days${{#FBFFFE}}",
                first,
                duration.num_days()
                );
            } else {
                println!(
                    "{}, remaining days to submit are {} days",
                    first,
                    duration.num_days()
                );
            }
        } else if duration.num_days() >= 1 {
            if term {
                println!("{},\x1b[0m submit \x1b[0;31mToday\x1b[0m", first);
            } else if conky {
                println!(
                    "${{#FAA916}}{},${{#FBFFFE}} submit ${{#FF0000}}Today${{#FBFFFE}}",
                    first
                );
            } else {
                println!("{}, submit Today", first);
            }
        }
    }
}
