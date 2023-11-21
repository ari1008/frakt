use std::io::Write;
use std::net::TcpStream;
use crate::message::Message;

pub fn buffer_to_object(message_buf: &mut Vec<u8>) -> Message {
    let message = std::str::from_utf8(&message_buf).expect("failed to parse message");

    let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
    record
}

pub fn send_message(mut stream: &TcpStream, message: Message) {
    let serialized = serde_json::to_string(&message).expect("failed to serialized object");
    let serialized_size = serialized.len() as u32;
    println!("{}", serialized);
    println!("{}", serialized_size);

    stream
        .write_all(&serialized_size.to_be_bytes())
        .expect("failed to send message size");
    stream
        .write_all(&serialized.as_bytes())
        .expect("failed to send message");
}