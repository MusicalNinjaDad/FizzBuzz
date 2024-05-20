# Packaging your code

Publishing your core rust code to crates.io is pretty straightforward, ensuring that your python code can be used by people on different systems and with multiple versions of python needs more work.

I'll assume you are happy publishing via github actions _in general_ and may an an excursion with more detail later.

!!! tip "Note:"
    I've used azure devops in the place of PyPi and crates.io to host the final packages (the world _really_ doesn't need another fizzbuzz implementation spamming public pacakge repositories!), publishing to the main package stores is easier.

## Crates.io

!!! rust "Packaging for crates.io"
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
