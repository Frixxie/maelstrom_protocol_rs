use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Init {
    #[serde(rename = "type")]
    _type: String,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InitOk {
    #[serde(rename = "type")]
    _type: String,
}

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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Echo {
    #[serde(rename = "type")]
    _type: String,
    echo: String,
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    let initmsg: Message<Init> = serde_json::from_str(&buffer).unwrap();

    let mut stdout = io::stdout().lock();

    let initok = Message::<InitOk> {
        src: initmsg.dest,
        dest: initmsg.src,
        body: Body::<InitOk> {
            payload: InitOk {
                _type: "init_ok".to_string(),
            },
            in_reply_to: initmsg.body.msg_id,
            msg_id: None,
        },
    };

    let mut initok_reply = serde_json::to_string(&initok)?;
    initok_reply.push('\n');

    stdout.write_all(initok_reply.as_bytes()).unwrap();

    for line in io::stdin().lines() {
        let echomsg: Message<Echo> = serde_json::from_str(&line.unwrap()).unwrap();

        let reply = Message::<Echo> {
            src: echomsg.dest,
            dest: echomsg.src,
            body: Body::<Echo> {
                in_reply_to: Some(echomsg.body.msg_id.unwrap()),
                msg_id: echomsg.body.msg_id,
                payload: Echo {
                    _type: "echo_ok".to_string(),
                    echo: echomsg.body.payload.echo,
                },
            },
        };

        let mut echo_reply = serde_json::to_string(&reply)?;
        echo_reply.push('\n');

        stdout.write_all(echo_reply.as_bytes()).unwrap();
    }
    Ok(())
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
