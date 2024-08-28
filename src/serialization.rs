use serde::{Deserialize, Serialize};
use rust_intro::rust_intro::DataMessage;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MessageList {
    pub messages: Vec<Message>,
}

impl Message {
    pub fn new(id: String, content: String) -> Self {
        Self { id, content }
    }
}

impl Into<DataMessage> for Message {
    fn into(self) -> DataMessage {
        DataMessage { id: self.id, content: self.content }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_message() {
        let m = serde_json::from_str::<Message>("{\"id\": \"1\", \"content\": \"Some Content\"}")
            .unwrap();
        assert_eq!(m.id, "1");
        assert_eq!(m.content, "Some Content");
    }

    #[test]
    fn test_deserialize_message_list() {
        let m = serde_json::from_str::<MessageList>(
            r#"
        {
                "messages": [
                {
                    "id": "1",
                    "content": "Content of Message1"
                },
                {
                    "id": "2",
                    "content": "Content of Message2"
                },
                {
                    "id": "3",
                    "content": "Content of Message3"
                }
            ]
        }"#,
        ).unwrap();

        assert_eq!(
            m.messages,
            vec![
                Message::new("1".to_string(), "Content of Message1".to_string()),
                Message::new("2".to_string(), "Content of Message2".to_string()),
                Message::new("3".to_string(), "Content of Message3".to_string())
            ]
        );
    }
}

