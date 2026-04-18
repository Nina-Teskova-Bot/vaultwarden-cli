use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct CommandDescriptor {
    name: &'static str,
    summary: &'static str,
    usage: &'static str,
    examples: &'static [&'static str],
    arguments: &'static [ArgumentDescriptor],
    options: &'static [OptionDescriptor],
    environment: &'static [EnvVarDescriptor],
    notes: &'static [&'static str],
}

#[derive(Serialize)]
pub struct ArgumentDescriptor {
    name: &'static str,
    required: bool,
    description: &'static str,
}

#[derive(Serialize)]
pub struct OptionDescriptor {
    name: &'static str,
    short: Option<&'static str>,
    value_name: Option<&'static str>,
    description: &'static str,
}

#[derive(Serialize)]
pub struct EnvVarDescriptor {
    name: &'static str,
    applies_to: &'static str,
    description: &'static str,
}

const NO_ARGS: &[ArgumentDescriptor] = &[];
const NO_OPTIONS: &[OptionDescriptor] = &[];
const NO_ENV: &[EnvVarDescriptor] = &[];

const COMMANDS: &[CommandDescriptor] = &[
    CommandDescriptor {
        name: "login",
        summary: "Authenticate to the Vaultwarden server with an API client id and secret.",
        usage: "vaultwarden-cli login [--server <url>] [--client-id <id>] [--client-secret <secret>]",
        examples: &[
            "vaultwarden-cli login --server https://vault.example.com --client-id xxx --client-secret yyy",
            "VAULTWARDEN_CLIENT_ID=xxx VAULTWARDEN_CLIENT_SECRET=yyy vaultwarden-cli login --server https://vault.example.com",
        ],
        arguments: NO_ARGS,
        options: &[
            OptionDescriptor {
                name: "--server",
                short: Some("-s"),
                value_name: Some("url"),
                description: "Vaultwarden server URL.",
            },
            OptionDescriptor {
                name: "--client-id",
                short: None,
                value_name: Some("id"),
                description: "OAuth-style API client id.",
            },
            OptionDescriptor {
                name: "--client-secret",
                short: None,
                value_name: Some("secret"),
                description: "API client secret. Prefer environment variables or secure prompts in automation.",
            },
        ],
        environment: &[
            EnvVarDescriptor {
                name: "VAULTWARDEN_CLIENT_ID",
                applies_to: "login",
                description: "Fallback for --client-id.",
            },
            EnvVarDescriptor {
                name: "VAULTWARDEN_CLIENT_SECRET",
                applies_to: "login",
                description: "Fallback for --client-secret.",
            },
        ],
        notes: &[
            "Saves the access token and related config locally.",
            "Does not unlock the vault. Run `vaultwarden-cli unlock` after login.",
        ],
    },
    CommandDescriptor {
        name: "unlock",
        summary: "Unlock the local vault session with the master password.",
        usage: "vaultwarden-cli unlock [--password <password>]",
        examples: &[
            "vaultwarden-cli unlock",
            "VAULTWARDEN_PASSWORD=supersecret vaultwarden-cli unlock",
        ],
        arguments: NO_ARGS,
        options: &[OptionDescriptor {
            name: "--password",
            short: Some("-p"),
            value_name: Some("password"),
            description: "Master password. If omitted, falls back to VAULTWARDEN_PASSWORD, then prompts interactively.",
        }],
        environment: &[EnvVarDescriptor {
            name: "VAULTWARDEN_PASSWORD",
            applies_to: "unlock",
            description: "Fallback for --password.",
        }],
        notes: &[
            "Requires an existing login session.",
            "Decrypts the local vault key and organization keys for later commands.",
        ],
    },
    CommandDescriptor {
        name: "lock",
        summary: "Forget local decryption keys without logging out of the server.",
        usage: "vaultwarden-cli lock",
        examples: &["vaultwarden-cli lock"],
        arguments: NO_ARGS,
        options: NO_OPTIONS,
        environment: NO_ENV,
        notes: &["Clears saved decryption keys from local state."],
    },
    CommandDescriptor {
        name: "logout",
        summary: "Clear the login session and stored client secret.",
        usage: "vaultwarden-cli logout",
        examples: &["vaultwarden-cli logout"],
        arguments: NO_ARGS,
        options: NO_OPTIONS,
        environment: NO_ENV,
        notes: &["Removes local auth state and best-effort stored client secret."],
    },
    CommandDescriptor {
        name: "list",
        summary: "List vault items with optional filters.",
        usage: "vaultwarden-cli list [--type <type>] [--search <term>] [--org <org>] [--collection <collection>] [--json]",
        examples: &[
            "vaultwarden-cli list --type login --json",
            "vaultwarden-cli list --search github --org Engineering",
        ],
        arguments: NO_ARGS,
        options: &[
            OptionDescriptor {
                name: "--type",
                short: Some("-t"),
                value_name: Some("type"),
                description: "Filter by item type: login, note, card, identity, ssh.",
            },
            OptionDescriptor {
                name: "--json",
                short: None,
                value_name: None,
                description: "Emit JSON instead of human-readable output.",
            },
            OptionDescriptor {
                name: "--search",
                short: Some("-s"),
                value_name: Some("term"),
                description: "Full-text search term.",
            },
            OptionDescriptor {
                name: "--org",
                short: None,
                value_name: Some("org"),
                description: "Organization name or id filter.",
            },
            OptionDescriptor {
                name: "--collection",
                short: Some("-c"),
                value_name: Some("collection"),
                description: "Collection name or id filter.",
            },
        ],
        environment: NO_ENV,
        notes: &["Requires an unlocked vault."],
    },
    CommandDescriptor {
        name: "get",
        summary: "Fetch a specific item by id or display name.",
        usage: "vaultwarden-cli get <item> [--format <json|env|value|username>] [--username] [--password] [--org <org>] [--collection <collection>]",
        examples: &[
            "vaultwarden-cli get github --format env",
            "vaultwarden-cli get my-secret --password",
        ],
        arguments: &[ArgumentDescriptor {
            name: "item",
            required: true,
            description: "Item id or display name.",
        }],
        options: &[
            OptionDescriptor {
                name: "--format",
                short: Some("-f"),
                value_name: Some("format"),
                description: "Output format: json, env, value, username.",
            },
            OptionDescriptor {
                name: "--username",
                short: Some("-u"),
                value_name: None,
                description: "Shorthand for --format username.",
            },
            OptionDescriptor {
                name: "--password",
                short: Some("-p"),
                value_name: None,
                description: "Shorthand for --format value.",
            },
            OptionDescriptor {
                name: "--org",
                short: None,
                value_name: Some("org"),
                description: "Organization name or id filter.",
            },
            OptionDescriptor {
                name: "--collection",
                short: None,
                value_name: Some("collection"),
                description: "Collection name or id filter.",
            },
        ],
        environment: NO_ENV,
        notes: &["Requires an unlocked vault."],
    },
    CommandDescriptor {
        name: "get-uri",
        summary: "Fetch the best matching item for a URI or hostname.",
        usage: "vaultwarden-cli get-uri <uri> [--format <json|env|value|username>] [--username] [--password] [--org <org>] [--collection <collection>]",
        examples: &[
            "vaultwarden-cli get-uri github.com --username",
            "vaultwarden-cli get-uri https://app.example.com --format json",
        ],
        arguments: &[ArgumentDescriptor {
            name: "uri",
            required: true,
            description: "URI or hostname to match.",
        }],
        options: &[
            OptionDescriptor {
                name: "--format",
                short: Some("-f"),
                value_name: Some("format"),
                description: "Output format: json, env, value, username.",
            },
            OptionDescriptor {
                name: "--username",
                short: Some("-u"),
                value_name: None,
                description: "Shorthand for --format username.",
            },
            OptionDescriptor {
                name: "--password",
                short: Some("-p"),
                value_name: None,
                description: "Shorthand for --format value.",
            },
            OptionDescriptor {
                name: "--org",
                short: None,
                value_name: Some("org"),
                description: "Organization name or id filter.",
            },
            OptionDescriptor {
                name: "--collection",
                short: None,
                value_name: Some("collection"),
                description: "Collection name or id filter.",
            },
        ],
        environment: NO_ENV,
        notes: &["Requires an unlocked vault."],
    },
    CommandDescriptor {
        name: "run",
        summary: "Run a child process with selected secrets injected as environment variables.",
        usage: "vaultwarden-cli run [--name <item>] [--org <org>] [--folder <folder>] [--collection <collection>] [--info] -- <command> [args...]",
        examples: &[
            "vaultwarden-cli run --name GitHub -- env | grep GITHUB",
            "vaultwarden-cli run --credential-name My App -- npm run deploy",
        ],
        arguments: &[ArgumentDescriptor {
            name: "command",
            required: false,
            description: "Trailing child command after `--`.",
        }],
        options: &[
            OptionDescriptor {
                name: "--name",
                short: None,
                value_name: Some("item"),
                description: "Item name or id to inject. Alias: --credential-name.",
            },
            OptionDescriptor {
                name: "--org",
                short: None,
                value_name: Some("org"),
                description: "Organization name or id filter.",
            },
            OptionDescriptor {
                name: "--folder",
                short: None,
                value_name: Some("folder"),
                description: "Folder name or id filter.",
            },
            OptionDescriptor {
                name: "--collection",
                short: None,
                value_name: Some("collection"),
                description: "Collection name or id filter.",
            },
            OptionDescriptor {
                name: "--info",
                short: Some("-i"),
                value_name: None,
                description: "Print injected variable names without values.",
            },
        ],
        environment: NO_ENV,
        notes: &[
            "Requires an unlocked vault.",
            "Use `--` before the child command so clap stops parsing vaultwarden-cli flags.",
        ],
    },
    CommandDescriptor {
        name: "run-uri",
        summary: "Run a child process with secrets selected by URI match.",
        usage: "vaultwarden-cli run-uri <uri> [--info] -- <command> [args...]",
        examples: &[
            "vaultwarden-cli run-uri github.com -- env | grep GITHUB",
            "vaultwarden-cli run-uri https://app.example.com -- ./deploy.sh",
        ],
        arguments: &[
            ArgumentDescriptor {
                name: "uri",
                required: true,
                description: "URI or hostname to match.",
            },
            ArgumentDescriptor {
                name: "command",
                required: false,
                description: "Trailing child command after `--`.",
            },
        ],
        options: &[OptionDescriptor {
            name: "--info",
            short: Some("-i"),
            value_name: None,
            description: "Print injected variable names without values.",
        }],
        environment: NO_ENV,
        notes: &[
            "Requires an unlocked vault.",
            "Use `--` before the child command so clap stops parsing vaultwarden-cli flags.",
        ],
    },
    CommandDescriptor {
        name: "status",
        summary: "Show whether the CLI is logged in and whether the vault is unlocked.",
        usage: "vaultwarden-cli status",
        examples: &["vaultwarden-cli status"],
        arguments: NO_ARGS,
        options: NO_OPTIONS,
        environment: NO_ENV,
        notes: &["Safe diagnostic command. Does not require unlock."],
    },
    CommandDescriptor {
        name: "interpolate",
        summary: "Replace vault placeholders inside a YAML file.",
        usage: "vaultwarden-cli interpolate --file <path> [--output <path>] [--skip-missing]",
        examples: &[
            "vaultwarden-cli interpolate --file config.yml",
            "vaultwarden-cli interpolate --file config.yml --output rendered.yml --skip-missing",
        ],
        arguments: NO_ARGS,
        options: &[
            OptionDescriptor {
                name: "--file",
                short: Some("-f"),
                value_name: Some("path"),
                description: "YAML file to render.",
            },
            OptionDescriptor {
                name: "--output",
                short: Some("-o"),
                value_name: Some("path"),
                description: "Write rendered YAML to a file instead of stdout.",
            },
            OptionDescriptor {
                name: "--skip-missing",
                short: Some("-s"),
                value_name: None,
                description: "Leave unresolved placeholders unchanged instead of failing.",
            },
        ],
        environment: NO_ENV,
        notes: &["Requires an unlocked vault."],
    },
    CommandDescriptor {
        name: "skill",
        summary: "Describe the CLI for agents and automation.",
        usage: "vaultwarden-cli skill <list|describe>",
        examples: &[
            "vaultwarden-cli skill list",
            "vaultwarden-cli skill describe unlock",
        ],
        arguments: &[ArgumentDescriptor {
            name: "subcommand",
            required: true,
            description: "Either `list` or `describe <command>`.",
        }],
        options: NO_OPTIONS,
        environment: NO_ENV,
        notes: &[
            "`skill list` emits command names, one per line.",
            "`skill describe <command>` emits a JSON description of the command contract.",
        ],
    },
];

pub fn print_command_list() {
    for command in command_names() {
        println!("{command}");
    }
}

pub fn command_names() -> impl Iterator<Item = &'static str> {
    COMMANDS.iter().map(|entry| entry.name)
}

pub fn has_command(command_name: &str) -> bool {
    COMMANDS.iter().any(|command| command.name == command_name)
}

pub fn print_command_description(command_name: &str) -> Result<()> {
    let descriptor = COMMANDS
        .iter()
        .find(|command| command.name == command_name)
        .with_context(|| {
            let available = COMMANDS
                .iter()
                .map(|command| command.name)
                .collect::<Vec<_>>()
                .join(", ");
            format!("Unknown command '{command_name}'. Available commands: {available}")
        })?;

    println!(
        "{}",
        serde_json::to_string_pretty(descriptor)
            .context("Failed to serialize command description")?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_list_contains_unlock() {
        assert!(COMMANDS.iter().any(|command| command.name == "unlock"));
    }

    #[test]
    fn describe_unknown_command_returns_error() {
        let error = print_command_description("missing-command").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Unknown command 'missing-command'")
        );
    }
}
