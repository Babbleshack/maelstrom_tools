use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum MessageRequestType {
    #[serde(rename = "init")]
    Init {
        msg_id: u64,
        node_id: String,
        node_ids: Vec<String>,
    },
    #[serde(rename = "echo")]
    Echo { echo: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RequestBody {
    #[serde(rename = "msg_id")]
    pub msg_id: Option<u64>,
    #[serde(flatten)]
    pub message: MessageRequestType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum MessageResponseType {
    #[serde(rename = "init_ok")]
    Init {},
    #[serde(rename = "echo_ok")]
    Echo { echo: String },
}

// TODO: we should .map_err() from messsage_loop to error enum
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ErrorResponse {
    pub code: u8,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResponseBody<T> {
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,
    #[serde(flatten)]
    pub message: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message<T> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: T,
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
            dst: "dest_node".to_owned(),
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
    fn test_init_message() {
        let init_message = r#"
        {
            "id":0,
            "src":"c0",
            "dst":"n1",
            "body":{
                "type":"init",
                "node_id":"n1",
                "node_ids":["n1"],
                "msg_id":1
            }
        }
        "#;

        let res: Result<Message<RequestBody>, _> = serde_json::from_str(init_message);
        if let Err(e) = res {
            println!("{}", e);
        }
    }

    #[test]
    fn test_echo_message() {
        let echo_message = r#"
        {
        "src":"src_node",
        "dest":"dst_node",
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
