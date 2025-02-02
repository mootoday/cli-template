# cli-template

A Rust CLI template that follows clig.dev best practices. Use this to start your own CLI.

## Purpose

The CLI in this repository attempts to follow all [clig.dev](https://clig.dev/) Command Line Interface Guidelines. It is meant as a starting point for anyone to develop a great CLI experience.

## Technologies

The CLI is develop with Rust and the [`clap`](https://crates.io/crates/clap) crate. For other dependencies, please refer to the [`Cargo.toml`](./Cargo.toml) file.

## Guidelines

### The Basics

[Link to clig.dev section](https://clig.dev/#the-basics)

- [x] Return zero exit code on success, non-zero on failure.
- [ ] Send output to `stdout`.
- [ ] Send messaging to `stderr`.

### Help

[Link to clig.dev section](https://clig.dev/#help)

- [x] Display help text when passed no options, the `-h` flag, or the `--help` flag.
- [ ] Display a concise help text by default.
- [x] Show full help when `-h` and `--help` is passed.
- [ ] Provide a support path for feedback and issues.
- [ ] In help text, link to the web version of the documentation.
- [ ] Lead with examples.
- [ ] If you’ve got loads of examples, put them somewhere else
- [ ] Display the most common flags and commands at the start of the help text.
- [ ] Use formatting in your help text.
- [ ] If the user did something wrong and you can guess what they meant, suggest it.
- [ ] If your command is expecting to have something piped to it and `stdin` is an interactive terminal, display help immediately and quit.

### Documentation

[Link to clig.dev section](https://clig.dev/#documentation)

- [ ] Provide web-based documentation.
- [ ] Provide terminal-based documentation.
- [ ] Consider providing man pages.

### Output

[Link to clig.dev section](https://clig.dev/#output)

- [ ] Human-readable output is paramount.
- [ ] Have machine-readable output where it does not impact usability.
- [ ] If human-readable output breaks machine-readable output, use `--plain` to display output in plain, tabular text format for integration with tools like `grep` or `awk`.
- [ ] Display output as formatted JSON if `--json` is passed.
- [ ] Display output on success, but keep it brief.
- [ ] If you change state, tell the user.
- [ ] Make it easy to see the current state of the system.
- [ ] Suggest commands the user should run.
- [ ] Actions crossing the boundary of the program’s internal world should usually be explicit.
- [ ] Increase information density—with ASCII art!
- [ ] Use color with intention.
- [ ] Disable color if your program is not in a terminal or the user requested it.
- [ ] If `stdout` is not an interactive terminal, don’t display any animations.
- [ ] Use symbols and emoji where it makes things clearer.
- [ ] By default, don’t output information that’s only understandable by the creators of the software.
- [ ] Don’t treat `stderr` like a log file, at least not by default.
- [ ] Use a pager (e.g. `less`) if you are outputting a lot of text.

### Errors

[Link to clig.dev section](https://clig.dev/#output)

- [ ] Catch errors and rewrite them for humans.
- [ ] Signal-to-noise ratio is crucial.
- [ ] Consider where the user will look first.
- [ ] If there is an unexpected or unexplainable error, provide debug and traceback information, and instructions on how to submit a bug.
- [ ] Make it effortless to submit bug reports.

### Arguments and flags

[Link to clig.dev section](https://clig.dev/#arguments-and-flags)

- [x] Prefer flags to args.
- [x] Have full-length versions of all flags.
- [ ] Only use one-letter flags for commonly used flags.
- [ ] Multiple arguments are fine for simple actions against multiple files.
- [ ] If you’ve got two or more arguments for different things, you’re probably doing something wrong.
- [ ] Use standard names for flags, if there is a standard.
- [ ] Make the default the right thing for most users.
- [ ] Prompt for user input.
- [ ] Never *require* a prompt.
- [ ] Confirm before doing anything dangerous.
- [ ] If input or output is a file, support `-` to read from `stdin` or write to `stdout`.
- [ ] If a flag can accept an optional value, allow a special word like “none.”
- [ ] If possible, make arguments, flags and subcommands order-independent.
- [ ] Do not read secrets directly from flags.

### Interactivity

[Link to clig.dev section](https://clig.dev/#interactivity)

- [ ] Only use prompts or interactive elements if stdin is an interactive terminal (a TTY).
- [ ] If `--no-input` is passed, don’t prompt or do anything interactive.
- [ ] If you’re prompting for a password, don’t print it as the user types.
- [ ] Let the user escape.

### Subcommands

[Link to clig.dev section](https://clig.dev/#subcommands)

- [ ] Be consistent across subcommands.
- [ ] Use consistent names for multiple levels of subcommand.
- [ ] Don’t have ambiguous or similarly-named commands.

### Robustness

[Link to clig.dev section](https://clig.dev/#robustness)

- [ ] Validate user input.
- [ ] Responsive is more important than fast.
- [ ] Show progress if something takes a long time.
- [ ] Do stuff in parallel where you can, but be thoughtful about it.
- [ ] Make things time out.
- [ ] Make it recoverable.
- [ ] Make it crash-only.

### Future-proofing

[Link to clig.dev section](https://clig.dev/#future-proofing)

- [ ] Keep changes additive where you can.
- [ ] Warn before you make a non-additive change.
- [ ] Changing output for humans is usually OK.
- [ ] Don’t have a catch-all subcommand.
- [ ] Don’t allow arbitrary abbreviations of subcommands.
- [ ] Don’t create a “time bomb.”

### Signals and control characters

[Link to clig.dev section](https://clig.dev/#future-proofing)

- [ ] If a user hits Ctrl-C (the INT signal), exit as soon as possible.
- [ ] If a user hits Ctrl-C during clean-up operations that might take a long time, skip them.

### Configuration

[Link to clig.dev section](https://clig.dev/#configuration)

- [ ] Follow the XDG-spec.
- [ ] If you automatically modify configuration that is not your program’s, ask the user for consent and tell them exactly what you’re doing.
- [ ] Apply configuration parameters in order of precedence.

### Environment variables

[Link to clig.dev section](https://clig.dev/#environment-variables)

- [ ] Environment variables are for behavior that *varies with the context* in which a command is run.
- [ ] For maximum portability, environment variable names must only contain uppercase letters, numbers, and underscores (and mustn’t start with a number).
- [ ] Aim for single-line environment variable values.
- [ ] Avoid commandeering widely used names.
- [ ] Check general-purpose environment variables for configuration values when possible
- [ ] Read environment variables from `.env` where appropriate.
- [ ] Don’t use `.env` as a substitute for a proper configuration file.
- [ ] Do not read secrets from environment variables.

### Naming

[Link to clig.dev section](https://clig.dev/#naming)

- [ ] Make it a simple, memorable word.
- [ ] Use only lowercase letters, and dashes if you really need to.
- [ ] Keep it short.
- [ ] Make it easy to type.

### Distribution

[Link to clig.dev section](https://clig.dev/#distribution)

- [x] If possible, distribute as a single binary.
- [ ] Make it easy to uninstall.

### Analytics

[Link to clig.dev section](https://clig.dev/#analytics)

- [ ] Do not phone home usage or crash data without consent.
- [ ] Consider alternatives to collecting analytics.
