use async_openai::types::CreateChatCompletionRequest;
use std::{fs, io};
use toml;

// pub fn input_request(file_name: &str) -> Result<CreateChatCompletionRequest, io::Error>{
//     let request_bytes = fs::read_to_string(&file_name)?;
//     let request = toml::from_str(&request_bytes).expect("Was not toml");
//     Ok(request)
// }

pub fn output_request(request: &CreateChatCompletionRequest, file_name: &str){
    let request_toml = toml::to_string(&request).expect("Could not TOML Encode");
    match fs::write(&file_name, &request_toml){
        Ok(_) => println!("Chat saved as {}", &file_name),
        Err(error) => println!("There was an error output the chat log: {}",  error),
    };
}