# sway-titlebar-toggle

Toggle Titlebar in SwayWM

* Written in [Rust](https://www.rust-lang.org/) using
[SwayIPC](https://github.com/jaycefayne/swayipc-rs) for [SwayWM](https://github.com/swaywm/sway)

## Note

**Currently Only Written to check if current titlebar is set to *'normal'***

* Will add configuration options in the future

:pushpin: **Accepting Pull Requests**

## Executable

Located in [Releases](https://github.com/amazingefren/sway-titlebar-toggle/releases)

## Installation via Source

### Dependencies

**[SwayWM](https://github.com/swaywm/sway)**

**[Rust](https://www.rust-lang.org/) (Installer: [Rustup](https://rustup.rs/))**

``` zsh
# clone directory to your choosing
git clone https://github.com/amazingefren/sway-titlebar-toggle.git
# cd into directory
cd ./sway-titlebar-toggle
```

#### (Optional) Change Toggle Styling in src/main.rs

* Available Commands

    ``` zsh
    # Sway(5) man page, section: COMMANDS
    man 5 sway
    ```

* Apply Commands

    ``` rust
    // src/main.rs (lines 23 & 26 *subject to change)
    connection.run_command("{YOUR_COMMAND_HERE}")
    ```

### (Optional) Build

``` zsh
# sway-titlebar-toggle/
cargo build
```

### (Optional) Test

* **Will run on whatever is the currently focused window
(most likely the terminal you're in)**

    ``` zsh
    # sway-titlebar-toggle/
    cargo run 
    ```

### Create Final Executable

``` zsh
# Executable will be ./target/release/sway-titlebar-toggle
cargo build --release
```

## Recommended

* Copy the executable to your bin (or /usr/local/bin/.)

    ```zsh
    sudo cp ./target/release/sway-titlebar-toggle {BIN_PATH}
    ```

* Add to SwayWM config

    ``` ini
    # mine for reference ($HOME/.config/sway/config)
    bindsym $mod+Shift+p exec sway-titlebar-toggle 
    ```

## Credits

* [JayceFayne](https://github.com/JayceFayne) for [SwayIPC](https://github.com/jaycefayne/swayipc-rs)

* [Drew DeVault](https://github.com/ddevault) &
[Contributors](https://github.com/swaywm/sway/graphs/contributors)
for [Sway](https://github.com/swaywm/sway)
