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

impl<T: Clone> Message<T> {
    pub fn switch_src_dest(&self, payload: T) -> Message<T> {
        Message::<T> {
            src: self.dest.to_owned(),
            dest: self.src.to_owned(),
            body: Body::<T> {
                msg_id: self.body.msg_id,
                in_reply_to: self.body.msg_id,
                payload,
            },
        }
    }

    pub fn get_payload(&self) -> T {
        self.body.payload.clone()
    }
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

        let message = r#"{"src": "test1", "dest": "test2", "body": {"type": "read", "msg_id": 123, "key": 3}}"#;

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

    #[test]
    fn deserialization_works() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Payload {
            key: u32,
            #[serde(rename = "type")]
            _type: String,
        }

        let message = Message::<Payload> {
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

        let res: String = serde_json::to_string(&message).unwrap();

        let message_result: Message<Payload> = serde_json::from_str(&res).unwrap();

        assert_eq!(message_result, message);
    }
}
