use clap::Parser;
use zmq::{Context, Socket, PULL, PUSH, SNDMORE};
use std::thread;
use crossbeam_channel::unbounded;

pub struct Duplicator {
    in_socket: Socket,
    out_threads: Vec<(crossbeam_channel::Sender<Vec<u8>>, thread::JoinHandle<()>)>,
    debug: bool,
}

impl Duplicator {
    pub fn new(in_address: &str, out_addresses: Vec<&str>, debug: bool) -> Duplicator {
        let zmq_context = Context::new();
        let in_socket = zmq_context.socket(PULL).unwrap();
        match in_socket.bind(in_address) {
            Ok(_) => {
                if debug {
                    println!("binding to {} for incoming messages", in_address);
                }
            }
            Err(e) => panic!("Failed to bind incoming socket to {}: {}", in_address, e),
        }

        let mut out_threads = Vec::new();

        for address in out_addresses {
                let (s, r) = unbounded();
                let outbound_receiver: crossbeam_channel::Receiver<Vec<u8>> = r;
                
                let zmq_context = Context::new();
                let socket = zmq_context.socket(PUSH).unwrap();

                match socket.bind(address) {
                    Ok(_) => {
                        if debug {
                            println!("started thread. binding to {} for outgoing messages", address);
                        }        
                    }
                    Err(e) => panic!("Failed to bind outgoing socket to {}: {}", address, e),
                }
                
                let handle = thread::spawn(move || {
                    loop {
                        let msg: Vec<u8> = match outbound_receiver.recv() {
                            Ok(msg) => msg,
                            Err(_) => continue,
                        };

                        match socket.send(&*msg, 0) {
                            Ok(_) => {},
                            Err(e) => {
                                eprintln!("Warning: Failed to send message: {}", e);
                            }
                        }
                    }
                });
  
                out_threads.push((s, handle));
        }

        Duplicator {
            in_socket,
            out_threads,
            debug,
        }
    }

    fn run(&self) {
        loop {
            // The flags are either DONTWAIT or SNDMORE.
            // SNDMORE is the only one that blocks, so we use that.
            let msg_result = self.in_socket.recv_bytes(SNDMORE);
            let msg = match msg_result {
                Ok(msg) => msg,
                Err(_) => continue,
            };

            if self.debug {
                println!("Received: {:?}", msg);
            }

            for (s, _h) in &self.out_threads {
                match s.send(msg.clone()) {
                    Ok(_) => {},
                    Err(e) => eprintln!(
                        "Warning: Failed to send message: {}", e
                    ),
                }
            }
        }
    }
}

/// Responsible for ingesting messages from multiple sources and
/// duplicating them to other components.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Message socket for incoming messages
    #[arg(long, default_value = "tcp://0.0.0.0:14302")]
    message_socket: String,

    /// One or more message sockets for outgoing messages
    #[arg(default_value = "tcp://0.0.0.0:14304")]
    out_sockets: Vec<String>,

    /// Debug mode
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let duplicator = Duplicator::new(
        args.message_socket.as_str(),
        args.out_sockets.iter().map(|s| s.as_str()).collect(),
        args.debug,
    );

    duplicator.run();
}
