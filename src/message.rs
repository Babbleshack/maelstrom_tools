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
    pub dst: String,
    pub body: T,
}
