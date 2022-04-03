use core_time::Timer;

use crate::PacketBuffer;

use super::{
    streams::{init_stream, poll},
    Secret, State, SERVER_ADDR,
};
use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
    sync::mpsc::{channel, Receiver, Sender},
};

fn accept_clients(listener: TcpListener, event_sender: Sender<TcpEvent>, timer: Timer) {
    for stream in listener.incoming() {
        let sender = event_sender.clone();
        match stream {
            Ok(client) => {
                std::thread::spawn(move || handle_client(client, sender, timer));
            }
            Err(e) => println!("Error connecting: {:?}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream, event_sender: Sender<TcpEvent>, mut timer: Timer) {
    if init_stream(&mut stream).success {
        let mut state = State::Receive;
        let mut message_queue = vec![];

        // Read in secret
        let secret = poll(&mut stream, &mut state, &mut message_queue);

        if secret.reconnect {
            return;
        }

        let secret = match secret.msg {
            Some(secret) => Secret {
                secret: String::from_utf8(secret.data().to_vec()).unwrap(),
            },
            None => return,
        };

        // Send event with communication channels

        let (sender, packet_receiver) = channel();
        let (thread_sender, receiver) = channel();

        event_sender
            .send(TcpEvent::ClientConnected {
                secret: secret.clone(),
                sender,
                receiver,
            })
            .unwrap();

        // We'll start the state in Send mode since we already read in the secret

        // Start main loop
        loop {
            if timer.tick().triggered {
                for msg in packet_receiver.try_iter() {
                    message_queue.push(msg);
                }

                let result = poll(&mut stream, &mut state, &mut message_queue);
                match result.msg {
                    Some(packet) => match thread_sender.send(ClientThread::Packet(packet)) {
                        Ok(_) => {}
                        Err(_) => {}
                    },
                    None => {}
                }

                if result.reconnect {
                    event_sender
                        .send(TcpEvent::ClientDisconnected { secret })
                        .unwrap();

                    return;
                }
            }
        }
    }
}

enum TcpEvent {
    ClientDisconnected {
        secret: Secret,
    },
    ClientConnected {
        secret: Secret,
        sender: Sender<PacketBuffer>,
        receiver: Receiver<ClientThread>,
    },
}

enum ClientThread {
    Packet(PacketBuffer),
}

/// The server for a dev connection.
pub struct Server {
    clients: HashMap<Secret, ClientId>,
    client_threads: HashMap<ClientId, (Sender<PacketBuffer>, Receiver<ClientThread>)>,
    next_client_id: usize,
    timer: Timer,
    queued_msgs: Vec<(ClientId, PacketBuffer)>,
    recvd_msgs: Vec<(ClientId, PacketBuffer)>,
    event_receiver: Receiver<TcpEvent>,
}
impl Server {
    /// Creates a new server.
    pub fn new(tick_rate: u32) -> Self {
        let (event_sender, event_receiver) = channel();

        let listener = TcpListener::bind(SERVER_ADDR).unwrap();
        let timer = Timer::new(tick_rate);
        std::thread::spawn(move || accept_clients(listener, event_sender, timer));

        Self {
            timer,
            client_threads: HashMap::new(),
            next_client_id: 0,
            clients: HashMap::new(),
            queued_msgs: vec![],
            recvd_msgs: vec![],
            event_receiver,
        }
    }

    /// Polls the server for packets.
    pub fn poll(&mut self) -> Option<(ClientId, PacketBuffer)> {
        for event in self.event_receiver.try_iter() {
            match event {
                TcpEvent::ClientDisconnected { secret } => {
                    // Remove any senders that may exist
                    if let Some(client_id) = self.clients.get(&secret) {
                        self.client_threads.remove(&client_id);
                    }
                }
                TcpEvent::ClientConnected {
                    secret,
                    sender,
                    receiver,
                } => {
                    // Insert a new client id if this one doesn't exist
                    let client_id = match self.clients.get(&secret) {
                        Some(id) => id.clone(),
                        None => {
                            let id = ClientId(self.next_client_id);
                            self.next_client_id = self.next_client_id.wrapping_add(1);
                            self.clients.insert(secret, id);
                            id
                        }
                    };

                    // Insert senders/receivers
                    self.client_threads.insert(client_id, (sender, receiver));
                }
            }
        }

        // Drain queued messages and send to clients
        while self.queued_msgs.is_empty() == false {
            let (client, packet_buffer) = self.queued_msgs.remove(0);
            if let Some((sender, _receiver)) = self.client_threads.get_mut(&client) {
                match sender.send(packet_buffer) {
                    _ => {}
                }
            }
        }

        // Receive messages from clients.
        for (client, (_sender, receiver)) in self.client_threads.iter_mut() {
            for msg in receiver.try_iter() {
                match msg {
                    ClientThread::Packet(packet) => {
                        self.recvd_msgs.push((*client, packet));
                    }
                }
            }
        }

        // Drain message if it exists
        if self.recvd_msgs.is_empty() {
            None
        } else {
            Some(self.recvd_msgs.remove(0))
        }
    }

    /// Sends a message to the client.
    pub fn send(&mut self, client: ClientId, packet: PacketBuffer) {
        self.queued_msgs.push((client, packet));
    }
}

#[derive(Clone, Copy, PartialEq, Debug, PartialOrd, Hash, Eq)]
pub struct ClientId(usize);
