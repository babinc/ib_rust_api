use std::net::{TcpStream, Shutdown};
use crate::api_parameter::ApiParameters;
use std::io::{Write, Read, Seek, SeekFrom, Cursor, ErrorKind};
use crate::decoder::Decoder;
use std::{thread, io};
use std::error::Error;
use crossbeam_channel::{unbounded, Sender, Receiver};
use crate::enums::incoming_message_enum::IncomingMessagesEnum;
use crate::enums::outgoing_messages::OutgoingMessages;
use crate::constants::{min_server_version, helper_constants};
use std::convert::TryInto;

pub struct ClientSocket {
    pub host: String,
    pub port: i32,
    pub client_id: i32,
    pub tcp_stream: Option<TcpStream>,
    pub server_version: i32,
    pub is_connected: bool,
    pub extra_auth: bool,
    pub use_v1000_plus: bool,
    pub server_time: String,
    event_sender: Sender<IncomingMessagesEnum>,
}

impl ClientSocket {
    pub fn new(host: String, port: i32, client_id: i32, event_sender: Sender<IncomingMessagesEnum>) -> Self {
        ClientSocket {
            host,
            port,
            client_id,
            tcp_stream: None,
            server_version: 0,
            is_connected: false,
            extra_auth: false,
            use_v1000_plus: true,
            server_time: "".to_string(),
            event_sender,
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let url = format!("{}:{}", self.host, self.port);
        match TcpStream::connect(url) {
            Ok(res) => {
                self.tcp_stream = Some(res);
                println!("Connected to the server");
            },
            Err(e) => {
                println!("Couldn't connect to server...");
                return Err(
                    Box::new(std::io::Error::new(ErrorKind::NotConnected, e))
                );
            }
        }

        self.send_connect_request()?;

        if self.is_connected {
            self.tell_ib_start_api()?;
        }

        let (tx, rx) = unbounded();

        self.listen_for_messages(tx)?;

        self.parse_messages(rx)?;

        Ok(())
    }

    fn send_connect_request(&mut self) -> std::io::Result<()> {
        if self.use_v1000_plus {
            let mut params = ApiParameters::new();
            params.add_string("API");

            let length_pos = params.prepare_buffer(self.use_v1000_plus);

            params.add_string_without_eol("v100..151");

            self.close_and_send(&mut params, length_pos).unwrap_or_else(|err| {
                std::io::Error::new(ErrorKind::Other, format!("Error client_socket, send_connect_request: {}", err.to_string()));
            });

            if let Some(tcp_stream) = self.tcp_stream.as_mut() {
                let msg = ClientSocket::read_single_message(tcp_stream)?;

                Decoder::process_connect_ack(Cursor::new(msg), &mut self.server_version, &mut self.server_time, &mut self.is_connected);
            }
        }
        else {
            unimplemented!()
        }

        Ok(())
    }

    fn read_single_message(tcp_stream: &mut TcpStream) -> Result<Vec<u8>, std::io::Error> {
        let msg_size = ClientSocket::read_tcp_i32(tcp_stream)?;

        if msg_size > helper_constants::MAX_MSG_SIZE {
            //TODO log
            eprintln!("Bad TCP Message Length");
            return Ok(vec![]);
        }

        let msg = ClientSocket::read_tcp_byte_array(tcp_stream, msg_size)?;
        Ok(msg)
    }

    pub fn close_and_send(&mut self, params: &mut ApiParameters, length_pos: u32) -> Result<(), Box<dyn Error>> { //make return Result and if error use
        if self.use_v1000_plus {
            params.cursor.seek(SeekFrom::Start(length_pos as u64))?;
            let value = (params.cursor.get_ref().len() - length_pos as usize - std::mem::size_of::<i32>()) as i32;
            params.cursor.write(&value.to_be_bytes())?;
        }

        match self.tcp_stream.as_mut() {
            Some(tcp) => {
                let bytes_written = tcp.write(params.cursor.get_ref())?;

                let buffer_len = params.cursor.get_ref().len();

                if bytes_written < buffer_len {
                    return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Sent {}/{} bytes", bytes_written, buffer_len))));
                }

                tcp.flush()?;
            },
            None => {
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, "Error retrieving tcp stream".to_string())));
            }
        }

        Ok(())
    }

    fn read_tcp_i32(tcp_stream: &mut TcpStream) -> Result<i32, std::io::Error> {
        let mut read_buf = [0u8; 4];
        tcp_stream.read(&mut read_buf)?;
        Ok(i32::from_be_bytes(read_buf))
    }

    fn read_tcp_byte_array(tcp_stream: &mut TcpStream, msg_size: i32) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = vec![0_u8; msg_size as usize];
        tcp_stream.read(&mut buf)?;
        Ok(buf)
    }

    fn tell_ib_start_api(&mut self) -> Result<(), Box<dyn Error>> {
//        if self.check_connection() == false {
//            return
//        }

        const VERSION: i32 = 2;

        let mut params = ApiParameters::new();
        let length_pos = params.prepare_buffer(self.use_v1000_plus);
        params.add_int(OutgoingMessages::StartApi as i32);
        params.add_int(VERSION);
        params.add_int(self.client_id);

        if self.server_version > min_server_version::OPTIONAL_CAPABILITIES {
            let optional_capabilities = "";
            params.add_string(optional_capabilities);
        }

        Ok(self.close_and_send(&mut params, length_pos)?)
    }

    fn recv_all_msg(tcp_stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut cont = true;
        let mut allbuf: Vec<u8> = Vec::new();
        const NUM_BYTES: usize = 4096;

        while cont {
            let mut buf: [u8; NUM_BYTES] = [0; NUM_BYTES];

            let bytes_read = tcp_stream.read(&mut buf).expect("Couldnt read from reader...");

            allbuf.extend_from_slice(&buf[0..bytes_read]);
            //logger.debug("len %d raw:%s|", len(buf), buf)

            if bytes_read < NUM_BYTES {
                cont = false;
            }
        }
        Ok(allbuf)
    }

    pub fn recv_packet(tcp_stream :&mut TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
        let buf = Self::recv_all_msg(tcp_stream)?;
        // receiving 0 bytes outside a timeout means the connection is either
        // closed or broken
        if buf.len() == 0 {
            tcp_stream.shutdown(Shutdown::Both)?;
            eprintln!("IbRustApi Error: Cannot connect - socket either closed or broken, disconnecting");
        }
        Ok(buf)
    }

    pub fn read_msg<'a>(buf: &[u8]) -> Result<(usize, String, Vec<u8>), Box<dyn Error>> {
        // first the size prefix and then the corresponding msg payload ""

        if buf.len() < 4 {
            dbg!("read_msg:  buffer too small!! {:?}", buf.len());
            return Ok((0, String::new(), buf.to_vec()));
        }

        let size = i32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize;
        //debug!("read_msg: Message size: {:?}", size);

        if buf.len() - 4 >= size {
            let text = String::from_utf8(buf[4..4 + size].to_vec()).unwrap();
            //debug!("read_msg: text in read message: {:?}", text);
            Ok((size, text, buf[4 + size..].to_vec()))
        } else {
            Ok((size, String::new(), buf.to_vec()))
        }
    }

    fn listen_for_messages(&mut self, messages: Sender<String>) -> Result<(), Box<dyn Error>> {
        let tcp = self.tcp_stream.as_ref();
        if let Some(tcp) = tcp {
            let mut tcp_clone = tcp.try_clone()?;
            thread::spawn(move || {
                loop {
                    // grab a packet of messages from the socket
                    let mut message_packet = Self::recv_packet(&mut tcp_clone).unwrap();
                    //debug!(" recvd size {}", message_packet.len());

                    // Read messages from the packet until there are no more.
                    // When this loop ends, break into the outer loop and grab another packet.
                    // Repeat until the connection is closed
                    let _msg = String::new();
                    while message_packet.len() > 0 {
                        // Read a message from the packet then add it to the message queue below.
                        let (_size, msg, remaining_messages) = Self::read_msg(message_packet.as_slice()).unwrap();

                        // clear the Vec that holds the bytes from the packet
                        // and reload with the bytes that haven't been read.
                        // The variable remaining_messages only holds the unread messagesleft in the packet
                        message_packet.clear();
                        message_packet.extend_from_slice(remaining_messages.as_slice());

                        if msg.as_str() != "" {
                            messages.send(msg).expect("READER CANNOT SEND MESSAGE");
                        } else {
                            //Break to the outer loop in run and get another packet of messages.

                            dbg!("more incoming packet(s) are needed ");
                            break;
                        }
                    }
                }
            });
        }

        Ok(())
    }

    fn parse_messages(&mut self, msg_queue: Receiver<String>) -> Result<(), Box<dyn Error>> {
        let event_sender = self.event_sender.clone();
        let server_version = self.server_version.clone();

        thread::spawn(move || {
            loop {
                let msg = msg_queue.recv();
                if let Ok(msg) = msg {
                    let fields = Self::read_fields((&msg).as_ref());
                    let mut decoder = Decoder::new(fields.as_slice());
                    decoder.process_incoming_message(server_version, &event_sender).unwrap_or_else(|err| {
                        eprintln!("ib_rust_api error process_incoming_message: {}", err.to_string());
                    });
                }
            }
        });

        Ok(())
    }

    fn read_fields(buf: &str) -> Vec<String> {
        //msg payload is made of fields terminated/separated by NULL chars
        let a = '\u{0}';
        let mut fields: Vec<&str> = buf.split(a).collect::<Vec<&str>>();
        //debug!("fields.len() in read_fields: {}", fields.len());
        //last one is empty
        fields.remove(fields.len() - 1);

        fields
            .iter()
            .map(|x| String::from(*x))
            .collect::<Vec<String>>()
    }
}
