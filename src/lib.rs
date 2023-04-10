#![allow(dead_code)]
mod clock;
mod message;
mod net;
mod node;

pub use clock::LamportClock;
pub use message::*;
pub use net::*;
pub use node::*;

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
            "echo":"some_string"
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
