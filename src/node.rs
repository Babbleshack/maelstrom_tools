use crate::message::RequestBody;

use super::clock::LamportClock;
use super::message::*;
use super::net::{IOHandler, LogLevel};
use serde::Serialize;

struct Node {
    id: String,
    node_ids: Vec<String>,
    clock: LamportClock,
    io: IOHandler,
}

impl Node {
    fn new() -> Self {
        Node {
            id: "".to_string(),
            node_ids: Vec::new(),
            clock: LamportClock::new(),
            io: IOHandler::new(),
        }
    }

    fn read_message(&mut self) -> std::io::Result<Message<RequestBody>> {
        let line = self.io.read_line()?;
        self.io.log(format!("Received {}", line), LogLevel::INFO)?;
        let message: Message<RequestBody> = serde_json::from_str(&line)?;
        let m_id = message.body.msg_id.unwrap_or(0u64);
        self.clock.fetch_set(m_id);
        Ok(message)
    }

    fn write_message<T: Serialize + Clone>(
        &mut self,
        dst: String,
        response_body: &mut ResponseBody<T>,
    ) {
        let m_id = self.clock.increment();
        response_body.msg_id = Some(m_id);
        let resp = Message::<ResponseBody<T>> {
            src: self.id.clone(),
            dst,
            body: response_body.clone(),
        };
        let buf = serde_json::to_string(&resp).unwrap();
        self.io.write(buf.as_bytes()).unwrap();
    }

    fn process_message(&mut self, _message: &Message<RequestBody>) {}

    fn innitialise(&mut self, node_id: String, node_ids: Vec<String>) {
        self.id = node_id;
        self.node_ids = node_ids;
        self.io
            .log(
                format!(
                    "Initialised node: node_id: {}, other_nodes: {:?}",
                    self.id, self.node_ids,
                ),
                LogLevel::INFO,
            )
            .unwrap();
    }

    fn innitialise_from_message(
        &mut self,
        message: Message<RequestBody>,
    ) -> Result<Message<ResponseBody<MessageResponseType>>, String> {
        match message.body.message {
            MessageRequestType::Init {
                msg_id,
                node_id,
                node_ids,
            } => {
                self.innitialise(node_id, node_ids);
                Ok(Message::<ResponseBody<MessageResponseType>> {
                    src: self.id.clone(),
                    dst: message.src,
                    body: ResponseBody {
                        msg_id: Some(0),
                        in_reply_to: Some(msg_id),
                        message: MessageResponseType::Init {},
                    },
                })
            }
            _ => Err("error found invalid enum".to_string()),
        }
    }

    fn wait_for_init(&mut self) -> Message<ResponseBody<MessageResponseType>> {
        loop {
            let message = self.read_message().unwrap();
            match self.innitialise_from_message(message) {
                Ok(response) => return response,
                Err(_) => continue,
            }
        }
    }

    fn message_loop(&mut self) {
        let init_response = self.wait_for_init();
        let response_json = serde_json::to_string(&init_response).unwrap();
        self.io.write(response_json.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_node() {
        let mut node = Node::new();
        let init_request = Message {
            src: "src_node".to_owned(),
            dst: "dst_node".to_owned(),
            body: RequestBody {
                msg_id: Some(0),
                message: MessageRequestType::Init {
                    msg_id: 1,
                    node_id: "test_node_01".to_string(),
                    node_ids: std::vec!["test_node_01".to_string(), "test_node_02".to_string()],
                },
            },
        };
        let res = node.innitialise_from_message(init_request);
        assert!(res.is_ok());
        let res = res.unwrap();

        assert_eq!(res.src, "test_node_01");
        assert_eq!(res.dst, "src_node");
        assert_eq!(node.id, "test_node_01".to_string())
    }
}
