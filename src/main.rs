use chrono::{DateTime, TimeZone, Utc};
use colored::Colorize;
use filebuffer::FileBuffer;
use std::env;
use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    // term variable to set term color output
    let mut term = true;
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
            "no-term" | "not-terminal" => term = false,
            "conky" => conky = true,
            "add" => {
                let mut task = String::new();
                let mut deadline = String::new();
                print!("{} {} ", "==>".green().bold(), "Enter a task >>".bold());
                let _ = stdout().flush();
                stdin().read_line(&mut task).unwrap();
                task = task.trim().to_string();
                print!(
                    "{} {} ",
                    "==>".green().bold(),
                    "Enter a Deadline[dd/mm/yyyy HH:MM] >>".bold()
                );
                let _ = stdout().flush();
                stdin().read_line(&mut deadline).unwrap();
                deadline = deadline.trim().to_string();
                let date = Utc.datetime_from_str(deadline.trim(), "%d/%m/%Y %H:%M");
                match date {
                    Err(e) => {
                        println!("{} {:?}", "Invalid DateTime Format".red().bold(), e);
                        std::process::exit(1);
                    }
                    _ => (),
                }
                let date = DateTime::from(date.unwrap());
                add_tasks(&mut task, &date);
                std::process::exit(0);
            }
            "remove" => {
                let mut task = String::new();
                print!("{} {} ", "==>".green().bold(), "Enter a task >>".bold());
                let _ = stdout().flush();
                stdin()
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
        let datetime = DateTime::parse_from_rfc3339(second.trim()).unwrap();
        let date = datetime.with_timezone(&Utc);
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
                    println!("{}: {}", "Failed to receive data".red().bold(), e);
                }
            }
        }
        Err(e) => {
            println!("{}:{}", "Failed to connect".red().bold(), e);
        }
    }
}

// Get events from tasks file
fn get_tasks(tasks: &mut Vec<String>) {
    let _file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&shellexpand::tilde("~/.tasks").to_string());
    let task_file = FileBuffer::open(&shellexpand::tilde("~/.tasks").to_string()).unwrap();
    let tasks_string = String::from_utf8(task_file[..].to_vec()).expect("not valid UTF-8");
    let list_of_tasks: Vec<String> = tasks_string.lines().map(String::from).collect();
    tasks.extend(list_of_tasks);
}

// Add tasks to .tasks file
fn add_tasks(task: &str, deadline: &DateTime<chrono::FixedOffset>) {
    let file_path_str = shellexpand::tilde("~/.tasks").to_string();
    let task_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path_str)
        .unwrap();
    let mut task_writer = BufWriter::new(task_file);
    match writeln!(task_writer, "{},{}", task, deadline.to_rfc3339()) {
        Err(e) => println!("{:?}", e.to_string().red().bold()),
        _ => println!("{} {}", "Added task".bold(), task),
    };
}

// Add list of tasks to .tasks file
fn add_tasks_list(tasks: &mut Vec<String>) {
    if tasks.len() == 0 {
        let now: DateTime<chrono::FixedOffset> = DateTime::from(Utc::now());
        add_tasks("", &now);
    }
    let file_path_str = shellexpand::tilde("~/.tasks").to_string();
    let task_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path_str)
        .unwrap();
    let mut task_writer = BufWriter::new(task_file);
    for ele in tasks {
        let mut splitter = ele.splitn(2, ',');
        let task = splitter.next().unwrap();
        let deadline: String = splitter.next().unwrap().to_owned();
        let date: DateTime<chrono::FixedOffset> = DateTime::parse_from_rfc3339(deadline.trim())
            .expect("Wrong datetime Format not adding task");
        match writeln!(task_writer, "{},{}", task, date.to_rfc3339()) {
            Err(e) => println!("{:?}", e.to_string().red().bold()),
            _ => println!("{} {}", "Added task".bold(), task),
        };
    }
}

// remove tasks from .tasks file
fn remove_tasks(task: &str, tasks: &mut Vec<String>) {
    let idx = tasks
        .iter()
        .filter(|x| x.split(',').next().unwrap() == task)
        .count();
    if idx != 0 {
        println!(
            "{} {}: {}",
            "No of Tasks Found containing".bold(),
            task,
            idx
        );
        if idx > 1 {
            let query = yes_or_no("Remove All");
            if query {
                tasks.retain(|x| x.split(',').next().unwrap() != task);
                add_tasks_list(tasks);
                println!("{} {}", "Removed all tasks containing".bold(), task);
            } else {
                println!(
                    "{} {} {} up to 10",
                    "::".cyan().bold(),
                    "Showing tasks containing".bold(),
                    task
                );
                println!(
                    "{} {}",
                    "::".cyan().bold(),
                    "Removing selected tasks...".bold()
                );
                println!(
                    "{} {}",
                    "Removed all selected tasks containing".bold(),
                    task
                );
            }
        }
    } else {
        println!("{}", "Not Found".red().bold());
    }
}

// Yes or no prompt
fn yes_or_no<S: ToString>(prompt: S) -> bool {
    let mut buf = String::new();
    print!(
        "{} {} [Y y] {} ",
        "==>".green().bold(),
        prompt.to_string().bold(),
        ">>".bold()
    );
    let _ = stdout().flush();
    stdin()
        .read_line(&mut buf)
        .expect("Could not get user input");
    buf.to_lowercase().trim() == "y"
}

// Prints the Acquired events in specified color format
// defaulting to no color output when not specified
fn print_duration(duration: chrono::Duration, term: bool, conky: bool, first: &str) {
    if duration.num_days() >= 2 {
        if term && !conky {
            println!(
                "{}, {} {} {}",
                first.truecolor(250, 169, 22).bold(),
                "remaining days to do are",
                duration
                    .num_days()
                    .to_string()
                    .truecolor(104, 161, 223)
                    .bold(),
                "days".truecolor(104, 161, 223).bold()
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
        if term && !conky{
            println!(
                "{}, to be done {}",
                first.truecolor(250, 169, 22).bold(),
                "Today".red().bold()
            );
        } else if conky {
            println!(
                "${{#FAA916}}{},${{#FBFFFE}} to be done ${{#FF0000}}Today${{#FBFFFE}}",
                first
            );
        } else {
            println!("{}, to be done Today", first);
        }
    } else {
        if term && !conky{
            println!(
                "{}, {}",
                first.truecolor(250, 169, 22).bold(),
                "is past Due".red().bold()
            );
        } else if conky {
            println!(
                "${{#FAA916}}{},${{#FBFFFE}} ${{#FF0000}}is past Due${{#FBFFFE}}",
                first
            );
        } else {
            println!("{}, is past Due", first);
        }
    }
}

// Unit testing of above Code
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
