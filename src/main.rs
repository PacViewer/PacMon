use libp2p::{dns, noise, swarm::NetworkBehaviour, tcp, yamux, Multiaddr, PeerId, Swarm};
use std::thread;
use uuid::Uuid;
mod get;
mod read_pactus_nodes;
use read_pactus_nodes::{get_pactus_nodes, Node};
#[async_std::main]
async fn main() {
    let nodes = get_pactus_nodes();
    let s = channel(nodes);
}

fn channel(nodes: Vec<Node>) -> Vec<thread::JoinHandle<()>> {
    let channel: pub_sub::PubSub<String> = pub_sub::PubSub::new();

    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..54 {
        let recv = channel.subscribe();

        handles.push(thread::spawn(move || {
            for _ in 0..54 {
                let res: String = recv.recv().unwrap();
                let complete_address = res.parse::<Multiaddr>().unwrap();
                // let parts: Vec<&str> = res.split("/p2p").collect();
                // let modified_url = parts[0];
                // let dst_addr = modified_url.parse::<Multiaddr>().expect("invalidUrl");
                // let result0 = async_std::task::block_on(get::get_data(dst_addr));
                let result1 = async_std::task::block_on(get::get_data(complete_address));
                println!("complete_address : {:?} =>{:?}", res, result1)
            }
        }));
    }

    for node in nodes {
        let channel = channel.clone();

        handles.push(thread::spawn(move || {
            let msg_id: Uuid = Uuid::new_v4();
            println!("    sent {}", msg_id);
            channel.send(node.address).unwrap();
        }));
    }

    while let Some(handle) = handles.pop() {
        handle.join().unwrap();
    }
    return handles;
}
