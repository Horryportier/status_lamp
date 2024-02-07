use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::Path,
};

use crate::common::SOCKET_PATH;

pub fn client<T>(msg: T) -> anyhow::Result<Vec<u8>>
where
    T: ToString,
{
    let socket = Path::new(SOCKET_PATH);
    let mut stream = UnixStream::connect(&socket)?;

    write_request_and_shutdown(&mut stream, msg.to_string().as_bytes())?;
    Ok(read_from_stream(&mut stream)?)
}

fn write_request_and_shutdown(stream: &mut UnixStream, msg: &[u8]) -> anyhow::Result<()> {
    stream.write(msg)?;
    stream.shutdown(std::net::Shutdown::Write)?;
    Ok(())
}

fn read_from_stream(stream: &mut UnixStream) -> anyhow::Result<Vec<u8>> {
    let mut res = Vec::new();
    stream.read_to_end(&mut res)?;

    Ok(res)
}

#[cfg(test)]
mod test {

    use crate::opt::Msg;

    use super::*;

    #[test]
    fn test_client() -> anyhow::Result<()> {
        let msg = Msg {
            op: crate::opt::OptCodes::SetRing,
            data: crate::opt::MsgKind::SetRing(crate::opt::SetRing {
                color: crate::opt::Color { r: 20, g: 20, b: 9 },
                fill: crate::opt::Fill { start: 0, stop: 6 },
            }),
        };
        println!("{:?}", String::from_utf8(client(msg)?));
        assert_ne!(false, true);
        Ok(())
    }
}
