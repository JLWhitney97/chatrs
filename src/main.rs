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
            println!("\nWelcome to ChatCLI-rs!\n");
        }
        Err(_) => {
            println!("\nError: You must set OPENAI_API_KEY.  Please set this env variable and try again.\n");
            return;
        }
    };

    let cli = ChatCLIArgs::parse();
    let chat_loop_args = match &cli.action {
        args::Action::New => {
            chat_loop::ChatArgs::NewChat(String::from(MODEL), String::from(SYSTEM_MESSAGE))
        }
        args::Action::Resume => match &cli.file {
            Some(file_name) => {
                chat_loop::ChatArgs::PreviousChat(conversations::read_request(&file_name))
            }
            None => {
                println!("\n Please provide a file name with the -f command line option.");
                return;
            }
        },
        args::Action::Quick => {
            chat_loop::ChatArgs::NewChat(String::from(MODEL), String::from(SYSTEM_MESSAGE))
        }
    };

    let request = chat_loop::run_chat_loop(chat_loop_args).await;

    match &cli.action {
        args::Action::New => {
            //todo## Prompt to save chat.
        }
        args::Action::Resume => {
            conversations::write_request(
                &request,
                &cli.file
                    .expect("Program should have returned if file isn't provided on resume."),
            );
        }
        args::Action::Quick => {
            println!("\n Discarding coversation \n");
        }
    };
}
