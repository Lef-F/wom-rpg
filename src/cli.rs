use std::{env, io};

use llm_chain::{
    chains::conversation::Chain, executor, output::Output, parameters, prompt, step::Step,
};
use parser::Template;
use template::{DM_HEADER, INTRO, THE_END, USER_HEADER};
use termimad::MadSkin;
use tokio;

mod commands;
mod parser;
mod template;

use crate::{
    commands::user_commands,
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

        loop {
            let mut user = String::new();
            io::stdin()
                .read_line(&mut user)
                .expect("error: unable to read user input");

            let user_response = user.trim();

            if user_response.len() > 0 {
                let command_out = user_commands(user_response);
                if command_out.is_ok() {
                    // Pass the command output to the DM
                    step = Step::for_prompt_template(prompt!(user: &command_out.unwrap()));
                    break; // Exit the loop if a valid input is entered
                }
            }
        }

        res = chain.send_message(step, &parameters!(), &exec).await?;
    }

    skin.print_text(THE_END);
    Ok(())
}
