## iitbh_smart_atm_jihyo: XAMPP like tool to start and manager our database and other server(s) 

![img](https://img.shields.io/github/license/radcolor/iitbh_smart_atm_jihyo)

 XAMPP like tool to start and manager our database and other server(s), written in rust and svelte! 

### Setting up the system (UNIX)

```bash
# For Arch and Arch based distros
sudo pacman -Syu
sudo pacman -S --needed webkit2gtk base-devel curl wget file openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips

# For Debian based distros
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### Setting the development tools and dependencies

Install the rust-lang and all dependencies then run the below command to compile and run the:

```bash
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install tauri and  tauri-cli etc
cargo install create-tauri-app tauri-cli

# Install bun
curl -fsSL https://bun.sh/install | bash
```

### Starting the Terminal

Run the below command in the project root dir to install frontend dependencies and start the terminal in dev mode.

```bash
# Then use bun and cargo commands to install other dependencies and run
bun install
cargo tauri dev
```

### Running screenshots

![img](/app_ss.png)

### Reporting bug or feature request

You can easily report a bug or request a feature by opening a [pull request](https://github.com/radcolor/iitbh_smart_atm_jihyo/compare) or [opening an issue](https://github.com/radcolor/iitbh_smart_atm_jihyo/issues/new/choose)

**How to report a bug/issue**

- Make sure of taking a logs in detail
- Make sure no other similar bugs already reported

**How to request a feature**

- A detail description of feature

### Contributing

Read [Contributing.md](https://github.com/radcolor/iitbh_smart_atm_jihyo/blob/master/CONTRIBUTING.md) to get the application running locally and various ways to help us on development.

iitbh_smart_atm_jihyo Copyright (c) 2023 Shashank Baghel, IBITF. All rights reserved.
