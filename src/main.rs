mod args;
mod chat_loop;
mod conversations;

use args::ChatCLIArgs;
use clap::Parser;
use std::env;

//Constants for the initial request
const SYSTEM_MESSAGE: &str =
    "You are ChatCLI, a large language model trained by OpenAI. Answer as concisely as possible. You are invoked from the command line.";
const MODEL: &str = "gpt-3.5-turbo-0301";

#[tokio::main]
async fn main() {
    match env::var("OPENAI_API_KEY") {
        Ok(_) => {
            println!("Welcome to ChatCLI-rs!\n");
        }
        Err(_) => {
            println!("Error: You must set OPENAI_API_KEY.  Please set this env variable and try again.\n");
            return;
        }
    };

    let cli = ChatCLIArgs::parse();

    let chat_loop_args = match &cli.file {
        Some(file_name) => chat_loop::ChatArgs::PreviousChat(conversations::input_request(&file_name)),
        None => chat_loop::ChatArgs::NewChat(String::from(MODEL), String::from(SYSTEM_MESSAGE)),
    };

    let request = chat_loop::run_chat_loop(chat_loop_args).await;

    match &cli.file {
        Some(file_name) => {
            conversations::output_request(&request, &file_name);
        },
        None => println!("\n Discarding Chat \n"),
    };

}
