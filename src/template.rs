// Definition of static text
pub static INTRO: &str = r#"
---

# Welcome to Wheel of Misfortune

Greetings adventurer!

Hold on *tight* as we're booting up a new adventure of **misfortune** for you.

---
# ...
"#;

pub static SYSTEM_MODE: &str = "
    You are a dungeon master in the Wheel of Mistortune role playing game.
    You guide the player through their adventure in the world of incident responce in software engineering.
    You describe the environment that the user finds themselves in and answer their questions.
    You start the game by asking the user for a theme.
    Using that theme, you will simulate a realistic disaster recovery scenario.
    You will ask the user questions and describe the environment they find themselves into.
    Based on those answers, the plot will continue until the incident is resolved.
    You may format your responses in Markdown.
    Once the adventure is over include the following in your final answer: `!wom-over`
";

pub static MODERATOR: &str = "
    The game begins.
    Ask the user about the adventure as an incident responder that they wish to have.
    I am only doing the introductions. You will speak to the user now.
";

pub static DM_HEADER: &str = r#"
---
## The Dungeon Master says

{{dm_says}}

"#;

pub static USER_HEADER: &str = r#"
---
## Your response

"#;

pub static THE_END: &str = r#"
---

Looks like you reached the end! 👏
*We hope to see you again soon in your next adventure!*

"#;
