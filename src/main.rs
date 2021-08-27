// the libaries of using
use rand_core;
use smol;
use sosistab;
use std;
use x25519_dalek;

use std::net::SocketAddr;
use std::sync::Arc;
use std::{env, process};

use rand_core::{OsRng, RngCore};
use x25519_dalek::{PublicKey, StaticSecret};

use smol::prelude::*;

const VERSION: &str = "1.0.0";

async fn genkey() -> StaticSecret {
    let mut key: [u8; 32] = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    let key: StaticSecret = StaticSecret::from(key);
    return key;
}

async fn takekey() -> StaticSecret {
    let me: String = env::current_exe()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let keyfile: String = me + ".sosistun-private-key";

    let key: StaticSecret = match std::fs::read(&keyfile) {
        Ok(v) => {
            if v.len() != 32 {
                eprintln!("sorry, the private key is stored as a file on your disk, but that file's value is invalid and not recoverable. so delete it is the only way to restore your server, if you deleted it and start me again, I will generate a new private key and you need to reconfigure your client (make your client use the new public key)");
                eprintln!("the file location of private key is {}", &keyfile);
                process::exit(15);
            }

            let mut k: [u8; 32] = [0u8; 32];
            for i in 0..32 {
                k[i] = v[i];
            }

            StaticSecret::from(k)
        }
        Err(_e) => {
            let key: StaticSecret = genkey().await;
            let keyfile_value: [u8; 32] = key.to_bytes();
            std::fs::write(&keyfile, keyfile_value).unwrap();
            key
        }
    };

    return key;
}

async fn client(pk: PublicKey, listen: SocketAddr, remote: SocketAddr) {
    let sosistab_stats: Arc<sosistab::StatsGatherer> =
        Arc::new(sosistab::StatsGatherer::new_active());
    let sosistab_client: sosistab::ClientConfig =
        sosistab::ClientConfig::new(sosistab::Protocol::DirectUdp, remote, pk, sosistab_stats);

    let sosistab_conn: sosistab::Session = sosistab_client.connect().await.unwrap();
    let sosistab_conn: sosistab::Multiplex = sosistab_conn.multiplex();

    let tcp_server: smol::net::TcpListener = smol::net::TcpListener::bind(listen).await.unwrap();
    let mut tcp_in = tcp_server.incoming();

    loop {
        let tcp_conn: smol::net::TcpStream = tcp_in.next().await.unwrap().unwrap();
        let sosistab_conn: sosistab::RelConn = sosistab_conn.open_conn(None).await.unwrap();

        smol::spawn(smol::io::copy(sosistab_conn.clone(), tcp_conn.clone())).detach();
        smol::spawn(smol::io::copy(tcp_conn, sosistab_conn)).detach();
    }
}

async fn server(sk: StaticSecret, listen: SocketAddr, origin: SocketAddr) {
    let sosistab_server: sosistab::Listener = sosistab::Listener::listen_udp(
        listen,
        sk,
        |_size: usize, _peer: SocketAddr| { /* on receive */ },
        |_size: usize, _peer: SocketAddr| { /* on send */ },
    )
    .await
    .unwrap();

    loop {
        let sosistab_conn: sosistab::Session = sosistab_server.accept_session().await.unwrap();
        let sosistab_conn: sosistab::Multiplex = sosistab_conn.multiplex();

        smol::spawn(async move {
            loop {
                let sosistab_conn: sosistab::RelConn = sosistab_conn.accept_conn().await.unwrap();
                let tcp_conn = smol::net::TcpStream::connect(origin).await.unwrap();

                smol::spawn(smol::io::copy(sosistab_conn.clone(), tcp_conn.clone())).detach();
                smol::spawn(smol::io::copy(tcp_conn, sosistab_conn)).detach();
            }
        })
        .detach();
    }
}

