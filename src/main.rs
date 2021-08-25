#![feature(async_closure)]

// the libaries of using
use std;
use sosistab;
use x25519_dalek;
use rand_core;
use async_net;

use std::{env, process};
use std::net::SocketAddr;
use std::time::Duration;
use std::sync::Arc;

use rand_core::{RngCore, OsRng};
use x25519_dalek::{StaticSecret, PublicKey};

use futures_lite::prelude::*;
use futures_lite::AsyncRead;
use futures_lite::AsyncReadExt;

const VERSION:u8 = 3;

async fn genkey() -> StaticSecret {
    let mut key:[u8;32] = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    let key:StaticSecret = StaticSecret::from(key);
    return key;
}

async fn client(pk:PublicKey, listen:SocketAddr, remote:SocketAddr) {
    let tcp_server:async_net::TcpListener = async_net::TcpListener::bind(listen).await.unwrap();
    let mut tcp_in = tcp_server.incoming();

    loop {
        let tcp_conn = match tcp_in.next().await {
            Some(v) => v,
            None => { return; }
        };
        let mut tcp_conn = tcp_conn.unwrap();

        let gather:Arc<sosistab::StatsGatherer> = Arc::new(sosistab::StatsGatherer::new_active());
        let sosistab_client:sosistab::ClientConfig = sosistab::ClientConfig::new(sosistab::Protocol::DirectUdp, remote, pk, gather);
        let sosistab_conn = sosistab_client.connect().await.unwrap();
    
        tokio::spawn(async move {
            loop {
                let mut buf:[u8;65599] = [0u8; 65599];
                let mut data:Vec<u8> = Vec::new();
                match tokio::time::timeout(Duration::new(0, 1), tcp_conn.read(&mut buf)).await {
                    Ok(v) => {
                        let size = v.unwrap();
                        data.extend(&buf[..size]);
                        sosistab_conn.send_bytes(&data[..]).await;
                    },
                    Err(_e) => {}
                }

                // ==================================================

                match tokio::time::timeout(Duration::new(0, 1), sosistab_conn.recv_bytes()).await {
                    Ok(v) => {
                        let data = v.unwrap();
                        tcp_conn.write_all(&data).await;
                    },
                    Err(_e) => {}
                };
            };
        });
    };
}

async fn server(sk:StaticSecret, listen:SocketAddr, origin:SocketAddr) {
    // XXX: doing...

    let sosistab_server:sosistab::Listener = sosistab::Listener::listen_udp(listen, sk, |size:usize, peer:SocketAddr|{
        // on receive
        println!("receive");
    }, |size:usize, peer:SocketAddr|{
        // on send
        println!("send");
    }).await.unwrap();

    loop {
        let sosistab_conn:Option<sosistab::Session> = sosistab_server.accept_session().await;
        let sosistab_conn:sosistab::Session = match sosistab_conn {
            Some(v) => v,
            None => { continue; }
        };

        let mut tcp_client = match async_net::TcpStream::connect(origin).await {
            Ok(v) => v,
            Err(_e) => {
                drop(sosistab_conn);
                continue;
            }
        };

        tokio::spawn(async move {
            loop {
                let mut buf:[u8;65599] = [0u8; 65599];
                let mut data:Vec<u8> = Vec::new();
                match tokio::time::timeout(Duration::new(0, 1), tcp_client.read(&mut buf)).await {
                    Ok(v) => {
                        let size = v.unwrap();
                        data.extend(&buf[..size]);
                        sosistab_conn.send_bytes(&data[..]).await;
                    },
                    Err(_e) => {}
                };

                // =========================================================

                match tokio::time::timeout(Duration::new(0, 1), sosistab_conn.recv_bytes()).await {
                    Ok(v) => {
                        let data = v.unwrap();
                        tcp_client.write_all(&data).await;
                    },
                    Err(_e) => {}
                };
            }
        });
    }
}

#[tokio::main]
async fn main() {
    println!("== SosisTUN v{} ==", VERSION);

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
                Err(_e) => {
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
                    eprintln!("I am a sosistab tunnel, not a client like shadowsocks client. so why you do not give me a target? what is the remote server? you should use me like a kcptun client, not like a proxy client.");
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
                Err(_e) => {
                    eprintln!("why you give me a wrong format of remote server's address???");
                    process::exit(14);
                }
            };

            println!("still in developing..."); //process::exit(0);

            client(PublicKey::from([0u8;32]), listen, remote).await;
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
                Err(_e) => {
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
                    eprintln!("I am a sosistab tunnel, not a server like shadowsocks server. so why you do not give me a target? what is the original server? you should use me like a kcptun server, not like a proxy server.");
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
                Err(_e) => {
                    eprintln!("why you give me a wrong format of original server's address???");
                    process::exit(8);
                }
            };

            println!("still in developing..."); //process::exit(0);

            let key = genkey().await;
            println!("key: {:?}", key.to_bytes());
            server(key, listen, origin).await;
        },
        _ => {
            eprintln!("why you type a wrong mode???");
            process::exit(2);
        }
    }
}
