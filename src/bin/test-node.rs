use std::io::{Read, Write};

use maelstrom::message::Init;
use maelstrom::net::IOHandler;
use maelstrom::node::{main_loop, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum TestMessageType {
    Message,
    Response,
}
struct TestNode {
    #[allow(dead_code)]
    id: String,
}

impl Node<TestMessageType> for TestNode {
    fn from_init(init: Init) -> anyhow::Result<Self> {
        Ok(Self { id: init.node_id })
    }

    fn process_message<R: Read, W: Write, L: Write>(
        &mut self,
        message: maelstrom::Message<TestMessageType>,
        io: &mut IOHandler<R, W, L>,
    ) -> anyhow::Result<()> {
        io.write(format!("got message: {:?}", message).as_bytes())?;
        panic!("");
    }
}

fn main() {
    main_loop::<TestNode, TestMessageType>().expect("error running main loop");
}
