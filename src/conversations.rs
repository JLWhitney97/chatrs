use async_openai::types::CreateChatCompletionRequest;
use serde_json;
use std::fs;

pub fn input_request(file_name: &str) -> CreateChatCompletionRequest {
    let request_bytes = match fs::read_to_string(&file_name) {
        Ok(req_b) => req_b,
        Err(error) => panic!(
            "The following Error has occured while attempting to read {}: {}",
            file_name, error
        ),
    };
    let request = match serde_json::from_str::<CreateChatCompletionRequest>(&request_bytes){
        Ok(req) => req,
        Err(_) => panic!("There was an error deserializing your previous chat.  Please ensure it is in the proper JSON format and try again.")
    };
    request
}

pub fn output_request(request: &CreateChatCompletionRequest, file_name: &str) {
    let request_json = serde_json::to_string_pretty(&request).expect("Could not Json Encode");
    match fs::write(&file_name, &request_json) {
        Ok(_) => println!("Chat saved as {}\n", &file_name),
        Err(error) => println!("There was an error output the chat log: {}", error),
    };
}
