use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Body<T> {
    msg_id: Option<u32>,
    in_reply_to: Option<u32>,
    #[serde(flatten)]
    payload: T,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message<T> {
    src: String,
    dest: String,
    body: Body<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_works() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Payload {
            key: u32,
            #[serde(rename = "type")]
            _type: String,
        }

        let message =
            r#"{"src": "test1", "dest": "test2", "body": {"type": "read", "msg_id": 123, "key": 3}}"#;

        let message_expect = Message::<Payload> {
            src: "test1".to_string(),
            dest: "test2".to_string(),
            body: Body::<Payload> {
                msg_id: Some(123),
                in_reply_to: None,
                payload: Payload {
                    key: 3,
                    _type: "read".to_string(),
                },
            },
        };

        let res: Message<Payload> = serde_json::from_str(message).unwrap();

        assert_eq!(res, message_expect);
    }
}
