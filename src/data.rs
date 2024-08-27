use std::{
    fs,
    io::{Cursor, Read, Write}
};
use rust_intro::rust_intro::DataMessage;
use prost::Message;

const FILE: &str = "data.persist";

#[derive(Clone)]
pub(super) struct DataHandler {
    data: Vec<DataMessage>,
}

impl DataHandler {
    fn deserialize_multiple(buf: &[u8]) -> Vec<DataMessage> {
        let mut cursor = Cursor::new(buf);

        let mut messages = Vec::new();

        while cursor.position() < buf.len() as u64 {
            // can decode data which has been encoded by encode_length_delimited!
            // this is the easiest way to decode multiple messages, as a regular decode would consume the buffer
            let msg = DataMessage::decode_length_delimited(&mut cursor).expect("could not decode protobuf!");
            messages.push(msg);
        }

        messages
    }

    pub fn new() -> Self {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(false)
            .open(FILE)
            .expect("could not open file");

        let mut content = vec![];

        file.read_to_end(&mut content).expect("could not read from file");

        let current_data = Self::deserialize_multiple(content.as_slice());

        Self { data: current_data }
    }

    pub fn add(&mut self, data: DataMessage) {
        // prevent duplicates
        if self.data.iter().any(|x| x.id == data.id) {
            return;
        }

        let mut file = fs::OpenOptions::new().append(true).open(FILE).expect("could not open file");

        // encode the data length delimited to allow length delimited decoding
        file.write_all(data.encode_length_delimited_to_vec().as_slice()).expect("could not write to file");

        self.data.push(data);
    }

    pub fn get(&self, id: &String) -> Option<DataMessage> {
        Some(self.data.iter().find(|x| x.id == *id)?.clone())
    }
}