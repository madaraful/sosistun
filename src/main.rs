use sosistab;
use std::{env, process};
use std::net::SocketAddr;

fn server(listen: SocketAddr, origin: SocketAddr) {
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
                    eprintln!("fxxk. you want me to run as a server, but why you do not give me a listen address???");
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

            match args.get(3) {
                Some(v) => {
                    if v != "for" {
                        eprintln!("maybe you really are an idiot. as I said, you can use \"for\" as the next argument's value.");
                        process::exit(6);
                    };
                },
                None => {
                    eprintln!("I am a sosistab tunnel, not a server that like shadowsocks server. so why you do not give me a target address? what is the original server? you should use me like a kcptun server, not like a proxy server.");
                    eprintln!("do you want me to help you? ok, you can use \"for\" as the next argument's value.");
                    process::exit(5);
                }
            };

            let origin:&str = match args.get(4) {
                Some(v) => v,
                None => {
                    eprintln!("you are an idiot? as I said, what is the original server? if you don't have one, what is the reason of using me?");
                    process::exit(7);
                }
            };
            let origin:SocketAddr = match origin.parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("why you give me a wrong format of original server's address???");
                    process::exit(8);
                }
            };

            println!("still in developing..."); process::exit(0);

            //server(listen, );
        },
        _ => {
            eprintln!("why you input a wrong mode???");
            process::exit(2);
        }
    }
}
