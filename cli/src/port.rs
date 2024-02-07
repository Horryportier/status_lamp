use std::{thread::sleep, time::Duration};

use pretty_log::log::warn;
use serialport::SerialPort;

use crate::opt::{HelloMsg, Msg, Response, MsgKind};

pub fn def_port() -> std::io::Result<Box<dyn SerialPort>> {
    Ok(serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(10))
        .open()?)
}

pub fn setup_port(port: &mut Box<dyn SerialPort>) -> std::io::Result<()> {
    sleep(Duration::from_secs(2));
    port.write_data_terminal_ready(true)?;

    let hellomsg =  Msg  { op: crate::opt::OptCodes::HELLO, data: MsgKind::Hello( HelloMsg { msg: "hi".into() })};

    let res = Response {
        op: crate::opt::OptCodes::HELLO,
        msg_code: 200,
        msg: "hi".into(),
    };
    while send_and_recive(port, hellomsg.clone())? != res {
        sleep(Duration::from_secs(1))
    }
    Ok(())
}

pub fn send_and_recive (
    port: &mut Box<dyn SerialPort>,
    msg: Msg,
) -> std::io::Result<Response> {
    let json = serde_json::to_string(&msg)?;
    port.write(json.as_bytes())?;

    

    let mut buf = [0; 1000];
    for _i in 0..=10 {
        sleep(Duration::from_millis(500));
        let _ = match port.read(&mut buf) {
            Ok(..) => break,
            Err(e) =>  {
                if e.kind() != std::io::ErrorKind::TimedOut  { 
                    return Err(e);
                } 
                warn("Read operation timed out retying");
            }
        };
    }

    let mut s = String::from_utf8(buf.to_vec()).unwrap();
    s = s.trim_matches(char::from(0)).into();

    Ok(serde_json::from_str::<Response>(&s.trim_end())?)
}