async fn async_main() {
    eprintln!("== SosisTUN v{} ==", VERSION);

    let args: Vec<String> = env::args().collect();
    let mode: &str = match args.get(1) {
        Some(v) => v,
        None => {
            eprintln!("why you do not input the running mode?");
            process::exit(1);
        }
    };

    match mode {
        "client" => {
            let listen: &str = match args.get(2) {
                Some(v) => v,
                None => {
                    eprintln!("you want me to run as a client, but why you do not give me a listen address???");
                    process::exit(9);
                }
            };
            let listen: SocketAddr = match listen.parse() {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("why not give me a right format of listen address???");
                    process::exit(10);
                }
            };

            match args.get(3) {
                Some(v) => {
                    if v != "remote" {
                        eprintln!(
                            "as I said, you can use \"remote\" as the next argument's value."
                        );
                        process::exit(11);
                    };
                }
                None => {
                    eprintln!("I am a sosistab tunnel, not a client like shadowsocks client. so why you do not give me a target? what is the remote server? you should use me like a kcptun client, not like a proxy client.");
                    eprintln!("do you want me to help you? ok, you can use \"remote\" as the next argument's value.");
                    process::exit(12);
                }
            };

            let remote: &str = match args.get(4) {
                Some(v) => v,
                None => {
                    eprintln!("as I said, what is the remote server? if you don't have one, what is the reason of using me?");
                    process::exit(13);
                }
            };
            let remote: SocketAddr = match remote.parse() {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("why you give me a wrong format of remote server's address???");
                    process::exit(14);
                }
            };

            match args.get(5) {
                Some(v) => {
                    if v != "pubkey" {
                        eprintln!("you made a mistake, please retype it again.");
                        process::exit(17);
                    }
                }
                None => {
                    eprintln!("good, you did it right. next step, give me a server's public key.");
                    eprintln!("you can use \"pubkey\" as next arguments.");
                    process::exit(16);
                }
            };

            let pubkey_hex: String = match args.get(6) {
                Some(v) => {
                    let v_len = v.len();
                    if (v_len % 2) != 0 {
                        eprintln!("the format of public key is hex string. please make sure you entered correct.");
                        process::exit(19);
                    };

                    let accept_chars: &str = "0123456789ABCDEF";

                    for it in v.chars() {
                        match accept_chars.find(it) {
                            Some(_) => {}
                            None => {
                                eprintln!("the format of public key is hex string, and the letters must be capital. please make sure you entered correct.");
                                process::exit(20);
                            }
                        }
                    }

                    if v_len != (32 * 2) {
                        eprintln!("the length of public key is must be 32 bytes. please make sure you entered correct.");
                        process::exit(21);
                    }

                    v.to_string()
                }
                None => {
                    eprintln!("please type the public key as the next argument's value.");
                    process::exit(18);
                }
            };

            let pubkey: [u8; 32] = {
                let mut out: [u8; 32] = [0u8; 32];
                let mut now: usize = 0;
                for i in (0..(pubkey_hex.len())).step_by(2) {
                    let it: &str = &pubkey_hex[i..=i + 1];
                    let it: u8 = u8::from_str_radix(it, 16).unwrap();

                    out[now] = it;
                    now += 1;
                }

                out
            };

            client(PublicKey::from(pubkey), listen, remote).await;
        }
        "server" => {
            let listen: &str = match args.get(2) {
                Some(v) => v,
                None => {
                    eprintln!("you want me to run as a server, but why you do not give me a listen address???");
                    process::exit(3);
                }
            };
            let listen: SocketAddr = match listen.parse() {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("why not give me a right format of listen address???");
                    process::exit(4);
                }
            };

            match args.get(3) {
                Some(v) => {
                    if v != "origin" {
                        eprintln!(
                            "as I said, you can use \"origin\" as the next argument's value."
                        );
                        process::exit(6);
                    };
                }
                None => {
                    eprintln!("I am a sosistab tunnel, not a server like shadowsocks server. so why you do not give me a target? what is the original server? you should use me like a kcptun server, not like a proxy server.");
                    eprintln!("do you want me to help you? ok, you can use \"origin\" as the next argument's value.");
                    process::exit(5);
                }
            };

            let origin: &str = match args.get(4) {
                Some(v) => v,
                None => {
                    eprintln!("as I said, what is the original server? if you don't have one, what is the reason of using me?");
                    process::exit(7);
                }
            };
            let origin: SocketAddr = match origin.parse() {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("why you give me a wrong format of original server's address???");
                    process::exit(8);
                }
            };

            let key = takekey().await;

            let pubkey = PublicKey::from(&key);
            let pubkey_bytes = pubkey.to_bytes();

            let pubkey_hex: String = {
                let mut out: String = String::new();

                for i in 0..32 {
                    let it = pubkey_bytes[i];
                    let it: String = format!("{:02X}", it);
                    out.extend(it.chars());
                }

                out
            };
            assert!((pubkey_hex.len() % 2) == 0);

            println!(
                "Your clients can use this public key to connect this server: {}",
                &pubkey_hex
            );
            server(key, listen, origin).await;
        }
        _ => {
            eprintln!("why you type a wrong mode???");
            process::exit(2);
        }
    }
}

fn main() {
    smol::block_on(async_main());
}
