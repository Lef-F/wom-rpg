use std::io;

use llm_chain::{
    chains::conversation::Chain, executor, output::Output, parameters, prompt, step::Step,
};
use termimad::MadSkin;
use tokio;

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

    let mut user = String::new();

    loop {
        let dm_answer = res.primary_textual_output().await.unwrap();
        let dm_says = DM_HEADER.replace("{{dm_says}}", dm_answer.as_str());

        skin.print_text(&dm_says);

        if res.to_string().contains("!wom-over") {
            break;
        }

        skin.print_text(USER_HEADER);

        io::stdin()
            .read_line(&mut user)
            .expect("error: unable to read user input");

        step = Step::for_prompt_template(prompt!(user: &user));

        res = chain.send_message(step, &parameters!(), &exec).await?;
    }

    skin.print_text(THE_END);
    Ok(())
}

// Definition of static text
static INTRO: &str = r#"
---

# Welcome to Wheel of Misfortune

Greetings adventurer!

Hold on *tight* as we're booting up a new adventure of **misfortune** for you.

---
# ...
"#;

static SYSTEM_MODE: &str = "
    You are a dungeon master in the Wheel of Mistortune role playing game. 
    You help the player walk through their adventure in the world of incident responce in software engineering. 
    You describe the environment that the user finds themselves in and answer their questions. 
    You start the game by asking the user for a theme. 
    Using that theme, you will simulate a realistic disaster recovery scenario.
    You will ask the user questions to which they have to give you answers.
    Based on those answers, the plot will continue until the incident is resolved.
    Only if the user ask for it you can provide additional information such as tips or hints.
    The user has access to at least the following tools: 
    - their terminal with any terminal tool
    - kibana with logs from different applications in different environments
    - AWS cloud services
    - GCP cloud services
    - the datadog monitoring service 
    - grafana monitoring dashboards
    - prometheus metrics from various systems and applications
    - airflow
    You may format your responses in Markdown.
    Once the adventure is over include the following in your final answer: `!wom-over`
";

static MODERATOR: &str = "
    The game begins.
    Ask the user about the adventure as an incident responder that they wish to have.
    I am only doing the introductions. You will speak to the user now.
";

static DM_HEADER: &str = r#"
---
## The Dungeon Master says

{{dm_says}}

"#;

static USER_HEADER: &str = r#"
---
## Your response

"#;

static THE_END: &str = r#"
---

Looks like you reached the end! 👏
*We hope to see you again soon in your next adventure!*

"#;
