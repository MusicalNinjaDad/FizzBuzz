# Combining rust & python

This aims to be a fully worked example showing how to build python libraries in rust. It is written from the perspective of a rust newbie coming from a python background, which also explains why this documentation in presented in a python mkdocs-style rather than as a rust mdbook.

I will walk you through the whole process from an empty repo to having packaged, published code in both pypi and crates.io with documentation and tests across both eco-systems.To keep things simple I've chose a well-known kata _fizzbuzz_ so that the focus is not on the algorithm but the ecosystem and "glue".

!!! abstract "Following this guide"
    You should be able to follow through step by step, although if like me you practice TDD you'll want to jump between the Testing and Developing & Building sections.

    Alternatively you should also be able to dive directly into whichever topic you're looking for guidance on right now, without needing to read everything else first.

I've used github for hosting the repo, CI pipeline and documentation and azure devops in the place of PyPi and crates.io to host the final packages (the world _really_ doesn't need another fizzbuzz implementation spamming public package repositories!). You can view, fork or clone the repo from [github: MusicalNinjaDad/FizzBuzz](https://github.com/MusicalNinjaDad/FizzBuzz) and get the packages from [ADO: MusicalNinjas/FizzBuzz](https://dev.azure.com/MusicalNinjas/FizzBuzz/_artifacts/feed/FizzBuzz).

The whole process took me a few months from start to finish, working on this in my spare time, with a few excursions along the way - there is a separate section on those (above). See the excursions section at the top (once I start writing them) for info ...

!!! abstract "Feedback and contributions"
    I'd love your feedback and contributions via Issues, Discussions and PRs in [github: MusicalNinjaDad/FizzBuzz](https://github.com/MusicalNinjaDad/FizzBuzz)

    Have fun and I hope this is useful!
