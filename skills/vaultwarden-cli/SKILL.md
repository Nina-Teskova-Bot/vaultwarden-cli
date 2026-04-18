---
name: vaultwarden-cli
description: Use Vaultwarden CLI safely and efficiently for login, unlock, lookup, URI matching, env injection, status checks, and YAML interpolation. Use when you need to query or inject secrets through `vaultwarden-cli`, especially after discovering the CLI via `vaultwarden-cli skill list` or `vaultwarden-cli skill describe <command>`.
---

# Vaultwarden CLI

Use the CLI's self-description first when you need an exact command contract:

```bash
vaultwarden-cli skill list
vaultwarden-cli skill describe unlock
vaultwarden-cli skill describe run
```

## Safe workflow

1. Run `vaultwarden-cli status` to see whether the CLI is logged in and unlocked.
2. If needed, run `vaultwarden-cli login`.
3. If needed, run `vaultwarden-cli unlock`.
4. Prefer the narrowest read command that solves the task:
   - `list` for discovery
   - `get` for direct item lookup
   - `get-uri` for hostname/URI lookup
   - `run` or `run-uri` for ephemeral env injection into a child process
   - `interpolate` for YAML placeholder replacement

## Important notes

- Prefer environment variables over inline secrets where possible.
- `run` and `run-uri` require `--` before the child command.
- `skill describe <command>` returns machine-readable JSON for automation and agents.
- A future fallback path may use `<tool> <command> --help --json`, but the canonical interface here is `skill list` and `skill describe`.
