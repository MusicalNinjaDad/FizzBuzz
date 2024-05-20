# Development Environment

To work with both python and rust you're going to need a toolchain in place for both eco-systems and be on a Linux(-like) system. I've done all this on Windows with WSL and it's simply awesome!

I, personally, like to keep my working machine free of the range of different packages and tools that I need for all the different projects I look at. I also like to be able to rebuild a reproducable working environment easily, and let others know which tools they need if they want to contribute.

[Dev Containers](https://containers.dev/) make this easy, they consist of:

1. a Dockerfile with the entire set of tools needed for development
1. a json file with IDE settings, extensions and project specific build scripts

Think of it like this: you wouldn't develop a python project without a virtual environment ...

## Quick-Start

!!! abstract "Getting up and running fast"
    1. Install docker (any set up will do but docker desktop works easiest in my experience if you're starting from scratch)
    1. Install the `ms-vscode-remote.remote-containers` extension in VSCode
    1. Copy `.devcontainer/devcontainer.json` from [MusicalNinjaDad/FizzBuzz](https://github.com/MusicalNinjaDad/FizzBuzz) into your project (keeping the location and filename)
    1. Select `Reopen in container`from the toastie or the command bar

## Packages and tools (Dockerfile)

!!! abstract "Pre-Build Docker Image"
    You can grab a pre-build image for either linux/amd64 or linux/arm64 via `docker pull ghcr.io/musicalninjas/pyo3-devcontainer:latest`. The source is at [MusicalNinjas/devcontainers-pyo3](https://github.com/MusicalNinjas/devcontainers-pyo3)

    I like to use fedora for my dev environment base as it provides the most up-to-date versions of tools via the package manager dnf, I also like to keep the environment as clean as possible, with only the tools that the project needs. You'll find similiarly named packages in most distros.

??? abstract "The full Dockerfile"
    ```Docker
    --8<-- "https://github.com/MusicalNinjas/devcontainers-pyo3/raw/main/Dockerfile"
    ```

    If you are putting together an environment for yourself the important things to note are the packages in the `# Python` and `# Rust (and python headers)` sections. As well as the usual tools for developing in each language you will also need the python headers for pyo3 to use - in the case of fedora that's `python3-devel`.

!!! python
    Most distributions need you to install the python package manager `pip` with a dedicated package. Python is so embedded into the OS that you really don't wan't to go installing a load of packages and a different version of python at the system level.

    There is no point installing a range of other python packages as you will be re-downloading these in a virtual environment anyway to keep your project set up and your system setup separate.

    ```sh
    dnf -y install python-pip
    ```

!!! rust
    Most distributions don't bundle a recent version of rust-up, so you need to install rust by `curl`-ing a shell-script as described at [rustup.rs](https://rustup.rs). Fedora is nice.

    To get rust working you'll also need `clang` as a C-compiler and at least `rust-src` so you have the sources of the core libraries

    ```sh
    dnf -y install \
        clang \
        python3-devel \
        rustup \
    && rustup-init -v -y \
    && rustup component add rust-src
    ```

    If you're installing rust inside a docker container which will not be run as `root` you need to use `RUSTUP_HOME` and `CARGO_HOME` environment variables to select where to install, make sure they are accessible to all users, add cargo to the `PATH` and then `chown` or `chmod` the entire contents to make them usable by a non-root user. Otherwise the installation goes under `/root` and the files are `rw-rw---- root root`

    Unlike python installing tools up-front via cargo can save quite some time when rebuilding / creating the dev environment as rust tools are compiled from source when you install them. Getting this done once when you build the container rather than every time you create an environment saves a significant number of minutes.

!!! abstract "Other useful tools"
    In addition to the core language components I also found the following tools useful (they're in the `Dockerfile`):

    - [`just`](https://github.com/casey/just): You're going to be running a few multi-step commands. I came across it for the first time during this project and liked the simple approach. I guess you could use `nox`, but that means mentally translating from shell to python and context swtiching from rust to python if you're in the middle of rust-ing. `make` would be another option, but again, overly complicated for this use case, in my view
    - `llvm-tools-preview` and [`grcov`](https://github.com/mozilla/grcov) for getting test coverage of your rust code. There are a few different ways to get rust coverage, after much research and a few unsuccessful starts I found grcov, from mozilla, which pretty much just worked, gave the best quality results and sounds to me like it works in a way that makes most sense
    - [`mdbook`](https://github.com/rust-lang/mdBook) is what to use if you want to create rust-style manuals rather than python-style manuals
    - [`cargo-expand`](https://github.com/dtolnay/cargo-expand) is invaluable if you end up writing rust proc-macros

## IDE extensions and settings (json)

I use VSCode and love language-agnostic approach it takes. Devcontainers will run in IntelliJ as well (I've tried it with a colleague) and in neoVIM (I've read), many of these tips will probably translate to those IDEs too without too much google-ing.

??? abstract "The full devcontainer.json"
    ```json
    --8<-- ".devcontainer/devcontainer.json"
    ```

!!! python
    Extensions:

    - `ms-python.python` & `ms-python.vscode-pylance` give you language services, hints and intellisense
    - `charliermarsh.ruff` integrates `ruff` as your linter (It's written in rust!)
    
    Settings:

    - `"python.defaultInterpreterPath": "./.venv/bin/python3"` sets VSCode to use the interpreter your virtual environment
    - `"python.testing.pytestEnabled": true` sets VSCode to use [`pytest`](https://https://pytest.org/) and integrates the tests into the Test Explorer

!!! rust
    Extensions:

    - `rust-lang.rust-analyzer` the only extension you need is the official LSP implementation for rust. This handles everything including testing

    Settings:

    - `"rust-analyzer.interpret.tests": true` & `"rust-analyzer.testExplorer": true` enable integrating tests into VSCode
    - `"[rust]": {"editor.rulers": [100]}`: rust has a _standard_ formatter which is as opinionated as `black`! You're going to want a ruler at 100 chars!
