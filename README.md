# Wheel of Misfortune - RPG

A CLI game using OpenAI as your Dungeon Master.

## Get going

Get an API token from OpenAI and export it as an environment variable in your terminal:

```shell
export OPENAI_API_KEY="sk-..."
```

- Assuming you have rust setup (check rust.up)
- Clone this repo and in it run: `cargo run`

## TODO

- [ ] Make sure that the DM does not go into solution mode e.g. ask questions to help the user to solve their problem as if it's real.
- [ ] Allow for providing more context to the DM e.g. answering `!context-shell kubectl --help` should execute `kubectl --help` in your terminal and pass the output to the DM, letting them know that the user is giving them more context.
- [ ] Allow for the user to provide handwritten context e.g. `!context-manual Just letting you know that airflow is a task orchestrator. Each task is a directed acyclic graph.`
- [ ] Allow the user to provide whole documents to the DM e.g. `!context-file path/to/document.txt`
- [x] Implement templates instead of current hardcoded statics
