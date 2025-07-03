use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use std::{
    io::Read,
    process::{Command, Stdio},
};

type ParentMsg = String;
type ClientMsg = String;

fn main() {
    println!("Parent process started");

    // Create an IPC one-shot server to receive the parent_tx, parent_rx to the child process
    // This allows the parent to connect back to the child and send/receive messages
    let (server0, server_name) = IpcOneShotServer::new().unwrap();
    println!("One Shot Server name - Parent: {}", server_name);

    let child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("child")
        .arg(format!("channel_name:{}", server_name))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run child process");

    let child_pid = child.id();
    println!("Child process ID: {}", child_pid);

    let (_, (tx_handle, rx_handle)): (_, (IpcSender<ParentMsg>, IpcReceiver<ClientMsg>)) =
        server0.accept().unwrap();

    // The channel is now established, and we can send messages to the child process

    let list_of_messages = vec![
        "Hello from parent".to_string(),
        "How are you?".to_string(),
        "Goodbye!".to_string(),
    ];

    for msg in list_of_messages {
        tx_handle.send(msg).unwrap();
        match rx_handle.recv() {
            Ok(response) => {
                println!("Received from child: {}", response);
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
    // Send a quit message to the child process
    tx_handle.send("quit".to_string()).unwrap();

    // print the standard output of the child process
    let output = child.wait_with_output().expect("Failed to wait on child");
    let mut buff = String::new();
    output.stdout.as_slice().read_to_string(&mut buff).unwrap();
    println!("--------------------- stdout: {}", buff);
    println!("-----------------------------------------")
}
