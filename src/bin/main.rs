use maelstrom::message::Init;
use maelstrom::node::{main_loop, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum TestMessageType {
    Message,
    Response,
}
struct TestNode {
    id: String,
}

impl Node<TestMessageType> for TestNode {
    fn from_init(init: Init) -> anyhow::Result<Self> {
        Ok(Self { id: init.node_id })
    }

    fn process_message(&mut self, _: maelstrom::Message<TestMessageType>) -> anyhow::Result<()> {
        eprintln!("got id: {}", self.id);
        panic!("");
    }
}

fn main() {
    main_loop::<TestNode, TestMessageType>().expect("error running main loop");
}
