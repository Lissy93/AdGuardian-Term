<h1 align="center">AdGuardian-Term</h1>
<p align="center">
	<i>Terminal-based, real-time traffic monitoring and statistics for your AdGuard Home instance</i>
</p>
<p align="center">
  <img width="200" src="https://i.ibb.co/25GNT0n/adguardian-banner-4.png" />
</p>

## About

AdGuardian Terminal Eddition - Keep an eye on your traffic, with this (unofficial) buddy for your AdGuard Home instance

<p align="center">
<img width="600" src="https://i.ibb.co/Nrtd01d/adguardian-demo.gif?" >
</p>

#### Features
- **Real-time Query Monitoring**: _Fetches and displays all DNS queries in real time, letting you see exactly what's happening on your network at any given moment_
- **Block and Allow Stats**: _Get a quick overview of the number of queries that have been allowed, filtered or blocked by AdGuard_
- **Historical Query Counts**: _Analyze network activity over time with historical query count data. This lets you track trends and spot any unusual activity_
- **Filter Lists**: _AdGuardian displays your active filter lists, showing which ones are doing the most work_
- **Top Domain Statistics**: _See which domains are getting the most queries (blocked, allowed and otherwise) in your network_
- **Easy and Lightweight**: _AdGuardian can be run either with a super tiny Docker image, or directly with the zero-dependency executable_
- **Good and Safe**: _Written in Rust and unit tested, the app runs locally with no external requests, and (of course) it's fully open source_

