use sosistab;
use std::{env, process};
use std::net::SocketAddr;

const VERSION:u128 = 0;

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
        "client" => {
            let listen:&str = match args.get(2) {
                Some(v) => v,
                None => {
                    eprintln!("fxxk. you want me to run as a client, but why you do not give me a listen address???");
                    process::exit(9);
                }
            };
            let listen:SocketAddr = match listen.parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("are you an idiot? why not give me a right format of listen address???");
                    process::exit(10);
                }
            };

            match args.get(3) {
                Some(v) => {
                    if v != "remote" {
                        eprintln!("maybe you really are an idiot. as I said, you can use \"remote\" as the next argument's value.");
                        process::exit(11);
                    };
                },
                None => {
                    eprintln!("I am a sosistab tunnel, not a client that like shadowsocks client. so why you do not give me a target address? what is the remote server? you should use me like a kcptun client, not like a proxy client.");
                    eprintln!("do you want me to help you? ok, you can use \"remote\" as the next argument's value.");
                    process::exit(12);
                }
            };

            let remote:&str = match args.get(4) {
                Some(v) => v,
                None => {
                    eprintln!("you are an idiot? as I said, what is the remote server? if you don't have one, what is the reason of using me?");
                    process::exit(13);
                }
            };
            let remote:SocketAddr = match remote.parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("why you give me a wrong format of remote server's address???");
                    process::exit(14);
                }
            };

            println!("still in developing..."); process::exit(0);

            //client(listen, );
        },
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
                    if v != "origin" {
                        eprintln!("maybe you really are an idiot. as I said, you can use \"origin\" as the next argument's value.");
                        process::exit(6);
                    };
                },
                None => {
                    eprintln!("I am a sosistab tunnel, not a server that like shadowsocks server. so why you do not give me a target address? what is the original server? you should use me like a kcptun server, not like a proxy server.");
                    eprintln!("do you want me to help you? ok, you can use \"origin\" as the next argument's value.");
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
