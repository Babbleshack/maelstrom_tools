mod clock;
mod message;

use std::io::{self, Read, Write};

use clock::LamportClock;
use message::*;

use serde::{Deserialize, Serialize};

struct Node {
    id: String,
    node_ids: Vec<String>,
    clock: LamportClock,
}

impl Node {
    fn new() -> Self {
        Node {
            id: "".to_string(),
            node_ids: Vec::new(),
            clock: LamportClock::new(),
        }
    }

    fn log_message(&mut self, message: String) {
        io::stderr().write(message.as_bytes()).unwrap();
    }

    fn read_message(&mut self) -> Message<RequestBody> {
        let mut s = String::new();
        self.read_to_string(&mut s).unwrap();
        self.log_message(format!("Received {}", s));
        let message: Message<RequestBody> = serde_json::from_str(s.as_ref()).unwrap();
        let m_id = message.body.msg_id.unwrap();
        self.clock.fetch_set(m_id);
        message
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
            dst: dst.clone(),
            body: response_body.clone(),
        };
        let buf = serde_json::to_string(&resp).unwrap();
        self.write(buf.as_bytes());
    }

    fn process_message(&mut self, message: &Message<RequestBody>) {}

    fn innitialise(&mut self, node_id: String, node_ids: Vec<String>) {
        self.id = node_id;
        self.node_ids = node_ids;
        self.log_message(format!(
            "Initialised node: node_id: {}, other_nodes: {:?}",
            self.id, self.node_ids,
        ));
    }

    fn innitialise_from_message(
        &mut self,
        message: Message<RequestBody>,
    ) -> Result<Message<ResponseBody<MessageResponseType>>, String> {
        return match message.body.message {
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
        };
    }

    fn wait_for_init(&mut self) -> Message<ResponseBody<MessageResponseType>> {
        loop {
            let message = self.read_message();
            match self.innitialise_from_message(message) {
                Ok(response) => return response,
                Err(_) => continue,
            }
        }
    }

    fn message_loop(&mut self) {
        let init_response = self.wait_for_init();
    }
}

impl Read for Node {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        buf.to_owned().write(s.as_bytes())
    }
}

impl Write for Node {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::stdout().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // only run this test if we want to see output
    // use cargo test -- --ignored to run this
    #[test]
    #[ignore]
    fn print_echo_request() -> Result<(), serde_json::Error> {
        let m = Message {
            src: "src_node".to_owned(),
            dst: "dst_node".to_owned(),
            body: RequestBody {
                msg_id: Some(0),
                message: MessageRequestType::Echo {
                    echo: "some_echo".to_string(),
                },
            },
        };
        let s = serde_json::to_string(&m)?;
        println!("-------------");
        println!("{:?}", m);
        println!("{}", s);
        println!("-------------");
        Ok(())
    }

    #[test]
    fn test_echo_message() {
        let echo_message = r#"
        {
        "src":"src_node",
        "dst":"dst_node",
        "body":{
            "type":"echo",
            "msg_id":1,
            "echo":"some string"
            }
        }
        "#;
        let m: Message<RequestBody> = serde_json::from_str(&echo_message).unwrap();
        assert_eq!(m.src, "src_node");
        assert_eq!(m.dst, "dst_node");
        let echo = match m.body.message {
            MessageRequestType::Echo { echo } => echo,
            _ => panic!("error unexpected patter"),
        };
        assert_eq!(echo, "some_string");
    }
}
