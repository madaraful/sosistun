mod lib;
use lib::*;

async fn async_main() {
    eprintln!("== SosisTUN v{} ==", VERSION);

    let args: Vec<String> = env::args().collect();
    let mode: &str = match args.get(1) {
        Some(v) => v,
        None => {
            eprintln!("爲什麼你不告訴我該如何運行？請在本程序的第一個參數中寫上運行模式。");
            //eprintln!("why you do not input the running mode?");
            process::exit(1);
        }
    };

    match mode {
        "client" => {
            let listen: &str = match args.get(2) {
                Some(v) => v,
                None => {
                    eprintln!("你既然想讓我作爲客戶端運行，那爲什麼不給我一個TCP監聽地址？請在本程序的第二個參數中寫上TCP監聽地址。有了這個地址後，該地址就能作爲你的TCP客戶端的訪問點（該地址是直接聯通 Sosistun Server 上的 origin 地址的，訪問該地址就等於訪問origin地址）");
                    //eprintln!("you want me to run as a client, but why you do not give me a listen address???");
                    process::exit(9);
                }
            };
            let listen: SocketAddr = match listen.parse() {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("爲什麼不給我一個正確的格式呢？TCP監聽地址的格式不正確，他應該是 IP:Port 的格式（例如 127.0.0.1:1234 ）。請檢查本程序的第二個參數是否輸入正確。");
                    //eprintln!("why not give me a right format of listen address???");
                    process::exit(10);
                }
            };

            match args.get(3) {
                Some(v) => {
                    if v != "remote" {
                        eprintln!("都說了讓你在第三個參數那裏寫 \"remote\" 了，爲什麼寫錯了？要用半角字符，而不是全角。請檢查本程序的第三個參數是否輸入正確。");
                        /*eprintln!(
                            "as I said, you can use \"remote\" as the next argument's value."
                        );*/
                        process::exit(11);
                    };
                }
                None => {
                    eprintln!("你需要給我一個 Sosistun 服務器的地址，不然我怎麼知道服務器的地址是什麼呢？請在本程序的第四個參數中寫上 Sosistun 服務器的地址。本程序會將第二個參數那裏的地址給轉發到 Sosistun 服務器的 origin 地址。而你需要提供給我： Sosistun服務器  的可訪問IP + Sosistun服務器程序  的第二個參數的端口。");
                    eprintln!("你應該使用 \"remote\" 作爲第三個參數的值。");
                    //eprintln!("I am a sosistab tunnel, not a client like shadowsocks client. so why you do not give me a target? what is the remote server? you should use me like a kcptun client, not like a proxy client.");
                    //eprintln!("do you want me to help you? ok, you can use \"remote\" as the next argument's value.");
                    process::exit(12);
                }
            };

            let remote: &str = match args.get(4) {
                Some(v) => v,
                None => {
                    eprintln!("你確實在第三個參數中寫上了 \"remote\"，但是你應該在第四個參數中寫上 Sosistun服務器 的地址。如果你沒有服務器地址，就無法使用我（畢竟瀏覽器如果沒有URL那該怎麼訪問網站呢？就是同樣的道理）");
                    //eprintln!("as I said, what is the remote server? if you don't have one, what is the reason of using me?");
                    process::exit(13);
                }
            };
            let remote: SocketAddr = match remote.parse() {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("爲什麼不給我一個正確的格式呢？Sosistun服務器 地址的格式不正確，他應該是 IP:Port 的格式（例如 127.0.0.1:1234 ）。請檢查本程序的第四個參數是否輸入正確。");
                    //eprintln!("why you give me a invalid format of remote server's address???");
                    process::exit(14);
                }
            };

            match args.get(5) {
                Some(v) => {
                    if v != "pubkey" {
                        eprintln!("都說了讓你在第五個參數那裏寫 \"pubkey\" 了，爲什麼寫錯了？要用半角字符，而不是全角。請檢查本程序的第五個參數是否輸入正確。");
                        //eprintln!("you made a mistake, please retype it again.");
                        process::exit(17);
                    }
                }
                None => {
                    eprintln!("good，你做的好，目前輸入的參數都是有效的格式。下一步，請給我 Sosistun服務器 的 public key（也就是公鑰），就輸入在第六個參數即可。");
                    eprintln!("你應該使用 \"pubkey\" 作爲第五個參數的值。");
                    /*eprintln!("good, you did it right. next step, give me a server's public key.");
                    eprintln!("you can use \"pubkey\" as next arguments.");*/
                    process::exit(16);
                }
            };

            let pubkey_hex: String = match args.get(6) {
                Some(v) => {
                    let v_len = v.len();
                    if (v_len % 2) != 0 {
                        eprintln!("公鑰的格式是 hex，請檢查本程序的第六個參數是否輸入正確。");
                        //eprintln!("the format of public key is hex string. please make sure you entered correct.");
                        process::exit(19);
                    };

                    let accept_chars: &str = "0123456789ABCDEF";

                    for it in v.chars() {
                        match accept_chars.find(it) {
                            Some(_) => {}
                            None => {
                                eprintln!("公鑰的格式是 hex，而且字母必須用大寫字母。請檢查本程序的第六個參數是否輸入正確。");
                                //eprintln!("the format of public key is hex string, and the letters must be capital. please make sure you entered correct.");
                                process::exit(20);
                            }
                        }
                    }

                    if v_len != (32 * 2) {
                        eprintln!(
                            "公鑰的長度必須是 32 字節，請檢查本程序的第六個參數是否輸入正確。"
                        );
                        //eprintln!("the length of public key is must be 32 bytes. please make sure you entered correct.");
                        process::exit(21);
                    }

                    v.to_string()
                }
                None => {
                    eprintln!("你不給我公鑰那我怎麼連接 Sosistun服務器 呢？請在本程序的第六個參數中寫上 Sosistun服務器 的公鑰。");
                    //eprintln!("please type the public key as the next argument's value.");
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
                    eprintln!("你既然想讓我作爲服務器運行，那爲什麼不給我一個 Sosistab監聽地址？請在本程序的第二個參數中寫上 Sosistab監聽地址。該地址對應於 Sosistun客戶端 的 remote 地址。");
                    //eprintln!("you want me to run as a server, but why you do not give me a listen address???");
                    process::exit(3);
                }
            };
            let listen: SocketAddr = match listen.parse() {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("爲什麼不給我一個正確的格式呢？Sosistab監聽地址 的格式不正確，他應該是 IP:Port 的格式（例如 127.0.0.1:1234 ）。請檢查本程序的第二個參數是否輸入正確。");
                    //eprintln!("why not give me a right format of listen address???");
                    process::exit(4);
                }
            };

            match args.get(3) {
                Some(v) => {
                    if v != "origin" {
                        eprintln!("都說了讓你在第三個參數那裏寫 \"origin\" 了，爲什麼寫錯了？要用半角字符，而不是全角。請檢查本程序的第三個參數是否輸入正確。");
                        /*
                        eprintln!(
                            "as I said, you can use \"origin\" as the next argument's value."
                        );
                        */
                        process::exit(6);
                    };
                }
                None => {
                    eprintln!("你需要給我一個 原始TCP服務器 的地址，不然我怎麼知道 TCP服務器 的地址是什麼呢？請在本程序的第四個參數中寫上 原始TCP服務器的地址。本程序會將第二個參數那裏的入站連接給轉發到 原始TCP服務器 的地址。而你需要提供給我：原始TCP服務器的可訪問IP和端口。");
                    eprintln!("你應該使用 \"origin\" 作爲第三個參數的值。");
                    //eprintln!("I am a sosistab tunnel, not a server like shadowsocks server. so why you do not give me a target? what is the original server? you should use me like a kcptun server, not like a proxy server.");
                    //eprintln!("do you want me to help you? ok, you can use \"origin\" as the next argument's value.");
                    process::exit(5);
                }
            };

            let origin: &str = match args.get(4) {
                Some(v) => v,
                None => {
                    eprintln!("你確實在第三個參數中寫上了 \"origin\"，但是你應該在第四個參數中寫上 原始TCP服務器 的地址。如果你沒有服務器地址，就無法使用我（畢竟 netcat 如果沒有地址那該怎麼訪問目標主機呢？就是同樣的道理）");
                    //eprintln!("as I said, what is the original server? if you don't have one, what is the reason of using me?");
                    process::exit(7);
                }
            };
            let origin: SocketAddr = match origin.parse() {
                Ok(v) => v,
                Err(_e) => {
                    eprintln!("爲什麼不給我一個正確的格式呢？原始TCP服務器 地址的格式不正確，他應該是 IP:Port 的格式（例如 127.0.0.1:1234 ）。請檢查本程序的第四個參數是否輸入正確。");
                    //eprintln!("why you give me a invalid format of original server's address???");
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
                "你的客戶端程序可以使用這個 public key（也就是公鑰） 來連接本服務器：{}",
                &pubkey_hex
            );
            /*
            println!(
                "Your clients can use this public key to connect this server: {}",
                &pubkey_hex
            );*/
            server(key, listen, origin).await;
        }
        _ => {
            eprintln!(
                "爲什麼你給我一個無效的運行模式？運行模式不是 client 就是 server，沒有第三個模式。"
            );
            //eprintln!("why you type a invalid mode???");
            process::exit(2);
        }
    }
}

fn main() {
    smol::block_on(async_main());
}
