use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    // listen for new connections
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    // create a map of address::name
    let names = Arc::new(RwLock::new(HashMap::new()));

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        // add a new empty name to the map
        let mut map = names.write().unwrap();
        map.insert(addr.to_string(), Mutex::<Option<String>>::new(None));
        drop(map);

        let data = Arc::clone(&names);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // spawn a new task for each socket
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            // ask each user its name
            writer.write_all(b"Name?\n").await.unwrap();

            // handles reading or writing
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        let map = data.write().unwrap();
                        // get the name of the sender
                        if let Some(name) = map.get(&addr.to_string()) {
                            let mut name = name.lock().unwrap();

                            match &*name {
                                // if set, send their message
                                Some(n) => {
                                    tx.send((n.clone(), line.clone(), addr)).unwrap();
                                }
                                // otherwise use set their name
                                None => {
                                    let mut tmp_name = line.clone();
                                    // remove /r/n
                                    let len = tmp_name.len();
                                    tmp_name.truncate(len - 2);
                                    *name = Some(tmp_name);
                                }
                            }
                        }
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (name, msg, other_addr) = result.unwrap();

                        // send the message to all users, prefixed with the name
                        if addr != other_addr {
                            let msg = name + ": " +&msg;
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
