use std::io;

use llm_chain::{
    chains::conversation::Chain, executor, output::Output, parameters, prompt, step::Step,
};
use regex::Regex;
use template::{DM_HEADER, INTRO, MODERATOR, SYSTEM_MODE, THE_END, USER_HEADER};
use termimad::MadSkin;
use tokio;

mod template;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let skin = MadSkin::default();
    skin.print_text(INTRO);

    // Create a new ChatGPT executor.
    let exec = executor!()?;

    // Create a new Chain with the executor.
    let mut chain = Chain::new(prompt!(system: SYSTEM_MODE))?;

    let mut step = Step::for_prompt_template(prompt!(user: MODERATOR));

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
        io::stdin()
            .read_line(&mut user)
            .expect("error: unable to read user input");

        execute_commands(&user);
        step = Step::for_prompt_template(prompt!(user: &user));

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
