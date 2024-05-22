# Packaging your code

Publishing your core rust code to crates.io is pretty straightforward, ensuring that your python code can be used by people on different systems and with multiple versions of python needs more work.

I'll assume you are happy publishing via github actions _in general_ and may add an excursion with more detail later.

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

??? rust "TODO - publishing to ADO"
    **`.cargo/config.toml`**
    ```yaml
    --8<-- ".cargo/config.toml"
    ```

## Pypi

The python distribution is significantly more tricky than the rust one in this situation.

!!! python "Distribution formats - pre-built vs build from source"
    Python offers two distribution formats for packages:

    - [Source distributions](https://packaging.python.org/en/latest/specifications/source-distribution-format/) which require the user to have a rust toolchain in place and to compile your code themselves when they install it.
    - [Binary distributions](https://packaging.python.org/en/latest/specifications/binary-distribution-format/) which require you to compile your code for a specific combination of operating system, architecture and python version.

Here's how to successfully provide both and support the widest range of users possible:

!!! warning "Supporting many linux distros"
    Each version of each linux distro has different standard libraries available. To ensure compatibility with as many as possible PyPa defined [`manylinux`](https://github.com/pypa/manylinux) with clear restrictions on build environments. `cibuildhweel` (see below)makes it easy to meet these requirements.

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
    1. Set up a build pipeline with a job to run cibuildwheel for you on Windows, Macos & Linux and to create a source distribution. Run the various linux builds using a matrix strategy to reduce your build time significantly.
    
    ??? python "**`.github/workflows/deploy-python.yml`** - full source"
        ```yaml
        --8<-- ".github/workflows/deploy-python.yml"
        ```
    
    ??? python "**`./pyproject.toml`** - full source"
        ```toml
        --8<-- "./pyproject.toml"
        ```

!!! rocket "Saving hours by using pre-built images"
    If you are wondering why there is no "install rust" command for the linux builds; I created dedicated container images for of pypa's base images with rust, just etc. pre-installed. This saves a load of time during the build, as installing rust and compiling the various components can be very slow under emulation.

    The dockerfile and build process are at [MusicalNinjas/cibuildwheel-rust](https://github.com/MusicalNinjas/cibuildwheel-rust), They get bumped by dependabot every time pypa release a new image for `manylinux2014_x86_64` so should be nice and up to date.
    
    The images can be used by including the following in your **`./pyproject.toml`**:
    ```toml
    [tool.cibuildwheel]
        ...
        # Use prebuilt copies of pypa images with rust and just ready compiled (saves up to 25 mins on emulated archs)
        manylinux-x86_64-image = "ghcr.io/musicalninjas/quay.io/pypa/manylinux2014_x86_64-rust"
        manylinux-i686-image = "ghcr.io/musicalninjas/quay.io/pypa/manylinux2014_i686-rust"
        manylinux-aarch64-image = "ghcr.io/musicalninjas/quay.io/pypa/manylinux2014_aarch64-rust"
        manylinux-ppc64le-image = "ghcr.io/musicalninjas/quay.io/pypa/manylinux2014_ppc64le-rust"
        manylinux-s390x-image = "ghcr.io/musicalninjas/quay.io/pypa/manylinux2014_s390x-rust"
        manylinux-pypy_x86_64-image = "ghcr.io/musicalninjas/quay.io/pypa/manylinux2014_x86_64-rust"
        manylinux-pypy_i686-image = "ghcr.io/musicalninjas/quay.io/pypa/manylinux2014_i686-rust"
        manylinux-pypy_aarch64-image = "ghcr.io/musicalninjas/quay.io/pypa/manylinux2014_aarch64-rust"
        musllinux-x86_64-image = "ghcr.io/musicalninjas/quay.io/pypa/musllinux_1_2_x86_64-rust"
        musllinux-aarch64-image = "ghcr.io/musicalninjas/quay.io/pypa/musllinux_1_2_aarch64-rust"
        ...
    ```

    ??? abstract "**`MusicalNinjas/cibuildwheel-rust/Dockerfile`** - full source"
        ```Docker
        --8<-- "https://raw.githubusercontent.com/MusicalNinjas/cibuildwheel-rust/main/Dockerfile"
        ```
    
    ??? abstract "**`MusicalNinjas/cibuildwheel-rust/.github/workflows/publish_docker.yml`** - full source"
        ```yaml
        --8<-- "https://raw.githubusercontent.com/MusicalNinjas/cibuildwheel-rust/main/.github/workflows/publish_docker.yml"
        ```

!!! warning "avoiding a 6 hour build"
    At one point I had a build run for nearly 6 hours before I cancelled it. The problem was that I use `ruff` for linting, and ruff doesn't provide pre-built wheels for some manylinux architectures. That meant that I was building ruff _from scratch_, _under emulation_ for _each wheel_ I built on these architectures!

    Separating the `[test]`, `[lint]`, `[doc]` and `[cov]`erage dependencies solved this; after a lot of searching I also found the way to provide a single `[dev]` set of dependencies combing them all.
    
    In **`./pyproject.toml`**:
    ```toml
    ...
    [tool.cibuildwheel]
        ...
        test-extras = "test"
    ...
    [project.optional-dependencies]
        lint = ["ruff"]
        test = ["pytest", "pytest-doctest-mkdocstrings"]
        cov = ["fizzbuzz[test]", "pytest-cov"]
        doc = ["black", "mkdocs", "mkdocstrings[python]", "mkdocs-material"]
        dev = ["fizzbuzz[lint,test,cov,doc]"]
    ...
    ```

!!! warning "failing wheel tests due to namespace collisions"
    I lost over half a day at one point trying to track down the reason that my wheel builds were failing final testing. There were two problems:

    1. Make sure you clean any pre-compiled `.so` files before building a wheel, otherwise `pip wheel` may decide that you already have a valid `.so` and use that, with the wrong versions of the C-libraries. This is only a problem when doing local wheel builds for debugging, because you're not checking the `.so` files into git are you? (I created a `just clean` command for this)
    1. Make sure you directly specify the test path for wheel tests in **`./pyproject.toml`**:
      ```toml
      ...
      [tool.cibuildwheel]
          # `test-command` will FAIL if only passed `pytest {package}`
          # as this tries to resolve imports based on source files (where there is no `.so` for rust libraries), not installed wheel
          test-command = "pytest {package}/tests"
      ...
      ```
      otherwise the directive `tool.pytest.ini_options.testpaths = ["tests", "python"]` will cause pytest to look in `python`, find your namespace, and ignore the installed wheel!

    ??? python "**`./pyproject.toml`** - full source"
        ```toml
        --8<-- "./pyproject.toml"
        ```
    ??? python "**`./justfile`** - full source"
        ```justfile
        --8<-- "./justfile"
        ```
