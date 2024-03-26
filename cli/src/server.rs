use serialport::SerialPort;

use crate::{
    common::SOCKET_PATH,
    port::{def_port, send_and_recive, setup_port},
};

use std::{
    fs,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    thread::sleep,
    time::Duration,
};
const MAX_RETRIES: usize = 32;
pub trait MsgHandler {
    fn handle_stream(&mut self, stream: UnixStream) -> anyhow::Result<()>;
}

#[derive(Clone, Copy)]
pub struct EchoHandler;
impl MsgHandler for EchoHandler {
    fn handle_stream(&mut self, mut stream: UnixStream) -> anyhow::Result<()> {
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        println!("{:?}", buf);
        stream.write(format!("echo {buf}").as_str().as_bytes())?;
        Ok(())
    }
}

pub struct ArduinoSerialHandrel {
    pub port: Box<dyn SerialPort>,
}

impl ArduinoSerialHandrel {
    fn init(port: Option<Box<dyn SerialPort>>) -> anyhow::Result<ArduinoSerialHandrel> {
        let mut p = match port {
            Some(p) => p,
            None => def_port()?,
        };
        setup_port(&mut p)?;
        Ok(ArduinoSerialHandrel { port: p })
    }
}

impl MsgHandler for ArduinoSerialHandrel {
    fn handle_stream(&mut self, mut stream: UnixStream) -> anyhow::Result<()> {
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        let msg = serde_json::from_str(&buf)?;
        let response = send_and_recive(&mut self.port, msg)?;
        stream.write(&serde_json::to_vec(&response)?)?;
        Ok(())
    }
}

pub fn server_init() -> anyhow::Result<()> {
    let ash = |_| {
        for i in 0..MAX_RETRIES {
            let a = ArduinoSerialHandrel::init(None);
            match a {
                Ok(ah) => return Some(ah),
                Err(err) => {
                    if "No such file or directory" == err.to_string() {
                        sleep(Duration::from_secs(2));
                        eprintln!("[{i}] could not connect to arduino retrying");
                        continue;
                    } else {
                        panic!("Could not connect to arduino {:?}", err);
                    }
                }
            }
        }
        None
    };
    server(ash(()).unwrap())?;
    Ok(())
}

pub fn server<T: MsgHandler>(mut handle: T) -> anyhow::Result<()> {
    let socket = Path::new(SOCKET_PATH);

    if socket.exists() {
        fs::remove_file(&socket)?;
    }

    let listener = UnixListener::bind(&socket)?;

    loop {
        let (stream, _) = listener.accept()?;
        handle.handle_stream(stream)?
    }
}
