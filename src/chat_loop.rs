use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};
use rustyline::{error::ReadlineError, DefaultEditor};

pub enum ChatArgs {
    PreviousChat(CreateChatCompletionRequest),
    NewChat(String, String),
}

pub async fn run_chat_loop(chat_args: ChatArgs) -> CreateChatCompletionRequest {
    //Create OPENAI Api Client from env variable OPENAI_API_KEY
    let client = Client::new();

    let mut request = match chat_args {
        ChatArgs::PreviousChat(req) => req,
        ChatArgs::NewChat(model, system_message) => CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages([ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system_message)
                .build()
                .expect("Default Paramaters always provided")])
            .build()
            .expect("Default Paramaters always provided"),
    };

    //Create Rustyline Editor
    let mut rl = match DefaultEditor::new() {
        Ok(rl) => rl,
        Err(err) => {
            println!("Error creating rustyline editor, please try again: {}", err);
            return request;
        }
    };

    loop {
        //Read line from stdin
        let readline = match rl.readline("\n<   You   > ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {
                println!("\nEnding Conversation");
                return request;
            }
            Err(err) => {
                println!("There was an error, please try again: {}", err);
                return request;
            }
        };
        //Evaluate

        //Append new message to request
        request.messages.push(
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(readline)
                .build()
                .expect("Values are always provided."),
        );

        //Query response
        let response = match client.chat().create(request.to_owned()).await {
            Ok(response) => response,
            Err(error) => {
                println!(
                    "*** There was an error sending your last message.  {} ***",
                    error
                );
                request.messages.pop();
                continue;
            }
        };

        //Ensure there was a response from OpenAI, if not, remove the previous request and ask for input
        let response_message = match response.choices.get(0) {
            Some(message) => &message.message.content,
            None => {
                println!("\n*** There was an error reading the response from OpenAI. You may retry  your previous message.. ***\n");
                request.messages.pop();
                continue;
            }
        };

        //Output Response
        println!("\n< ChatCLI > {}", &response_message);

        //Append response to request message
        request.messages.push(
            ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content(response_message)
                .build()
                .expect("Values are always provided."),
        );
    }
}