#### About AdGuard
[AdGuard Home](https://github.com/AdguardTeam/AdGuardHome) is a free and open source self-hosted (or managed) network-wide ad + tracker blocker. It operates as a DNS server that re-routes tracking domains to a "black hole", thus preventing your devices from connecting to those servers. It makes your internet, faster, safer and gives you a bunch of useful features, like encrypted DNS (DoH, DoT, DNSCrypt), parental controls, blocking of malware / phishing, per-device configs, custom DNS rules, etc.

<details>
<summary><b>Contents</b></summary>

- [About](#about)
- [Getting Started](#getting-started)
  - [Docker](#docker)
  - [Executable](#executable)
  - [Install from Crates.io](#install-from-cratesio)
  - [Build from Source](#build-from-source)
  - [One-Liner](#one-liner)
- [Configuring](#configuring)
   - [With Flags](#with-flags)
   - [With Env Vars](#with-env-vars)
   - [In Docker](#in-docker)
- [Web Mode](#web-mode)
- [Development](#development)
  - [Prerequisites](#prerequisites)
  - [Run](#run)
  - [Technical Docs](#technical-docs)
  - [Testing and Quality](#testing-and-quality)
  - [Building](#building)
- [Credits](#credits)
  - [Contributors](#contributors)
  - [Sponsors](#sponsors)
  - [Dependencies](#dependencies)
- [Mirror](#mirror)
- [Alterntives](#alterntives)
- [Contributing](#contributing)
- [License](#license)
	
</details>

---

## Getting Started

There are several options for running...

### Docker

```bash
docker run -it lissy93/adguardian
```
> You may also pass in your AdGuard info with env vars (using `-e`), see the [Configuring](#configuring) section for an example, and list of availible config params.<br>
> If you experience issues with DockerHub, or would rather use a different registry, the image is also available via GHCR - just replace the image name with: `ghcr.io/lissy93/adguardian`. Alternatively, if you'd like to build it yourself from source, you can do so with `docker buildx build -t  adguardian .` then run it with `docker run -it adguardian`.

### Executable

```bash
curl -o adguardian https://github.com/Lissy93/AdGuardian-Term/releases/latest/download/adguardian-linux && \
chmod +x adguardian && \
./adguardian
```

> In the above example, don't forget to update the URL to download the latest stable version for your operating system<br>
> You may also just head over the the [Releases](https://github.com/Lissy93/AdGuardian-Term/releases) tab, download the latest executable, and double-click on it to run

### Install from Crates.io

```
cargo install adguardian
adguardian
```

> AdGuardian is published as a crate to [crates.io/crates/adguardian](https://crates.io/crates/adguardian). So providing you've got Cargo installed, you can pull the binary directly, and then execute it as above. Again, see the [Configuring](#configuring) section below for how to pass in your AdGuard info.

### Build from Source

```bash
git clone git@github.com:Lissy93/AdGuardian-Term.git && \
cd AdGuardian-Term && \
make
```

> You'll need `git`, `cargo` and `make` (see [here](#development) for installation notes). You can also run the cargo commands defined in the Makefile directly, e.g. `cargo run`

### Scoop

```
scoop install extras/adguardian
```

> For Windows users, AdGuardian is availible via the [Scoop](https://scoop.sh/) package manager, as part of the `extras` bucket - You'll need Scoop installed, then follow [these instructions](https://scoop.sh/#/apps?q=adguardian). This was contributed by [@kzshantonu](https://github.com/kzshantonu) in [ScoopInstaller/Extras#11386](https://github.com/ScoopInstaller/Extras/pull/11386)

### [AUR](https://aur.archlinux.org/packages/adguardian)

```bash
paru -Syu adguardian
# or
yay -Syu adguardian
# or
git clone https://aur.archlinux.org/adguardian.git && cd adguardian && makepkg -si
```

### One-Liner

```bash
bash <(curl -sL https://raw.githubusercontent.com/Lissy93/AdGuardian-Term/main/quick-start.sh)
```

> This will run the [quick-start.sh](https://github.com/Lissy93/AdGuardian-Term/blob/main/quick-start.sh) Bash script, which downloads and executes the latest binary for your system type. Be sure to read and understand the file first

<details>

<summary><h4>Not sure which method to choose?</h4></summary>

- Docker is the easiest but needs to be installed, and adds a bit of overhead (12Mb, to be precise)
- Whereas using the executable won't require any additional dependencies
- If you've got Rust installed, fetching from crates.io will also be both easy and performant
- If your system architecture isn't supported you'll need to build from source, as you also will if you wish to run a fork or make amendments to the code

</details>

---

## Configuring

The app requires the details of an AdGuard instance to connect to.
This info can be provided either as environmental variables, or passed in as flag parameters.
If any of these fields are missing or incomplete, you'll be prompted to enter a value once the app starts.

The following params are accepted:

- `ADGUARD_IP` / `--adguard-ip` - The IP address of your local AdGuard Home instance
- `ADGUARD_PORT` / `--adguard-port` - The port that AdGuard is running on
- `ADGUARD_USERNAME` / `--adguard-username` - An AdGuard Home username
- `ADGUARD_PASSWORD` / `--adguard-password` - An AdGuard Home password

There's also some additional optional environment variables that you may set:

- `ADGUARD_PROTOCOL` - The protocol to use when connecting to AdGuard (defaults to `http`)
- `ADGUARD_UPDATE_INTERVAL` - The rate at which to refresh the UI in seconds (defaults to `2`)

<details>
<summary>Examples</summary>

#### With Flags
	
```bash
adguardian -- \
	--adguard-ip "192.168.180.1" \
	--adguard-port "3000" \
	--adguard-username "admin" \
	--adguard-password "bobs-your-uncle"
```
	
#### With Env Vars
	
```bash
ADGUARD_IP="192.168.180.1" ADGUARD_PORT="3000" ADGUARD_USERNAME="admin" ADGUARD_PASSWORD="bobs-your-uncle" adguardian
```
	
#### In Docker
	
```bash
docker run \
	-e "ADGUARD_IP=192.168.180.1" \
	-e "ADGUARD_PORT=3000" \
	-e "ADGUARD_USERNAME=admin" \
	-e "ADGUARD_PASSWORD=bobs-your-uncle" \
	-it lissy93/adguardian
```
	
</details>

---

## Web Mode

The terminal dashboard can also be viewed via a browser, thanks to [ttyd](https://github.com/tsl0922/ttyd).

AdGuardian is fully compatible with ttyd, so once you've [installed](https://github.com/tsl0922/ttyd#installation) it, you can just precede your run command with ttyd.
E.g. `ttyd docker run -it lissy93/adguardian` or `ttyd adguardian`

This might be useful for embedding into another app or dashboard (like Dashy 😉 - although Dashy already has an [AdGuard widget](https://github.com/Lissy93/dashy/blob/master/docs/widgets.md#adguard-home-block-stats)!) 

<p align="center">
<img width="500" src="https://i.ibb.co/YNYq3xv/adguardian-browser.png">
</p>

Another great option is [gotty](https://github.com/yudai/gotty), which works in a similar way. Note that if you want to allow user input, you'll need to pass the `-w` option.

You can also combine this with a service like [ngrok](https://ngrok.com/) to forward the port, and access the dashboard from anywhere. But be careful to apply the correct access controls!

Another fun idea, could be to display it on a little screen, either atatched or SSH'd into your AdGuard box.

<p align="center">
<img src="https://i.ibb.co/VNL65hZ/20230529-165416.jpg" width="300" />
</p>

---

## Development

### Prerequisites

You'll need Rust installed. Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` - see the [installation docs](https://forge.rust-lang.org/infra/other-installation-methods.html). You'll also need [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git), if you don't already have it.

Then clone the repo, and cd into it, with: `git clone git@github.com:Lissy93/AdGuardian-Term.git` && `cd AdGuardian-Term`

You can view the full list of availible project commands in the [`Makefile`](https://github.com/Lissy93/AdGuardian-Term/blob/main/Makefile)

### Run

To build and run the project for development, run `cargo run`

### Technical Docs

The documentation can be viewed at: [lissy93.github.io/AdGuardian-Term](https://lissy93.github.io/AdGuardian-Term/adguardian)

### Testing and Quality

- `cargo test` - Run unit tests
- `cargo check` - Ensure app is compilable
- `cargo bench` - Execute benchmarks
- `cargo clippy` - Analyse areas for improvement
- `cargo doc` - Generate the documentation

### Building

Once your finished developing, you can build the project for production with: `cargo build --release`
The binaries for your system will then be available in the `./target/release/` directory of the project.
You can execute this directly, e.g. by running `./target/release/adguardian` (add .exe if on Windows)

### CI / CD

The testing, building, and publishing of the app is done with GitHub Actions.
Below is an outline + current status of each workflow.

| Workflow                                           | Status                                                                                                                                                                |
|----------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Build Docker image and push to registry            | [![Build Docker Image 🐳](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/build-docker.yml/badge.svg)](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/build-docker.yml) |
| Compile binaries and upload artifacts to release   | [![Compile Release 🚀](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/release-binaries.yml/badge.svg)](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/release-binaries.yml) |
| Publish compiled app to crates.io                  | [![Publish to Crates.io 📦](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/push-cargo.yml/badge.svg)](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/push-cargo.yml) |
| Generate documentation from Rustdoc, upload to GH pages | [![Generate Rust Docs 📝](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/publish-docs.yml/badge.svg)](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/publish-docs.yml) |
| Sync repo with downstream codeberg mirror          | [![Mirror to Codeberg 🪞](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/mirror.yml/badge.svg)](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/mirror.yml) |
| Insert list of contributors + sponsors into readme | [![Insert Contributors 👥](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/insert-contributors.yml/badge.svg)](https://github.com/Lissy93/AdGuardian-Term/actions/workflows/insert-contributors.yml) |

---

## Credits

### Contributors

<!-- readme: contributors -start -->
<table>
<tr>
    <td align="center">
        <a href="https://github.com/Lissy93">
            <img src="https://avatars.githubusercontent.com/u/1862727?v=4" width="80;" alt="Lissy93"/>
            <br />
            <sub><b>Alicia Sykes</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/liss-bot">
            <img src="https://avatars.githubusercontent.com/u/87835202?v=4" width="80;" alt="liss-bot"/>
            <br />
            <sub><b>Alicia Bot</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/Sir-Photch">
            <img src="https://avatars.githubusercontent.com/u/47949835?v=4" width="80;" alt="Sir-Photch"/>
            <br />
            <sub><b>Christoph</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/tromcho">
            <img src="https://avatars.githubusercontent.com/u/113139586?v=4" width="80;" alt="tromcho"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td></tr>
</table>
<!-- readme: contributors -end -->

### Sponsors

<!-- readme: sponsors -start -->
<table>
<tr>
    <td align="center">
        <a href="https://github.com/koconder">
            <img src="https://avatars.githubusercontent.com/u/25068?u=582657b23622aaa3dfe68bd028a780f272f456fa&v=4" width="80;" alt="koconder"/>
            <br />
            <sub><b>Vincent Koc</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/peng1can">
            <img src="https://avatars.githubusercontent.com/u/225854?v=4" width="80;" alt="peng1can"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/bgadrian">
            <img src="https://avatars.githubusercontent.com/u/830001?u=69f115baad2fcd8c14eb05bdbf5cd80f4649a95a&v=4" width="80;" alt="bgadrian"/>
            <br />
            <sub><b>B.G.Adrian</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/tbjers">
            <img src="https://avatars.githubusercontent.com/u/1117052?u=539d96d5e581b3139c75713ce35b89a36626404c&v=4" width="80;" alt="tbjers"/>
            <br />
            <sub><b>Torgny Bjers</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/emlazzarin">
            <img src="https://avatars.githubusercontent.com/u/1141361?u=714e3487a3f2e0df721b01a0133945f075d3ff68&v=4" width="80;" alt="emlazzarin"/>
            <br />
            <sub><b>Eddy Lazzarin</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/AnandChowdhary">
            <img src="https://avatars.githubusercontent.com/u/2841780?u=747e554b3a7f12eb20b7910e1c87d817844f714f&v=4" width="80;" alt="AnandChowdhary"/>
            <br />
            <sub><b>Anand Chowdhary</b></sub>
        </a>
    </td></tr>
<tr>
    <td align="center">
        <a href="https://github.com/shrippen">
            <img src="https://avatars.githubusercontent.com/u/2873570?v=4" width="80;" alt="shrippen"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/bile0026">
            <img src="https://avatars.githubusercontent.com/u/5022496?u=aec96ad173c0ea9baaba93807efa8a848af6595c&v=4" width="80;" alt="bile0026"/>
            <br />
            <sub><b>Zach Biles</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/UlisesGascon">
            <img src="https://avatars.githubusercontent.com/u/5110813?u=3c41facd8aa26154b9451de237c34b0f78d672a5&v=4" width="80;" alt="UlisesGascon"/>
            <br />
            <sub><b>Ulises Gascón</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/digitalarche">
            <img src="https://avatars.githubusercontent.com/u/6546135?u=d033c9c16e8367987aec3f9ff5922bc67dd1eedf&v=4" width="80;" alt="digitalarche"/>
            <br />
            <sub><b>Digital Archeology</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/InDieTasten">
            <img src="https://avatars.githubusercontent.com/u/7047377?u=8d8f8017628b38bc46dcbf3620e194b01d3fb2d1&v=4" width="80;" alt="InDieTasten"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/bmcgonag">
            <img src="https://avatars.githubusercontent.com/u/7346620?u=2a0f9284f3e12ac1cc15288c254d1ec68a5081e8&v=4" width="80;" alt="bmcgonag"/>
            <br />
            <sub><b>Brian McGonagill</b></sub>
        </a>
    </td></tr>
<tr>
    <td align="center">
        <a href="https://github.com/vlad-timofeev">
            <img src="https://avatars.githubusercontent.com/u/11474041?u=eee43705b54d2ec9f51fc4fcce5ad18dd17c87e4&v=4" width="80;" alt="vlad-timofeev"/>
            <br />
            <sub><b>Vlad Timofeev</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/helixzz">
            <img src="https://avatars.githubusercontent.com/u/12218889?u=d06d0c103dfbdb99450623064f7da3c5a3675fb6&v=4" width="80;" alt="helixzz"/>
            <br />
            <sub><b>HeliXZz</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/mryesiller">
            <img src="https://avatars.githubusercontent.com/u/24632172?u=0d20f2d615158f87cd60a3398d3efb026c32f291&v=4" width="80;" alt="mryesiller"/>
            <br />
            <sub><b>Göksel Yeşiller</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/forwardemail">
            <img src="https://avatars.githubusercontent.com/u/32481436?v=4" width="80;" alt="forwardemail"/>
            <br />
            <sub><b>Forward Email - Open-source & Privacy-focused Email Service (2023)</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/Bastii717">
            <img src="https://avatars.githubusercontent.com/u/53431819?u=604977bed6ad6875ada890d0d3765a4cacc2fa14&v=4" width="80;" alt="Bastii717"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/frankdez93">
            <img src="https://avatars.githubusercontent.com/u/87549420?v=4" width="80;" alt="frankdez93"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td></tr>
<tr>
    <td align="center">
        <a href="https://github.com/ratty222">
            <img src="https://avatars.githubusercontent.com/u/92832598?u=137b65530cbd5f5af9c24cde51baa6cc77cc934b&v=4" width="80;" alt="ratty222"/>
            <br />
            <sub><b>Brent</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/hernanpopper">
            <img src="https://avatars.githubusercontent.com/u/104868017?v=4" width="80;" alt="hernanpopper"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/terminaltrove">
            <img src="https://avatars.githubusercontent.com/u/121595180?v=4" width="80;" alt="terminaltrove"/>
            <br />
            <sub><b>Terminal Trove</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/NixyJuppie">
            <img src="https://avatars.githubusercontent.com/u/138570196?u=b102c222487905728b858704962d32759df29ebe&v=4" width="80;" alt="NixyJuppie"/>
            <br />
            <sub><b>Nixy</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/nrvo">
            <img src="https://avatars.githubusercontent.com/u/151435968?u=e1dcb307fd0efdc45cddbe9490a7b956e4da6835&v=4" width="80;" alt="nrvo"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/mezza93">
            <img src="https://avatars.githubusercontent.com/u/153599966?v=4" width="80;" alt="mezza93"/>
            <br />
            <sub><b>Null</b></sub>
        </a>
    </td></tr>
</table>
<!-- readme: sponsors -end -->

### Dependencies

This project was made possible by the maintainers of the following dependencies
- [anyhow](https://github.com/dtolnay/anyhow) - Error objecr for idiomatic error handling
- [base64](https://github.com/marshallpierce/rust-base64) - Base 64 encoding
- [chrono](https://github.com/chronotope/chrono) - Date + time parsing and manipulating
- [colored](https://github.com/mackwic/colored) - Handling of terminal colors
- [crossterm](https://github.com/crossterm-rs/crossterm) - Term manipulation for kb + mouse events
- [futures](https://github.com/rust-lang/futures-rs) - Extension of futures for async computation
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [serde](https://github.com/serde-rs/serde) - Decerilization of JSON responses
- [tokio](https://github.com/tokio-rs/tokio) - Improved futures
- [tui-rs](https://github.com/tui-rs-revival/ratatui) - Terminal graphing

---

## Mirror

A mirror of this repository is published at: [codeberg.org/alicia/adguardian](https://codeberg.org/alicia/adguardian)

---

## Alterntives

This project was heavily inspired by [PADD](https://github.com/pi-hole/PADD) - terminal status for Pi-Hole users. If you're running [Pi-Hole](https://pi-hole.net/) instead of AdGuard, I highly reccomend you check that out, as it's awesome.

Other developers have built similar AdGuard Home monitoring programs for mobile, including:
- iOS app: [AdGuard Home Remote](https://apps.apple.com/us/app/adguard-home-remote/id1543143740) by RocketScience IT
- Android app: [AdGuard Home Manager](https://play.google.com/store/apps/details?id=com.jgeek00.adguard_home_manager) by [JGeek00](https://github.com/JGeek00)

If you're looking for more AdGuard add-ons, then check [this section](https://github.com/AdguardTeam/AdGuardHome#uses) of their repo.

If you're running [Dashy](https://github.com/Lissy93/dashy/) (a Homelab Dashboard app (which I am the author of)), then there's also 4 [AdGuard Home Widgets](https://github.com/Lissy93/dashy/blob/master/docs/widgets.md#adguard-home-block-stats).

Before I created this, I first built the same product in Go Lang. You can view that [here](https://github.com/Lissy93/OLD_AdGuardian-Term) - it's fully functional, but not as good as the Rust version (There were some valuable lessons that I learnt the hard way about choosing the right tech stack).

---

## Contributing

Contributions of any kind are very welcome (and would be much appreciated!)
For Code of Conduct, see [Contributor Convent](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).
For project setup, see the [Development](#development) section.

#### New here?
To get started, fork the repo, make your changes, add, commit and push the code, then come back here to open a pull request. If you're new to GitHub or open source, [this tutorial](https://www.freecodecamp.org/news/how-to-make-your-first-pull-request-on-github-3#let-s-make-our-first-pull-request-) may help, I've also put some beginner guides together at [git-into-open-source](https://github.com/Lissy93/git-into-open-source) - but feel free to reach out if you need any support.

#### Not a coder?
You can support the project in other ways too, drop us a star, consider sponsoring us on GitHub, share within your network, and report any bugs you come across.

---

## License


> _**[Lissy93/AdGuardian-Term](https://github.com/Lissy93/adguardian-term)** is licensed under [MIT](https://github.com/Lissy93/adguardian-term/blob/HEAD/LICENSE) © [Alicia Sykes](https://aliciasykes.com) 2023._<br>
> <sup align="right">For information, see <a href="https://tldrlegal.com/license/mit-license">TLDR Legal > MIT</a></sup>

<details>
<summary>Expand License</summary>

```
The MIT License (MIT)
Copyright (c) Alicia Sykes <alicia@omg.com> 

Permission is hereby granted, free of charge, to any person obtaining a copy 
of this software and associated documentation files (the "Software"), to deal 
in the Software without restriction, including without limitation the rights 
to use, copy, modify, merge, publish, distribute, sub-license, and/or sell 
copies of the Software, and to permit persons to whom the Software is furnished 
to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included install 
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANT ABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

</details>

<!-- License + Copyright -->
<p  align="center">
  <i>© <a href="https://aliciasykes.com">Alicia Sykes</a> 2023</i><br>
  <i>Licensed under <a href="https://gist.github.com/Lissy93/143d2ee01ccc5c052a17">MIT</a></i><br>
  <a href="https://github.com/lissy93"><img src="https://i.ibb.co/4KtpYxb/octocat-clean-mini.png" /></a><br>
  <sup>Thanks for visiting :)</sup>
</p>

<!-- Dinosaurs are Awesome -->
<!-- 
                        . - ~ ~ ~ - .
      ..     _      .-~               ~-.
     //|     \ `..~                      `.
    || |      }  }              /       \  \
(\   \\ \~^..'                 |         }  \
 \`.-~  o      /       }       |        /    \
 (__          |       /        |       /      `.
  `- - ~ ~ -._|      /_ - ~ ~ ^|      /- _      `.
              |     /          |     /     ~-.     ~- _
              |_____|          |_____|         ~ - . _ _~_-_
-->

