# Structuring the codebase

## Basic structure

Keep your rust, wrapped-rust and python code separate:

```text
FizzBuzz
- rust
  - fizzbuzz
    - src
      ... core rust code goes here
    - tests
      ... tests for core rust code go here
  - fizzbuzzo3
    - src
      ... pyo3 code goes here
- python
  - fizzbuzz
    ... additional python code goes here
- tests
  ... python tests go here
```

## Considerations

1. When you distribute the final python library as source the primary audience are python-coders, make it understandable for them without a lot of extra explanation by putting your rust code in a clearly marked location
1. You will want to test your code at each stage of integration: core rust, wrapped rust, final python result; so that you can easily track down bugs
1. Context switching between two languages is hard, even if you know both parts of the codebase well. Keeping it structured by language boundary helps when coding. For much larger projects you may want to provide a higher-level structure by domain and _then_ structure each domain as above ... but that's outside the scope of a simple starter how-to :wink:
1. Your underlying code probably does something useful - so you could also publish it to the rust eco-system as a dedicated crate for others to use directly in rust. Those users don't want the extra code wrapping your function for python!

!!! warning
    Having any top-level directories with names that match your package leads to all kinds of fun import errors. Depending on the exact context python can decide that you have implicit namespace packages which collide with your explicit package names.

    I ran into problems twice:

    - firstly, I had a time where my tests ran fine but I couldn't import my code in repl;
    - later, the final wheels were unusable but sdist was fine.  
    
    This is also the reason for keeping your final level of python tests in a separate top-level directory: you can be sure they are using the right import logic.
