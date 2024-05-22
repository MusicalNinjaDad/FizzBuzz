# Packaging your code

Publishing your core rust code to crates.io is pretty straightforward, ensuring that your python code can be used by people on different systems and with multiple versions of python needs more work.

I'll assume you are happy publishing via github actions _in general_ and may an an excursion with more detail later.

!!! tip "Note:"
    I've used azure devops in the place of PyPi and crates.io to host the final packages (the world _really_ doesn't need another fizzbuzz implementation spamming public pacakge repositories!), publishing to the main package stores is easier.

## Crates.io

??? rust "Packaging for crates.io"
    You can follow the [normal process](https://doc.rust-lang.org/cargo/reference/publishing.html) for publishing to crates.io:
    ```sh
    /projectroot$ cargo build --release -p fizzbuzz
    /projectroot$ cargo package -p fizzbuzz
    /projectroot$ cargo publish --package fizzbuzz
    ```

    **Just remember to add `-p fizzbuzz` so you don't accidentally also try to publish your wrapped code!**

??? rust "Publishing from github actions"
    Here is the workflow I used to publish (to ADO):

    **`.github/workflows/deploy-rust.yml`**
    ```yaml
    --8<-- ".github/workflows/deploy-rust.yml"
    ```

## Pypi

The python distribution is significantly more tricky than the rust one in this situation.

!!! python "Distribution formats - pre-built vs build from source"
    Python offers two distribution formats for packages:

    - [Source distributions](https://packaging.python.org/en/latest/specifications/source-distribution-format/) which require the user to have a rust toolchain in place and to compile your code themselves when they install it.
    - [Binary distributions](https://packaging.python.org/en/latest/specifications/binary-distribution-format/) which require you to compile your code for a specific combination of operating system, architecture and python version.

Here's how to successfully provide both and support the widest range of users possible.

!!! warning "Supporting many linux distros"
    Each version of each linux distro has different standard libraries available. To ensure compatibility with as many as possible PyPa defined [`manylinux`](https://github.com/pypa/manylinux) with clear restrictions on build environments. `Cibuildhweel`, detailled below, makes it easy to meet these requirements.

!!! python "Use `cibuildwheel` from PyPa"
    PyPA provide [`cibuildwheel`](https://github.com/pypa/cibuildwheel) to make the process of building wheels for different versions of python and different OS & architectures reasonably simple.

    1. Configure cibuildwheel to install rust and clean any existing build artefacts before beginning builds, via **`./pyproject.toml`**:
      ```toml 
      ...
      [tool.cibuildwheel.linux]
          before-all = "just clean"
          archs = "all"

      [tool.cibuildwheel.macos]
          before-all = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal && cargo install just && just clean"

      [tool.cibuildwheel.windows]
          before-all = "rustup target add aarch64-pc-windows-msvc i586-pc-windows-msvc i686-pc-windows-msvc x86_64-pc-windows-msvc && cargo install just && just clean"
      ...
      ```
    1. Test each wheel after building, you'll also need [`delvewheel`](https://github.com/adang1345/delvewheel) on windows builds (**`./pyproject.toml`**):
      ```toml
      ...
      [tool.cibuildwheel]
          # `test-command` will FAIL if only passed `pytest {package}`
          # as this tries to resolve imports based on source files (where there is no `.so` for rust libraries), not installed wheel
          test-command = "pytest {package}/tests"
          test-extras = "test"
          ...
          [tool.cibuildwheel.windows]
              ...
              before-build = "pip install delvewheel"
              repair-wheel-command = "delvewheel repair -w {dest_dir} {wheel}"
      ...
      ```
    1. Skip linux architecture / library combinations with no rust version available (**`./pyproject.toml`**):
      ```toml
      ...
      [tool.cibuildwheel]
          skip = [
              "*-musllinux_i686",    # No musllinux-i686 rust compiler available.
              "*-musllinux_ppc64le", # no rust target available.
              "*-musllinux_s390x",   # no rust target available.
          ]
          ...
      ```
    1. Set up a build pipeline with a job to run cibuildwheel for you on Windows, Macos & Linux and create a source distribution.
    ??? python "**`.github/workflows/deploy-python.yml`** - full source"
        ```yaml
        --8<-- ".github/workflows/deploy-python.yml"
        ```
    ??? python "**`./pyproject.toml`** - full source"
        ```toml
        --8<-- "./pyproject.toml"
        ```
