use std::u8;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OptCodes {
    HELLO = 1,
    SetRing = 2,
    SetMiddle = 3,
    SetPin = 4,
    GetPin = 5,
    Quit = 6,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default, Clone)]
pub struct Fill {
    pub start: usize,
    pub stop: usize,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Response {
    pub op: OptCodes,
    pub msg_code: usize,
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum MsgKind {
    Hello(HelloMsg),
    SetPin(SetPin),
    GetPin(GetPin),
    SetRing(SetRing),
    SetMiddle(SetMiddle),
    Quit,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Msg {
    pub op: OptCodes,
    pub data: MsgKind,
}

impl ToString for Msg {
    fn to_string(&self) -> String {
        match serde_json::to_string(self) {
            Ok(s) => s,
            Err(e) => e.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct HelloMsg {
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct GetPin {
    pub pin: usize,
    pub analog: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct SetPin {
    pub pin: usize,
    pub value: usize,
    pub analog: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct SetRing {
    pub color: Color,
    pub fill: Fill,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct SetMiddle {
    pub color: Color,
}

#[cfg(test)]
mod test {
    use log::info;
    use serialport::SerialPort;
    use std::{thread::sleep, time::Duration};

    fn def_port() -> std::io::Result<Box<dyn SerialPort>> {
        Ok(serialport::new("/dev/ttyUSB0", 9600)
            .timeout(Duration::from_millis(2000))
            .open()?)
    }

    fn send_and_recive(mut port: Box<dyn SerialPort>, msg: Msg) -> std::io::Result<Response> {
        sleep(Duration::from_secs(2));
        port.write_data_terminal_ready(true)?;

        let json = serde_json::to_string(&msg)?;
        port.write(json.as_bytes())?;

        let mut buf = [0; 1000];
        port.read(&mut buf).unwrap();

        let mut s = String::from_utf8(buf.to_vec()).unwrap();
        s = s.trim_matches(char::from(0)).into();

        Ok(serde_json::from_str::<Response>(&s.trim_end())?)
    }

    use super::*;
    #[test]
    fn hello_test() -> std::io::Result<()> {
        let correct = Response {
            op: OptCodes::HELLO,
            msg_code: 200,
            msg: "Hello World!".into(),
        };

        let msg = Msg {
            op: OptCodes::HELLO,
            data: MsgKind::Hello(HelloMsg {
                msg: "Hello World!".into(),
            }),
        };
        let res = send_and_recive(def_port()?, msg)?;
        info!("resonse {:#?}", res);
        assert_eq!(res, correct);

        Ok(())
    }

    #[ignore = "non concurent test"]
    #[test]
    fn test_get_pin() -> std::io::Result<()> {
        let correct = Response {
            op: OptCodes::GetPin,
            msg_code: 200,
            msg: "0".into(),
        };
        let msg = Msg {
            op: OptCodes::GetPin,
            data: MsgKind::GetPin(GetPin {
                pin: 2,
                analog: false,
            }),
        };
        let res = send_and_recive(def_port()?, msg)?;
        assert_eq!(res, correct);
        Ok(())
    }
    #[ignore = "non concurent test"]
    #[test]
    fn test_set_pin() -> std::io::Result<()> {
        let correct = Response {
            op: OptCodes::SetPin,
            msg_code: 200,
            msg: "none".into(),
        };
        let msg = Msg {
            op: OptCodes::SetPin,
            data: MsgKind::SetPin(SetPin {
                pin: 2,
                value: 1,
                analog: false,
            }),
        };
        let res = send_and_recive(def_port()?, msg)?;
        assert_eq!(res, correct);
        Ok(())
    }
    #[ignore = "non concurent test"]
    #[test]
    fn test_set_middle() -> std::io::Result<()> {
        let correct = Response {
            op: OptCodes::SetMiddle,
            msg_code: 200,
            msg: "none".into(),
        };
        let msg = Msg {
            op: OptCodes::SetMiddle,
            data: MsgKind::SetMiddle(SetMiddle {
                color: Color {
                    r: 50,
                    g: 10,
                    b: 23,
                },
            }),
        };
        let res = send_and_recive(def_port()?, msg)?;
        assert_eq!(res, correct);
        Ok(())
    }
    #[ignore = "non concurent test"]
    #[test]
    fn test_set_ring() -> std::io::Result<()> {
        let correct = Response {
            op: OptCodes::SetRing,
            msg_code: 200,
            msg: "none".into(),
        };
        let msg = Msg {
            op: OptCodes::SetRing,
            data: MsgKind::SetRing(SetRing {
                color: Color {
                    r: 50,
                    g: 10,
                    b: 23,
                },
                fill: Fill { start: 0, stop: 16 },
            }),
        };
        println!("{:#?}", serde_json::to_string(&msg));
        let res: Response = send_and_recive(def_port()?, msg)?;
        println!("{res:#?}");
        assert_ne!(res, correct);
        Ok(())
    }
}
