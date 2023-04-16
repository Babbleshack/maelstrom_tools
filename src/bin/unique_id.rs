use std::io::{Read, Write};

use anyhow::{anyhow, Context};
use maelstrom::message::Init;
use maelstrom::net::IOHandler;
use maelstrom::node::{main_loop, Node};
use maelstrom::snowflake::Snowflake;
use maelstrom::{Body, LogLevel, Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum GenerateMessageType {
    Generate,
    GenerateOk { id: u64 },
}
struct GenerateNode {
    #[allow(dead_code)]
    id: String,
    generator: Snowflake,
}

impl Node<GenerateMessageType> for GenerateNode {
    fn from_init(init: Init) -> anyhow::Result<Self> {
        let seed: u64 = init
            .node_id
            .strip_prefix("n")
            .ok_or(anyhow!("error could not strip node id"))?
            .parse()
            .map_err(|e| anyhow!("error could not create seed: {}", e))?;
        Ok(Self {
            id: init.node_id.clone(),
            generator: Snowflake::new(seed).into_iter(),
        })
    }

    fn process_message<R: Read, W: Write, L: Write>(
        &mut self,
        message: maelstrom::Message<GenerateMessageType>,
        io: &mut IOHandler<R, W, L>,
    ) -> anyhow::Result<()> {
        let id = self
            .generator
            .next()
            .ok_or_else(|| anyhow!("error generating id"))?;
        let response = match message.body.message {
            GenerateMessageType::Generate => Message {
                src: self.id.clone(),
                dst: message.src,
                body: Body {
                    msg_id: Some(0),
                    in_reply_to: message.body.msg_id,
                    message: GenerateMessageType::GenerateOk { id },
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

fn main() -> Result<(), ()> {
    main_loop::<GenerateNode, GenerateMessageType>().map_err(|e| {
        eprint!("error running main loop: {}", e);
        std::process::exit(1);
    })
}
