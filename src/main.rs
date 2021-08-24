use sosistab;
use std::{env, process};
use std::net::SocketAddr;

fn server(listen: SocketAddr) {
    // XXX: doing...
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let mode:&str = match args.get(1) {
        Some(v) => v,
        None => {
            eprintln!("why you do not input the running mode?");
            process::exit(1);
        }
    };

    match mode {
        "client" => {},
        "server" => {
            let listen:&str = match args.get(2) {
                Some(v) => v,
                None => {
                    eprintln!("fxxk! you want me to run as a server, but why you do not give me a listen address???");
                    process::exit(3);
                }
            };

            let listen:SocketAddr = match listen.parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("are you an idiot? why not give me a right format of listen address???");
                    process::exit(4);
                }
            };
            println!("{:?}", listen);

        },
        _ => {
            eprintln!("why you input a wrong mode???");
            process::exit(2);
        }
    }
}
