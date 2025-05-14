use std::net::{TcpListener, TcpStream};

use gumdrop::Options;
use log::DEBUG;

mod log;

#[derive(Debug, Options)]
struct Args {
    #[options(free)]
    free: Vec<String>,

    #[options(help = "print help message")]
    help: bool,

    #[options(
        help = "specify port that is exposed to the local network. Default: <local port + 1>"
    )]
    port: Option<u16>,

    #[options(help = "enable debug output")]
    debug: bool,

    #[options(help = "supress all output")]
    quiet: bool,
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    if args.help {
        println!("{}", Args::usage());
        return;
    }

    // unwrap: there is no way this could have already been initialized
    log::DEBUG.set(args.debug).unwrap();
    // unwrap: there is no way this could have already been initialized
    log::QUIET.set(args.quiet).unwrap();

    if *DEBUG.get().unwrap_or(&false) {
        debug!("Debug logging enabled");
    }

    let dst_port = args.free[0]
        .parse::<u16>()
        .expect("Provided port is not a valid number");
    let src_port = args.port.unwrap_or(dst_port + 1);

    info!(
        "Forwarding connections from port {} to port {}",
        src_port, dst_port
    );

    let listener =
        TcpListener::bind(format!("0.0.0.0:{src_port}")).expect("Failed to bind to port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(&dst_port, stream),
            Err(e) => {
                error!("Error accepting connection: {}", e);
                continue;
            }
        }
        .unwrap_or_else(|e| {
            error!("Error handling connection: {}", e);
        });
    }
}

fn handle_connection(
    dst_port: &u16,
    mut incoming: TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut local = TcpStream::connect(format!("127.0.0.1:{dst_port}"))?;

    info!(
        "Incoming connection from {}",
        incoming.local_addr().unwrap()
    );

    let mut incoming_clone = incoming.try_clone()?;
    let mut local_clone = local.try_clone()?;

    // Client -> Server
    std::thread::spawn(move || {
        match std::io::copy(&mut incoming, &mut local) {
            Ok(_) => {}
            Err(e) => {
                error!("Error forwarding data: {}", e);
            }
        };
        debug!("Client -> Server thread exiting");
    });

    // Server -> Client
    std::thread::spawn(move || {
        match std::io::copy(&mut local_clone, &mut incoming_clone) {
            Ok(_) => {}
            Err(e) => {
                error!("Error forwarding data: {}", e);
            }
        };
        debug!("Client -> Server thread exiting");
    });

    Ok(())
}
