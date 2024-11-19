# ZSH Alias Tips - in [Rust][5]

A [Rust][5] implementation of the [Alias Tips][8]'s ZSH plugin helps remember aliases by displaying a help message when you use a command for which an alias exists.

[![MIT license](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=flat-square)](LICENSE) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg?style=flat-square)][2] [![Editor Config](https://img.shields.io/badge/Editor%20Config-1.0.1-crimson.svg?style=flat-square)][3] [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?logo=conventional-commits&style=flat-square)][4] [![Rust](https://img.shields.io/badge/Rust-1.55.0-DEA584.svg?logo=rust&style=flat-square)][5] [![ZSH](https://img.shields.io/badge/ZSH-5.9-1A2C34.svg?logo=zsh&style=flat-square)][7]

## FEATURES

- Shows tips for aliases when you use the full command
- Supports both shell aliases and git aliases
- Customizable appearance and behavior
- Written in [Rust][5] for better performance
- Comprehensive test coverage

## INSTALLATION

```sh
git clone https://github.com/roalcantara/zsh_alias_tips
```

### Prerequisites

- [Rust][6] toolchain (cargo)
- [ZSH][7] shell

## CONFIGURATION

The following environment variables can be set to customize the behavior:

- `ZSH_PLUGINS_ALIAS_TIPS_TEXT`: The prefix of alias tips _(default: "Alias tip: ")_
- `ZSH_PLUGINS_ALIAS_TIPS_EXCLUDES`: Space-separated list of aliases to exclude from suggestions
- `ZSH_PLUGINS_ALIAS_TIPS_EXPAND`: Set to "0" to disable expansion of aliases _(default: "1")_
- `ZSH_PLUGINS_ALIAS_TIPS_FORCE`: Set to "1" to force use of aliases _(default: "0")_
- `ZSH_PLUGINS_ALIAS_TIPS_REVEAL`: Set to "1" to reveal command expansion _(default: "0")_
- `ZSH_PLUGINS_ALIAS_TIPS_REVEAL_TEXT`: The prefix for revealed commands _(default: "Alias for: ")_
- `ZSH_PLUGINS_ALIAS_TIPS_REVEAL_EXCLUDES`: Space-separated list of aliases to exclude from revealing

Example configuration in `~/.zshrc`:

```sh
export ZSH_PLUGINS_ALIAS_TIPS_TEXT=" Alias tip: "
export ZSH_PLUGINS_ALIAS_TIPS_EXCLUDES="_ c"
export ZSH_PLUGINS_ALIAS_TIPS_EXPAND=0
```

## USAGE

Once installed, the plugin will automatically show tips when you use a command that has an alias:

```sh
$ git status
 Alias tip: gs
```

## DEVELOPMENT

### Running Tests

```sh
cargo test
```

### Building

```sh
cargo build --release
```

### Step by Step: Manual Project Creation

If you want to create this project from scratch, here's a detailed guide:

1. Create a new Rust project:

    ```sh
    # Create project directory
    mkdir zsh_alias_tips
    cd zsh_alias_tips

    # Initialize new Rust project
    cargo init --name zsh_alias_tips
    ```

2. Add dependencies to `Cargo.toml`:

    ```toml
    [package]
    name = "zsh_alias_tips"
    version = "0.1.0"
    edition = "2021"
    authors = ["Your Name <your.email@example.com>"]
    description = "A Rust implementation of the ZSH Alias Tips plugin that helps remembering aliases"
    license = "MIT"

    [dependencies]
    ansi_term = "0.12"
    clap = { version = "4.4", features = ["derive"] }
    regex = "1.10"

    [dev-dependencies]
    pretty_assertions = "1.4"
    ```

3. Create the main Rust implementation in `src/main.rs`:

    ```rust
    // src/main.rs
    use std::env;
    use std::io::{self, BufRead};
    use std::process;
    use ansi_term::Colour::Blue;
    use clap::Parser;
    use regex::Regex;

    // ... rest of the implementation
    ```

4. Create the ZSH plugin file `zsh_alias_tips.plugin.zsh`:

    ```sh
    #!/usr/bin/env zsh

    _zsh_alias_tips__PLUGIN_DIR=${0:a:h}

    # Plugin implementation
    # ... rest of the plugin code
    ```

    5. Write tests in `src/main.rs`:

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_parse_aliases() {
            // ... test implementation
        }
        // ... more tests
    }
    ```

6. Build and test the project:

    ```sh
    # Run tests
    cargo test

    # Build release version
    cargo build --release
    ```

7. Create project documentation:

    ```sh
    # Create README.md with project documentation
    touch README.md

    # Create LICENSE file (MIT License)
    touch LICENSE
    ```

8. Set up Git repository:

    ```sh
    git init
    git add .
    git commit -m "Initial commit: Rust implementation of ZSH alias-tips"
    ```

9. Test the plugin:

    ```sh
    # Create a test ZSH session
    zsh

    # Source the plugin
    source /path/to/zsh_alias_tips.plugin.zsh

    # Test with some aliases
    alias g='git'
    git status  # Should show tip: g status
    ```

10. Debugging tips:

    - Use `cargo run -- "command"` to test the binary directly
    - Use `cargo test -- --nocapture` to see test output
    - Check ZSH plugin output with `set -x` for debugging
    - Monitor stderr with `cargo run 2>debug.log`

11. Common issues and solutions:

    - If the plugin doesn't load, check file permissions (`chmod +x zsh_alias_tips.plugin.zsh`)
    - If colors don't work, ensure terminal supports ANSI colors
    - If binary not found, check the path in `zsh_alias_tips.plugin.zsh`
    - For git aliases, ensure git is in PATH

12. Performance optimization:

    - Always use release build in production
    - Profile with `cargo build --release -- -Z time-passes`
    - Consider using `cargo flamegraph` for performance analysis

### Building from Source

#### 1. Clone the repository

```sh
git clone https://github.com/roalcantara/zsh_alias_tips.git
cd zsh_alias_tips
```

#### 2. Build the Rust binary

```sh
cargo build --release
```

#### 3. Add the plugin to your ZSH configuration

- If you use a plugin manager, add the plugin according to your manager's instructions. For example, with Oh My Zsh:

    1. Copy or symlink this repository to `$ZSH_CUSTOM/plugins/zsh_alias_tips`

    2. Add `zsh_alias_tips` to your plugin list in `~/.zshrc`:

        ```sh
        plugins=(... zsh_alias_tips)
        ```

- If you don't use a plugin manager, you can source the plugin directly in your `~/.zshrc`:

    ```sh
    source /path/to/zsh_alias_tips/zsh_alias_tips.plugin.zsh
    ```

## LICENSE

The project is available as open source under the terms of the [MIT][1] [License](LICENSE)

## CONTRIBUTING

- Bug reports and pull requests are welcome on [GitHub][0]
- Do follow [Editor Config][3] rules.
- Everyone interacting in the project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [Contributor Covenant][2] code of conduct.

## ACKNOWLEDGMENTS

- Original [alias-tips ZSH plugin][8].

## REFERENCES

- [The Rust Programming Language][5]
- [Z Shell][7]
- [Conventional Commits][4]

[0]: https://github.com/roalcantara/zsh_alias_tips "ZSH Alias Tips (Rust Implementation)"
[1]: https://opensource.org/licenses/MIT "MIT License"
[2]: https://contributor-covenant.org 'A Code of Conduct for Open Source Communities'
[3]: https://editorconfig.org 'EditorConfig'
[4]: https://conventionalcommits.org 'Conventional Commits'
[5]: https://rust-lang.org "The Rust Programming Language"
[6]: https://rust-lang.org/tools/install "Install Rust"
[7]: https://zsh.org "Z Shell"
[8]: https://github.com/djui/alias-tips "Original ZSH Alias Tips Plugin"
