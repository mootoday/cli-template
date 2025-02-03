---
default: minor
---

# Provide a `feedback` command

This is also invoked in case an error happens in the CLI. In that case, the wording is adjusted to ask for feedback related to the error vs. open-ended feedback. In the case of error feedback, the CLI prompts whether a user wants to provide feedback or not. It also offers a "Don't ask again" option, in which case that preference is stored in a config file and respected going forward.