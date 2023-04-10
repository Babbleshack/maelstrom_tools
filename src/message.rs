use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResponseBody<T> {
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,
    #[serde(flatten)]
    pub message: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body<MessageType> {
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,
    #[serde(flatten)]
    pub message: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitMessageType {
    Init(Init),
    InitOk,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message<MessageType> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<MessageType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn print_init_message() -> Result<(), serde_json::Error> {
        let m = Message::<InitMessageType> {
            src: "src_node".to_owned(),
            dst: "dest_node".to_owned(),
            body: Body::<InitMessageType> {
                msg_id: Some(1),
                in_reply_to: Some(0),
                message: InitMessageType::InitOk,
            },
        };
        let json = serde_json::to_string(&m)?;
        println!("{}", json);
        Ok(())
    }

    #[test]
    fn test_init_message() {
        let init_message = r#"
        {
            "src":"c0",
            "dest":"n1",
            "body":{
                "type":"init",
                "node_id":"n1",
                "node_ids":["n1"],
                "msg_id":1
            }
        }
        "#;
        let res: Result<Message<InitMessageType>, _> = serde_json::from_str(init_message);
        match res {
            Ok(init_message) => {
                assert_eq!(init_message.src, "c0");
                assert_eq!(init_message.dst, "n1");
                assert_eq!(init_message.body.msg_id, Some(1));
                match init_message.body.message {
                    InitMessageType::Init(init) => {
                        assert_eq!(init.node_id, "n1");
                        assert_eq!(init.node_ids.len(), 1);
                        assert_eq!(init.node_ids[0], "n1");
                    }
                    _ => panic!("invalid message type"),
                }
            }
            Err(e) => panic!("Error seriaising message {}", e),
        }
    }

    //INFO: Received {"id":0,"src":"c0","dest":"n1","body":{"type":"init","node_id":"n1","node_ids":["n1"],"msg_id":1}}
    #[test]
    fn test_err_missing_msg_id() {
        let json = r#"
        {
            "id":0,
            "src":"c0",
            "dest":"n1",
            "body":{
                "type":"init",
                "node_id":"n1",
                "node_ids":["n1"],
                "msg_id":1
                }
        }
        "#;
        let res: Result<Message<InitMessageType>, _> = serde_json::from_str(json);
        assert!(res.is_ok());
    }

    //#[test]
    //fn test_echo_message() {
    //    let echo_message = r#"
    //    {
    //    "src":"src_node",
    //    "dest":"dst_node",
    //    "body":{
    //        "type":"echo",
    //        "msg_id":1,
    //        "echo":"some_string"
    //        }
    //    }
    //    "#;
    //    let m: Message<RequestBody> = serde_json::from_str(&echo_message).unwrap();
    //    assert_eq!(m.src, "src_node");
    //    assert_eq!(m.dst, "dst_node");
    //    let echo = match m.body.message {
    //        MessageRequestType::Echo { echo } => echo,
    //        _ => panic!("error unexpected patter"),
    //    };
    //    assert_eq!(echo, "some_string");
    //}
}
