# playback

a simple workflow manager

## how do i install it?

install it via `cargo install playback` and use it via the `pb` command.

## how do i use it?

1. create a `.playback` folder
2. create a `workflows` folder inside of `.playback`
3. create a `.toml` file based on the following example:

```toml
name = "example"
description = "example playback workflow"
version = "0.1.0"
author = "user"

[workflow.env]
shell = "pwsh"

[workflow.arguments.message]
name = "arg1"
type = "positional"
required = true

[workflow.steps.step1]
command = "echo"
arguments = ['{{arg1}}']
order = 1
```

4. run it using `pb run example "input"`

## why should i use this?

`playback` is mostly a research project, and is half-baked (atleast, for right now). don't use playback if you can't handle breaking changes!