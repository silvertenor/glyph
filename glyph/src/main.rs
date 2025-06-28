use glyph::ThreadPool;
use std::io::{self, Write};
use std::thread;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn get_user_input() {
    loop {
        // Clear screen and prompt for contact
        print!("\x1b[2J\x1b[H");
        print!("Who would you like to send a message to? ");
        io::stdout().flush().unwrap();

        let mut contact = String::new();
        io::stdin()
            .read_line(&mut contact)
            .expect("Failed to read line");
        let contact = contact.trim();
        get_and_send_message(&String::from(contact));
    }
}
fn get_and_send_message(contact: &String) {
    // Prompt for multi-line message input
    println!("Enter the message to send:");
    println!("Type your message. When done, enter a line with just /send to finish.");

    let mut message_lines = Vec::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line_trimmed = line.trim_end();

        if line_trimmed == "/send" {
            break;
        }

        message_lines.push(line);
    }

    let message = message_lines.concat();

    println!("\nMessage to '{}':\n{}", contact, message);

    println!("Press Enter to continue...");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
    send_message(&String::from(contact), &message);
}
fn send_message(contact: &String, message: &String) {
    let mut socket = TcpStream::connect(contact).unwrap();
    socket.write(message.as_bytes()).unwrap();
}
fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let message: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Message from {:?}", stream.peer_addr().unwrap());
    println!("Message:");
    for line in message {
        println!("{line}");
    }

    println!("\nWould you like to reply? (y/n): ");
    // let mut reply = String::new();
    // io::stdin()
    //     .read_line(&mut reply)
    //     .expect("Failed to read line");
    // let reply = reply.trim();
    // print!("{reply}");
    // io::stdout().flush();
    // if reply == "y" {
    //     println!("YAY");
    // } else {
    //     println!("DAMN");
    // }
}

fn main() {
    print!("\x1b[2J\x1b[H");
    thread::spawn(|| get_user_input());
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
