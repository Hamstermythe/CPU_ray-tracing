use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::io::{self, BufRead};
use std::net::UdpSocket;
//use std::env;
use std::str;

pub fn read_server(socket: std::net::UdpSocket) -> std::io::Result<()> {

    let string = "teeesssstt";
    socket.send_to(string.as_bytes(), "192.168.1.30:3000")//hostname.to_string() + &":2000")
        .expect("Error on send");


    /*let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage {} hostname", args[0]);
        std::process::exit(1);
    }
    let hostname = &args[1];*/

    //let socket1 = UdpSocket::bind("[::]:0")?;  // for UDP4/6
    //let socket1 = UdpSocket::bind("10.10.3.179:8000")?;  // for UDP4/6
    //socket1.connect(hostname.to_string() + &":2000").expect("couldn't connect to address");

    // from https://stackoverflow.com/questions/30186037/how-can-i-read-a-single-line-from-stdin
    loop {
        let mut buf = [0; 2048];
        let (amt, _src) = socket.recv_from(&mut buf)?;
    
        let echo = str::from_utf8(&buf[..amt]).unwrap();
        //println!("Echo {}", echo);
        if echo == "BYE" {
            break;
        }
        //sender_server(socket, echo);
        //socket.send_to(&buf, "192.168.1.30:3000").expect("Error on send");
    }
    /*let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("Line read from stdin '{}'", line);
        if &line == "BYE" {
            break;
        }
        socket.send_to(line.as_bytes(), "10.10.3.179:3000")//hostname.to_string() + &":2000")
            .expect("Error on send");
    
        let mut buf = [0; 2048];
        let (amt, _src) = socket.recv_from(&mut buf)?;
    
        let echo = str::from_utf8(&buf[..amt]).unwrap();
        println!("Echo {}", echo);
    }*/
 
 
    println!("End of read_server");
    Ok(())
}

pub fn sender_server(socket: std::net::UdpSocket, msg: &str) -> std::io::Result<()> {
    socket.send_to(msg.as_bytes(), "192.168.1.30:3000")//hostname.to_string() + &":2000")
        .expect("Error on send");
    Ok(())
}



/*#[derive(Debug)]
pub struct UdpConnection {
    socket: std::net::UdpSocket,
    receiver: Receiver<String>,
    sender: Sender<String>,
}

impl UdpConnection {
    pub fn new() -> UdpConnection {
        let (sender, receiver) = mpsc::channel();
        let socket = UdpSocket::bind("192.168.1.30:3000").expect("couldn't bind to address");
        UdpConnection {
            socket,
            receiver,
            sender,
        }
    }
    pub fn start(&mut self) {
        let tx = self.sender.clone();
        let thread_result = thread::spawn(move || {
            tx.
        })
    }
}*/