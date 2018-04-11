# Contributing to aardwolf

Contributors are always welcome! Hopefully this guide will provide some pointers :D

## Getting started

High level approach:

1. Find something to fix/improve
2. Change code (Rust code lives in /src/, WebUI lives in /templates/`)
3. Run the app to verify (cargo run --bin aardwolf-server), make changes until it works
4. Open a PR (also can be done between 2. and 3. if you run into problems)

## Contributions

All contributions to Aardwolf or its dependencies should be made in the form of GitHub
pull requests (PR's). Each pull request will be reviewed by a core contributor
(someone with permission to merge PR's) and either merged in the main tree or
given feedback for changes that would be required. All contributions should
follow this format, even those from core contributors.

Should you wish to work on an issue, please claim it first by commenting on
the GitHub issue that you want to work on it. This is to prevent duplicated
efforts from contributors on the same issue.

Head over to [Aardwolf Starter Issues](TBD) to find good tasks to start with. 
If you come across words or jargon that do not make sense, please feel free to ask. 
We will probably be working on a proper glossary at some point after get a proper 
application running. 

## Pull Request Checklist

- Branch from the master branch and, if needed, rebase to the current master
  branch before submitting your pull request. If it doesn't merge cleanly with
  master you may be asked to rebase your changes.

- Commits should be as small as possible, while ensuring that each commit is
  correct independently (i.e., each commit should compile and pass tests). 

- If your patch is not getting reviewed or you need a specific person to review
  it, you can @-reply a reviewer asking for a review in the pull request or a
  comment, or you can ask for a review in [#aardwolf-discussion:matrix.org](https://riot.im/app/#/room/#aardwolf-discussion:matrix.org)
  or the mailing list `aardwolf-development@lists.riseup.net`.

- Add tests relevant to the fixed bug or new feature.  For a DOM change this
  will usually be a web platform test; for layout, a reftest.  See our [testing
  guide](https://github.com/servo/servo/wiki/Testing) for more information.

For specific git instructions, see [GitHub workflow 101](https://github.com/servo/servo/wiki/Github-workflow).
(( Yes this links to a different repo but I like how it reads so :P ))

## Conduct

Please be mindful of our [Code of Conduct](/CODE_OF_CONDUCT.md)

## Communication

Most of the core contributors are on Riot/Matrix room [#aardwolf-discussion:matrix.org](https://riot.im/app/#/room/#aardwolf-discussion:matrix.org)

You can also join the [`aardwolf-development` mailing list](https://lists.riseup.net/www/info/aardwolf-development).
