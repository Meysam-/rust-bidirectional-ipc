use ipc_channel::ipc::{channel, IpcReceiver, IpcSender};

use std::env;

type ParentMsg = String;
type ClientMsg = String;

pub fn get_channel_name_arg() -> Option<String> {
    for arg in env::args() {
        let arg_str = "channel_name:";
        if let Some(arg) = arg.strip_prefix(arg_str) {
            return Some(arg.to_owned());
        }
    }
    None
}
fn main() {
    let (parent_tx, client_rx): (IpcSender<ParentMsg>, IpcReceiver<ClientMsg>) = channel().unwrap();
    let (client_tx, parent_rx): (IpcSender<ClientMsg>, IpcReceiver<ParentMsg>) = channel().unwrap();

    println!("Child process started");

    let one_shot_server_name = get_channel_name_arg().unwrap();
    println!("One Shot Server name: {}", one_shot_server_name);

    let tx0 = IpcSender::connect(one_shot_server_name).unwrap();
    tx0.send((parent_tx, parent_rx)).unwrap();

    loop {
        match client_rx.recv() {
            Ok(msg) => {
                println!("Received from parent: {}", msg);
                if msg == "quit" {
                    println!("Child process exiting.");
                    break;
                }
                // Here you can process the message and send a response back if needed
                client_tx.send(format!("Child received: {}", msg)).unwrap();
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
    println!("Child process finished.");
}
