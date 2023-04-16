use std::io::{Read, Write};

use anyhow::{anyhow, Context};
use maelstrom::message::Init;
use maelstrom::net::IOHandler;
use maelstrom::node::{main_loop, Node};
use maelstrom::{Body, LogLevel, Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum EchoMessageType {
    Echo { echo: String },
    EchoOk { echo: String },
}
struct EchoNode {
    #[allow(dead_code)]
    id: String,
}

impl Node<EchoMessageType> for EchoNode {
    fn from_init(init: Init) -> anyhow::Result<Self> {
        Ok(Self { id: init.node_id })
    }

    fn process_message<R: Read, W: Write, L: Write>(
        &mut self,
        message: maelstrom::Message<EchoMessageType>,
        io: &mut IOHandler<R, W, L>,
    ) -> anyhow::Result<()> {
        let response = match message.body.message {
            EchoMessageType::Echo { echo } => Message {
                src: self.id.clone(),
                dst: message.src,
                body: Body {
                    msg_id: Some(0),
                    in_reply_to: message.body.msg_id,
                    message: EchoMessageType::EchoOk { echo },
                },
            },
            _ => panic!("got invalid message type"),
        };
        let response_json = serde_json::to_string(&response)?;
        io.log(
            format!("Sending {}", response_json.as_str()).as_str(),
            LogLevel::INFO,
        )?;
        io.write_line(response_json)?;
        Ok(())
    }
}

fn main() {
    main_loop::<EchoNode, EchoMessageType>().expect("error running main loop");
}
