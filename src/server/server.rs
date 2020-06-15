use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::RwLock;
use std::thread::spawn;
use std::time::Duration;
use tungstenite::handshake::server::{Request, Response};
use tungstenite::protocol::WebSocket;
use tungstenite::server::accept;
use tungstenite::{accept_hdr, connect, Message};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
};

pub struct SocketMap {
    socket_ids: HashMap<u32, HashMap<u32, WebSocket<TcpStream>>>, //first: computation id ; second:party id
    computation_ids: HashMap<SocketAddr, u32>,
    //computation_ids: HashMap<WebSocket<TcpStream>, u32>,
    party_ids: HashMap<SocketAddr, u32>, // party id
}

pub struct Server {
    pub name: String,
    //socketMap: SocketMap,
}

impl Server {
    pub fn on(&self) {
        let mut socket_map = SocketMap {
            socket_ids : HashMap::new(),
            computation_ids : HashMap::new(),
            party_ids :HashMap::new(),
        };
        
        let socket_map = Arc::new(RwLock::new(socket_map));

        let server = TcpListener::bind("127.0.0.1:9001").unwrap();
        //let shared_message = Arc::new(Mutex::new(String::from("")));
        //let websockets_hashmap = Arc::new(RwLock::new(HashMap::new()));
        let counter = Arc::new(Mutex::new(0));
        

        while let Ok((stream, addr)) = server.accept() {
            
            //let websockets_hashmap = Arc::clone(&websockets_hashmap);
            let socket_map = Arc::clone(&socket_map);
            let counter = Arc::clone(&counter);

            spawn(move || {
                println!("new thread!");
                let id;
                {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                    println!("Received: {}", &num);
                    id = num.clone();
                }
                let websocket = accept(stream).unwrap();

                //build SocketMap
                {
                    let mut socket_map = socket_map.write().unwrap();
                    if id == 1 || id == 2 {
                        socket_map.computation_ids.insert(addr, 1);
                    } else {
                        socket_map.computation_ids.insert(addr, 2);
                    }
                    socket_map.party_ids.insert(addr, id);
                    let computation_id = socket_map.computation_ids.get(&addr).unwrap();
                    let computation_id = *computation_id;
                    let mut socket_ids = &mut socket_map.socket_ids;
                    
                    socket_ids.entry(computation_id).or_insert(HashMap::new()).insert(id, websocket);
                    
                    
                    //println!("{:?}", socket_ids.get(&1).unwrap().get(&id));
                }

                let mut planner = periodic::Planner::new();
                planner.add(
                    move || {//let cur_websocket;//:
                        let mut socket_map = socket_map.write().unwrap();
                        let computation_id = socket_map.computation_ids.get(&addr).unwrap();
                        let computation_id = *computation_id;
                        let cur_websocket: &mut tungstenite::protocol::WebSocket<std::net::TcpStream> =
                            socket_map.socket_ids.get_mut(&computation_id).unwrap().get_mut(&id).unwrap();
                        let msg = cur_websocket.read_message().unwrap();
    
                        println!("Received: {}", msg);
                        let cur_message = msg.to_string();
    
                        

                        let broadcast_recipients = &mut socket_map.socket_ids.get_mut(&computation_id).unwrap().iter_mut().map(|(_, socket)| socket);
                        for recp in broadcast_recipients {
                            //recp.write_message(Message::Text((*(message.clone())).to_string())).unwrap();
                            recp.write_message(Message::Text(cur_message.clone()))
                                .unwrap();
                        }
                    },
                    periodic::Every::new(Duration::from_secs(5)),
                );
                planner.start();

                
            });
        }
    }
}
