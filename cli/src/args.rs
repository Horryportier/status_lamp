use std::usize;

use anyhow::Ok;
use clap::Parser;

use crate::{
    client::client,
    opt::{Color, Fill, Msg, MsgKind, Response},
    server::server_init,
};

macro_rules! if_arg {
    ($bool:expr, $msg:literal, $fun:expr) => {
        if $bool {
            println!("{}", $msg.cyan());
            $fun;
            println!("{}", "succes".green());
        }
    };
    ($bool:expr, $fun:expr) => {
        if $bool {
            $fun;
        }
    };
}

#[derive(Debug, Parser, Default)]
pub struct Args {
    #[arg(short, long)]
    server: bool,
    #[arg(short, long)]
    quiet: bool,
    #[arg(long)]
    analog: bool,
    #[arg(long)]
    pin: Option<usize>,
    #[arg(long)]
    pin_value: Option<usize>,
    #[arg(long)]
    color: Option<String>,
    #[arg(long)]
    fill: Option<String>,
    #[arg(long)]
    set_ring: bool,
    #[arg(long)]
    set_middle: bool,
    #[arg(long)]
    get_pin: bool,
    #[arg(long)]
    set_pin: bool,
}

impl Args {
    pub fn execute(&self) -> anyhow::Result<()> {
        if_arg!(self.server, {
            server_init()?;
            return Ok(());
        });

        let color: Color = color_from_string(self.color.clone().unwrap_or("".into()))?;
        let fill: Fill = fill_from_string(self.fill.clone().unwrap_or("".into()))?;

        let send_middle_msg = || -> anyhow::Result<()> {
            let msg = Msg {
                op: crate::opt::OptCodes::SetMiddle,
                data: MsgKind::SetMiddle(crate::opt::SetMiddle {
                    color: color.clone(),
                }),
            };
            let a = client(msg)?;
            if !self.quiet {
                println!("{:?}", serde_json::from_slice::<Response>(a.as_slice()));
            }
            Ok(())
        };
        if_arg!(self.set_middle, send_middle_msg()?);

        let send_ring_msg = || -> anyhow::Result<()> {
            let msg = Msg {
                op: crate::opt::OptCodes::SetRing,
                data: MsgKind::SetRing(crate::opt::SetRing { color, fill }),
            };
            let a = client(msg)?;

            if !self.quiet {
                println!("{:?}", serde_json::from_slice::<Response>(a.as_slice()));
            }
            Ok(())
        };
        if_arg!(self.set_ring, send_ring_msg()?);
        let send_get_pin_msg = || -> anyhow::Result<()> {
            let pin = self.pin.expect("specify [--pin] option ");
            let msg = Msg {
                op: crate::opt::OptCodes::GetPin,
                data: MsgKind::GetPin(crate::opt::GetPin {
                    pin,
                    analog: self.analog,
                }),
            };
            let a = client(msg)?;

            if !self.quiet {
                println!("{:?}", serde_json::from_slice::<Response>(a.as_slice()));
            }
            Ok(())
        };
        if_arg!(self.get_pin, send_get_pin_msg()?);
        let send_set_pin_msg = || -> anyhow::Result<()> {
            let pin = self.pin.expect("specify [--pin] option ");
            let pin_value = self.pin_value.expect("specify [--pin-value] option ");
            let msg = Msg {
                op: crate::opt::OptCodes::SetPin,
                data: MsgKind::SetPin(crate::opt::SetPin {
                    pin,
                    value: pin_value,
                    analog: self.analog,
                }),
            };
            let a = client(msg)?;
            if !self.quiet {
                println!("{:?}", serde_json::from_slice::<Response>(a.as_slice()));
            }
            Ok(())
        };
        if_arg!(self.set_pin, send_set_pin_msg()?);

        Ok(())
    }
}
fn hex_to_rgb(hex: String) -> Color {
    let hex = hex.strip_prefix("#").unwrap_or("000000").trim();
    let r = u8::from_str_radix(&hex.get(0..2).unwrap_or("0"), 16).unwrap();
    let g = u8::from_str_radix(&hex.get(2..4).unwrap_or("0"), 16).unwrap();
    let b = u8::from_str_radix(&hex.get(4..6).unwrap_or("0"), 16).unwrap();
    Color { r, g, b }
}

fn color_from_string(s: String) -> anyhow::Result<Color> {
    if s.starts_with("#") {
        return Ok(hex_to_rgb(s));
    }
    let v = s
        .split(" ")
        .map(|f| f.parse().unwrap_or(0))
        .collect::<Vec<u8>>();
    Ok(Color {
        r: *v.get(0).unwrap_or(&0),
        g: *v.get(1).unwrap_or(&0),
        b: *v.get(2).unwrap_or(&0),
    })
}
fn fill_from_string(s: String) -> anyhow::Result<Fill> {
    let v = s
        .split(" ")
        .map(|f| f.parse().unwrap_or(0))
        .collect::<Vec<usize>>();
    Ok(Fill {
        start: *v.get(0).unwrap_or(&0),
        stop: *v.get(1).unwrap_or(&0),
    })
}
