use chrono::offset::{TimeZone, Utc};
use filebuffer::FileBuffer;
use std::env;
use std::io::{BufWriter, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    // term variable to set term color output
    let mut term = false;
    // conky variable to set conky color output
    let mut conky = false;
    let mut tasks: Vec<String> = Vec::new();
    // args are the arguments supplied to the application
    let args: Vec<String> = env::args().collect();
    // For now we enable only one argument
    if let 2 = args.len() {
        let query = &args[1];
        // matching argument query and setting variable
        match &query[..] {
            "term" | "terminal" => term = true,
            "conky" => conky = true,
            "add" => {
                let mut task = String::new();
                let mut deadline = String::new();
                print!("Enter a task >> ");
                let _ = std::io::stdout().flush();
                std::io::stdin().read_line(&mut task).unwrap();
                task = task.trim().to_string();
                print!("Enter a Deadline[day month, hh::mm AM/PM year] >> ");
                let _ = std::io::stdout().flush();
                std::io::stdin().read_line(&mut deadline).unwrap();
                deadline = deadline.trim().to_string();
                add_tasks(&mut task, &mut deadline);
                std::process::exit(0);
            }
            "remove" => {
                let mut task = String::new();
                print!("Enter a task >> ");
                let _ = std::io::stdout().flush();
                std::io::stdin()
                    .read_line(&mut task)
                    .expect("Error reading from STDIN");
                task = task.trim().to_string();
                get_tasks(&mut tasks);
                remove_tasks(&mut task, &mut tasks);
                std::process::exit(0);
            }
            _ => {}
        }
    }
    //Creating events vector
    let mut events: Vec<String> = Vec::new();

    get_events(&mut events);
    get_tasks(&mut tasks);
    // We are splitting the given events into info , datetime string and parsing
    // the datetime string into datetime variable and finding duration or time left
    // based current on current time and date
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
        print_duration(duration, term, conky, first);
    }

    for task in tasks.iter() {
        let mut splitter = task.splitn(2, ',');
        let first = splitter.next().unwrap();
        let second: String = splitter.next().unwrap().to_owned();
        let now = Utc::now();
        let date = Utc
            .datetime_from_str(second.trim(), "%d %B, %I:%M %p %Y")
            .unwrap();
        let duration = date - now;
        print_duration(duration, term, conky, first);
    }
}

// Retrieves the events from lmsScraperGo service and converts them
// into list of string events
fn get_events(events: &mut Vec<String>) {
    let mut data = [0u8; 1512];
    match TcpStream::connect("localhost:9977") {
        Ok(mut stream) => {
            let msg = b"events";
            stream.write_all(msg).unwrap();
            match stream.read(&mut data) {
                Ok(n) => {
                    if n == 0 {
                        std::process::exit(0x0000);
                    }
                    let reply: Vec<String> = from_utf8(&data[0..n])
                        .unwrap()
                        .lines()
                        .map(String::from)
                        .collect();
                    events.extend(reply);
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
}

// Get events from tasks file
fn get_tasks(tasks: &mut Vec<String>) {
    let task_file = FileBuffer::open("~/.tasks").unwrap();
    let tasks_string = String::from_utf8(task_file[..].to_vec()).expect("not valid UTF-8");
    let list_of_tasks: Vec<String> = tasks_string.lines().map(String::from).collect();
    tasks.extend(list_of_tasks);
}

fn add_tasks(task: &str, deadline: &str) {
    let mut task_writer = BufWriter::new(std::fs::File::create("~/.tasks").unwrap());
    match write!(task_writer, "{},{}", task, deadline){
        Err(e) => println!("{:?}",e),
        _ => ()
    };
}

fn add_tasks_list(tasks: &mut Vec<String>){
    if tasks.len() == 0{
        add_tasks("","")
    }
    for ele in tasks{
        let mut splitter = ele.splitn(2, ',');
        let task = splitter.next().unwrap();
        let deadline: String = splitter.next().unwrap().to_owned();
        println!("Removing {},{}",task,deadline);
        add_tasks(&task, &deadline);
    }
}
fn remove_tasks(task: &str, tasks: &mut Vec<String>) {
    let idx = tasks
        .iter()
        .filter(|x| x.split(',').next().unwrap() == task)
        .count();
    if idx != 0 {
        println!("No of Tasks Found containing {}: {}", task, idx);
        if idx > 1 {
            let query = yes_or_no("Remove All");
            if query {
                tasks.retain(|x| x.split(',').next().unwrap() != task);
                add_tasks_list(tasks);
                println!("Removed all tasks containing {}", task);
            }
            else{
                println!("Showing tasks containing {} up to 10",task);
                println!("Removing selected tasks");
            }
        }
    } else {
        println!("Not Found");
    }
}

fn yes_or_no<S: ToString>(prompt: S) -> bool {
    let mut buf = String::new();
    print!("{} [Y y] >> ", prompt.to_string());
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Could not get user input");
    buf.to_lowercase().trim() == "y"
}
// Prints the Acquired events in specified color format
// defaulting to no color output when not specified
fn print_duration(duration: chrono::Duration, term: bool, conky: bool, first: &str) {
    if duration.num_days() >= 2 {
        if term {
            println!(
                "\x1b[38;2;250;169;22m{},\x1b[0m remaining days to do are \x1b[38;2;104;161;223m{} days\x1b[0m",
                first,
                duration.num_days()
            );
        } else if conky {
            println!(
                "${{#FAA916}}{},${{#FBFFFE}} remaining days to do are ${{#68A1DF}}{} days${{#FBFFFE}}",
                first,
                duration.num_days()
                );
        } else {
            println!(
                "{}, remaining days to do are {} days",
                first,
                duration.num_days()
            );
        }
    } else if duration.num_days() >= 1 {
        if term {
            println!("{},\x1b[0m submit \x1b[0;31mToday\x1b[0m", first);
        } else if conky {
            println!(
                "${{#FAA916}}{},${{#FBFFFE}} to be done ${{#FF0000}}Today${{#FBFFFE}}",
                first
            );
        } else {
            println!("{}, to be done Today", first);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_events() {
        let mut events: Vec<String> = Vec::new();
        assert_eq!(get_events(&mut events), ());
    }
    #[test]
    fn test_not_connect() {
        assert!(true); // should implement latter
    }
    #[test]
    fn test_main() {
        assert_eq!(main(), ());
        let duration: chrono::Duration = chrono::Duration::days(1);
        assert_eq!(print_duration(duration, false, false, "first"), ());
    }

    #[test]
    fn test_term() {
        let duration: chrono::Duration = chrono::Duration::days(2);
        let term = true;
        let conky = false;
        let first: &str = "Testing ";
        assert_eq!(print_duration(duration, term, conky, first), ());
        let duration: chrono::Duration = chrono::Duration::days(1);
        assert_eq!(print_duration(duration, term, conky, first), ());
    }

    #[test]
    fn test_conky() {
        let duration: chrono::Duration = chrono::Duration::days(2);
        let term = false;
        let conky = true;
        let first: &str = "Testing ";
        assert_eq!(print_duration(duration, term, conky, first), ());
        let duration: chrono::Duration = chrono::Duration::days(1);
        assert_eq!(print_duration(duration, term, conky, first), ());
    }
}
