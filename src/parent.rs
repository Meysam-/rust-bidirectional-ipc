use ipc_channel::ipc::{IpcOneShotServer, IpcReceiver, IpcSender};
use std::process::{Command, Stdio};

type ParentMsg = String;
type ClientMsg = String;

fn main() {
    println!("Parent process started");

    // Create an IPC one-shot server to receive the parent_tx, parent_rx to the child process
    // This allows the parent to connect back to the child and send/receive messages
    let (server0, server_name) = IpcOneShotServer::new().unwrap();
    println!("One Shot Server name - Parent: {}", server_name);

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("child")
        .env("CHANNEL_NAME", &server_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
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

    // Wait for the child process to finish
    let exit_code = child.wait().expect("Failed to wait on child");
    if exit_code.success() {
        println!("Child process exited successfully.");
    } else {
        eprintln!("Child process exited with an error.");
    }
}
