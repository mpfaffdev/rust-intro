use rust_intro::rust_intro::DataMessage;

pub fn create_data_message(id: String, msg: String) -> DataMessage {
    DataMessage { id, content: msg }
}
