use clap::Parser;
use zmq::{Context, Socket, DONTWAIT, PULL, PUSH, SNDMORE};

pub struct Duplicator {
    in_socket: Socket,
    out_sockets: Vec<(String, Socket)>,
    debug: bool,
}

impl Duplicator {
    pub fn new(in_address: &str, out_addresses: Vec<&str>, debug: bool) -> Duplicator {
        let zmq_context = Context::new();

        if debug {
            println!("binding to {} for incoming messages", in_address);
        }

        let in_socket = zmq_context.socket(PULL).unwrap();
        match in_socket.bind(in_address) {
            Ok(_) => {}
            Err(e) => panic!("Failed to bind incoming socket to {}: {}", in_address, e),
        }
        let out_sockets = out_addresses
            .into_iter()
            .map(|address| {
                if debug {
                    println!("binding to {} for outgoing messages", address);
                }
                let socket = zmq_context.socket(PUSH).unwrap();
                match socket.bind(address) {
                    Ok(_) => {}
                    Err(e) => panic!("Failed to bind outgoing socket to {}: {}", address, e),
                }
                (address.to_string(), socket)
            })
            .collect();
        Duplicator {
            in_socket,
            out_sockets,
            debug,
        }
    }

    fn run(&self) {
        loop {
            let mut msg = zmq::Message::new();

            // The flags are either DONTWAIT or SNDMORE.
            // SNDMORE is the only one that blocks, so we use that.
            if self.in_socket.recv(&mut msg, SNDMORE).is_ok() {
                if self.debug {
                    println!("Received: {:?}", msg.as_str());
                }
                for (socket_address, socket) in &self.out_sockets {
                    match socket.send(&*msg, DONTWAIT) {
                        Ok(_) => {}
                        Err(e) => eprintln!(
                            "Warning: Failed to send message with {} : {}",
                            socket_address, e
                        ),
                    }
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
