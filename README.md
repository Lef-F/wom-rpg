# Wheel of Misfortune - RPG 🐲

A CLI game using OpenAI as your Dungeon Master.

## Get going 🏃‍♀️

Get an API token from OpenAI and export it as an environment variable in your terminal:

```shell
export OPENAI_API_KEY="sk-..."
```

### Install `wom-rpg` ⚙️

Grab the latest [GitHub Release](https://github.com/Lef-F/wom-rpg/releases/latest) binary that fits your platform.
If you don't find one then you can [build it yourself](#build-it-yourself-🛠️) 🏗️.

Extract it with:

```shell
tar -xzvf wom-rpg.tar.gz -C ./wom-rpg/
```

You should have an executable file `wom-rpg` that you can invoke directly with:

```shell
wom-rpg
```

### Custom scenarios

You wanna create your own set of awesome scenarios to play?
You can simply create YAML files that follow the style of the [templates/default.yaml](./templates/default.yaml).

You can pass as the first parameter the path to your own custom YAML scenario:

```shell
wom-rpg path/to/custom.yaml
```

## Build it yourself 🛠️

- Assuming you have rust setup (check rust.up)
- Clone this repo and in it run: `cargo run`

## TODO

- [ ] Update the user when they have successfully passes the output of a command to the DM, so they don't wait on a stalled terminal.
- [ ] Make sure that the DM does not go into solution mode e.g. ask questions to help the user to solve their problem as if it's real.
- [ ] Allow for the user to provide handwritten context e.g. `!context-manual Just letting you know that airflow is a task orchestrator. Each task is a directed acyclic graph.`
- [ ] Allow the user to provide whole documents to the DM e.g. `!context-file path/to/document.txt`
- [x] Implement templates instead of current hardcoded statics
- [x] Allow for providing more context to the DM e.g. answering `!context-shell kubectl --help` should execute `kubectl --help` in your terminal and pass the output to the DM, letting them know that the user is giving them more context.
