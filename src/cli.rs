use std::{env, io};

use llm_chain::{
    chains::conversation::Chain, executor, output::Output, parameters, prompt, step::Step,
};
use parser::Template;
use regex::Regex;
use template::{DM_HEADER, INTRO, THE_END, USER_HEADER};
use termimad::MadSkin;
use tokio;

mod parser;
mod template;

use crate::{
    parser::read_yaml_file,
    template::{MODERATOR, SYSTEM_MODE},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    let mut template = Template {
        system_mode: SYSTEM_MODE.to_owned(),
        introduction: MODERATOR.to_owned(),
    };
    // Check if an argument is provided
    println!("{} => {}", &args.len(), &args.join(","));
    if args.len() < 2 {
        println!(
            "No path to custom scenario provided. Using default Wheel of Misfortune scenario."
        );
    } else if args.len() >= 3 {
        eprintln!(
            "Too many arguments provided {}, expected {}.",
            args.len() - 1,
            "1 or none"
        );
        return Err("Too many arguments".into());
    } else if args.len() == 2 {
        // Get the file path from the argument
        let file_path = &args[1];

        template = match read_yaml_file(file_path) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Error while opening file {}: {}", file_path, error);
                return Err("Failed to read YAML file".into());
            }
        };
    }

    let skin = MadSkin::default();
    skin.print_text(INTRO);

    // Create a new ChatGPT executor.
    let exec = executor!()?;

    // Create a new Chain with the executor.
    let mut chain = Chain::new(prompt!(system: &template.system_mode))?;

    let mut step = Step::for_prompt_template(prompt!(user: &template.introduction));

    // Execute the conversation steps.
    let mut res = chain.send_message(step, &parameters!(), &exec).await?;
    // println!("{}", res1.primary_textual_output().await.unwrap());

    loop {
        let dm_answer = res.primary_textual_output().await.unwrap();
        let dm_says = DM_HEADER.replace("{{dm_says}}", dm_answer.as_str());

        skin.print_text(&dm_says);

        if res.to_string().contains("!wom-over") {
            break;
        }

        skin.print_text(USER_HEADER);

        let mut user = String::new();
        let mut user_response;

        loop {
            io::stdin()
                .read_line(&mut user)
                .expect("error: unable to read user input");

            user_response = user.trim();
            if user_response.len() > 0 {
                break; // Exit the loop if a valid input is entered
            }
        }

        execute_commands(&user_response);
        step = Step::for_prompt_template(prompt!(user: &user_response));

        res = chain.send_message(step, &parameters!(), &exec).await?;
    }

    skin.print_text(THE_END);
    Ok(())
}

fn execute_commands(input: &str) -> () {
    let regex_pattern = r"!(\w+)";

    let regex = Regex::new(regex_pattern).unwrap();
    for captured in regex.captures_iter(input) {
        if let Some(word) = captured.get(1) {
            match word.as_str() {
                "tool" => println!("should execute: {}", word.as_str()),
                _ => println!("unknown command: {}", word.as_str()),
            }
        }
    }
}
