pub use std::net::SocketAddr;
pub use std::sync::Arc;
pub use std::{env, process};

pub use sosistab;
pub use {smol, smol::prelude::*};

pub use {
    rand_core,
    rand_core::{OsRng, RngCore},
};
pub use {
    x25519_dalek,
    x25519_dalek::{PublicKey, StaticSecret},
};

pub use async_recursion::async_recursion;
pub use dirs;

pub const VERSION: &str = "2.0.1";

pub async fn genkey() -> StaticSecret {
    let mut key: [u8; 32] = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    let key: StaticSecret = StaticSecret::from(key);
    return key;
}

#[async_recursion]
pub async fn takekey() -> StaticSecret {
    let home: String = dirs::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let keyfile: String = home + "/sosistun-private-key";

    let key: StaticSecret = match std::fs::read(&keyfile) {
        Ok(v) => {
            if v.len() != 32 {
                std::fs::remove_file(&keyfile).unwrap();
                return takekey().await;
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

pub async fn client(pk: PublicKey, listen: SocketAddr, remote: SocketAddr) {
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

        smol::spawn(
            smol::io::copy(sosistab_conn.clone(), tcp_conn.clone())
                .race(smol::io::copy(tcp_conn, sosistab_conn)),
        )
        .detach();
    }
}

pub async fn server(sk: StaticSecret, listen: SocketAddr, origin: SocketAddr) {
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

                smol::spawn(
                    smol::io::copy(sosistab_conn.clone(), tcp_conn.clone())
                        .race(smol::io::copy(tcp_conn, sosistab_conn)),
                )
                .detach();
            }
        })
        .detach();
    }
}
