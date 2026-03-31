pub const BUILD_SWITCH_PROMPT: &str = "Switching to build mode. You can now execute the plan.";

pub const COMPACT_SYSTEM_PROMPT: &str = r#"You are a helpful AI assistant tasked with summarizing conversations.

When asked to summarize, provide a detailed but concise summary of the conversation.
Focus on information that would be helpful for continuing the conversation, including:
- What was done
- What is currently being worked on
- Which files are being modified
- What needs to be done next
- Key user requests, constraints, or preferences that should persist
- Important technical decisions and why they were made

Your summary should be comprehensive enough to provide context but concise enough to be quickly understood.

Do not respond to any questions in the conversation, only output the summary."#;

pub const COMPACT_USER_PROMPT: &str = r#"Provide a detailed summary of our conversation above.
Focus on information that would be helpful for continuing the conversation, including what we did, what we're doing, which files we're working on, and what we're going to do next.
The summary that you construct will be used so that another agent can read it and continue the work.

When constructing the summary, try to stick to this template:
---
## Goal
[What goal(s) is the user trying to accomplish?]

## Instructions
[What important instructions did the user give you that are relevant]

## Discoveries
[What notable things were learned during this conversation that would be useful for the next agent to know when continuing the work]

## Accomplished
[What work has been completed, what work is still in progress, and what work is left?]

## Relevant files / directories
[Construct a structured list of relevant files that have been read, edited, or created that pertain to the task at hand. If all the files in a directory are relevant, include the path to the directory.]
---"#;

pub fn get_compact_system_prompt() -> &'static str {
    COMPACT_SYSTEM_PROMPT
}

pub fn get_compact_user_prompt() -> &'static str {
    COMPACT_USER_PROMPT
}
