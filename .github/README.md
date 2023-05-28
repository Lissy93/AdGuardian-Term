<h1 align="center">AdGuardian-Term</h1>
<p align="center">
	<i>Terminal-based, real-time traffic monitoring and statistics for your AdGuard Home instance</i>
</p>
<p align="center">
  <img width="200" src="https://i.ibb.co/25GNT0n/adguardian-banner-4.png" />
</p>

## About

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
curl -o adguardian https://github.com/Lissy93/AdGuardian-Term/releases/download/1.0.0/adguardian-linux && \
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

### One-Liner

<details>

<summary><h4>Not sure which method to choose?</h4></summary>

- Docker is the easiest but adds a bit of overhead
- Where as using the executable won't require any additional dependencies
- If you've got Rust installed, fetching from crates.io will also be both easy and performant
- If you're system architecture isn't supported you'll need to build from source, as you also will if you wish to run a fork or make amendments to the code

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

---

## Development

### Prerequisites

You'll need Rust installed. Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` - see the [installation docs](https://forge.rust-lang.org/infra/other-installation-methods.html). You'll also need [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git), if you don't already have it.

Then clone the repo, and cd into it, with: `git clone git@github.com:Lissy93/AdGuardian-Term.git` && `cd AdGuardian-Term`

You can view the full list of availible project commands in the [Makefile](https://github.com/Lissy93/AdGuardian-Term/blob/main/Makefile)

### Run

To build and run the project for development, run `cargo run`

### Technical Docs

The documentation can be viewed at: 

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

---

## Credits

### Contributors

<!-- readme: contributors -start -->
<!-- readme: contributors -end -->

### Sponsors

<!-- readme: sponsors -start -->
<!-- readme: sponsors -end -->

### Dependencies

---

## Mirror

---

## Contributing

Contributions of any kind are very welcome (and would be much appreciated!)
For Code of Conduct, see [Contributor Convent](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).
For project setup, see the [Development](#development) section.

#### New here?
To get started, fork the repo, make your changes, add, commit and push the code, then come back here to open a pull request. If you're new to GitHub or open source, [this guide](https://www.freecodecamp.org/news/how-to-make-your-first-pull-request-on-github-3#let-s-make-our-first-pull-request-) or the [git docs](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request) may help you get started, but feel free to reach out if you need any support.

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

<!-- Dinosaur -->
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

