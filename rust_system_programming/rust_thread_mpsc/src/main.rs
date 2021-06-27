use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmitter1, receiver) = mpsc::channel();
    let transmitter2 = mpsc::Sender::clone(&transmitter1);
    thread::spawn(move || {
        let num_vec: Vec<String> = vec![
            "One".to_string(),
            "Two".to_string(),
            "Three".to_string(),
            "Four".to_string(),
        ];
        for num in num_vec {
            transmitter1.send(num).unwrap();
        }
    });
    thread::spawn(move || {
        let num_vec: Vec<String> = vec![
            "One".to_string(),
            "Two".to_string(),
            "Three".to_string(),
            "Four".to_string(),
        ];
        for num in num_vec {
            transmitter2.send(num).unwrap();
        }
    });

    for received_val in receiver {
        println!("Received from thread: {}", received_val)
    }
}
