use crate::Body;

use super::message::{Init, InitMessageType, Message};
use super::IOHandler;
use anyhow::Context;
use serde::de::DeserializeOwned;

pub trait Node<MessageType> {
    fn from_init(_: Init) -> anyhow::Result<Self>
    where
        Self: Sized;
    fn process_message(&mut self, _: Message<MessageType>) -> anyhow::Result<()>;
}

pub fn main_loop<N, MT>() -> anyhow::Result<()>
where
    MT: DeserializeOwned + Send + 'static,
    N: Node<MT>,
{
    let mut io = IOHandler::new();
    let line = io.read_line().context("failed to read input")?;

    let message: Message<InitMessageType> =
        serde_json::from_str(&line).context("failed to deserialise init messsage")?;

    let init = match message.body.message {
        InitMessageType::Init(init) => init,
        _ => panic!("got invalid message type, expecting init"),
    };

    let mut node: N = Node::from_init(init).context("node init failed")?;

    let response = Message {
        src: message.dst,
        dst: message.src,
        body: Body {
            msg_id: Some(0),
            in_reply_to: Some(0),
            message: InitMessageType::InitOk,
        },
    };

    let response_json = serde_json::to_string(&response).context("failed to serialise response")?;
    io.write(response_json.as_bytes())
        .context("failed sending response")?;

    drop(io); // free lock
    let (tx, rx) = std::sync::mpsc::channel();

    let message_loop_handle = std::thread::spawn(move || {
        let mut io = IOHandler::new();
        let line = io
            .read_line()
            .context("error reading maelstrom input from STDIN")
            .unwrap();
        let message: Message<MT> = serde_json::from_str(&line)
            .context("deserialising message from stdin")
            .unwrap();
        tx.send(message).unwrap();
    });

    for input in rx {
        node.process_message(input)
            .context("failed to process message")?;
    }

    message_loop_handle
        .join()
        .expect("error thrown from stdin spooler");

    Ok(())
}
