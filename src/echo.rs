use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Echo {
    #[serde(rename = "type")]
    _type: String,
    echo: String,
}

pub fn handle_echo(msg: &String) -> String {
    let echo_msg: Message<Echo> = serde_json::from_str(msg).unwrap();

    let payload = echo_msg.get_payload();

    let echo_reply: Message<Echo> = echo_msg.switch_src_dest(payload);

    serde_json::to_string(&echo_reply).unwrap()
}
