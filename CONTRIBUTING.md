Contributors are always welcome! We are looking for help from technical and non-technical folks. Tech stuff: Rust, Rocket, Bulma, Handlebars, Javascript. See the [wiki](https://github.com/BanjoFox/aardwolf/wiki) for more about the project.

# Aardwolf Contribution Wishlist

## Frontend - non-technical
- A logo!
- Color scheme (colorblind- and low-vision-friendly). **TIP:** [use a contrast checker](https://webaim.org/resources/contrastchecker/)
- Clarify and beautify Github documentation. **TIP:** check out [Github's Markdown Guide](https://guides.github.com/features/mastering-markdown/)

## Frontend - technical
- Templates for web interface (prioritizing accessibility. **TIP:** [use the Web Accessibility Evaluation Tool (WAVE)](http://wave.webaim.org/))
- Find an appropriate [Resource-Oriented Client Architecture (ROCA)](http://roca-style.org/index.html) JavaScript Framework

## Backend
- ActivityPub implementation

## Contribution Tips and Tricks
- Please review our [Code of Conduct](CODE_OF_CONDUCT) before contributing.
- Contributions should be made as Github pull requests. If this sounds daunting, we can help! Don't get discouraged, because we were all beginners once.
- Rust code lives in /src/, WebUI lives in /templates/
- If you see an issue you'd like to work on, leave a comment on the issue in order to prevent duplicate efforts, and so interested folks have a point of contact for collaborating.
- Verify code changes (cargo run --bin aardwolf-server) are functional before submitting, and ask for help when you run into problems.
- All pull requests will be reviewed by at least one [core contributor](https://github.com/BanjoFox/aardwolf/wiki/Contributors) (someone with permission to merge PR's) before merging. Core contributors will also follow this process.
- You may find [the project's references](https://github.com/BanjoFox/aardwolf/wiki/Project-References) and [the project's wiki](https://github.com/BanjoFox/aardwolf/wiki) helpful.
- When in doubt, [ask for help](#communication)!

## Pull Request Checklist

- Branch from the master branch and, if needed, rebase to the current master branch before submitting your pull request. If it doesn't merge cleanly with master you may be asked to rebase your changes.
- Commits should be as small as possible, while ensuring that each commit is correct independently (i.e., each commit should compile and pass tests).
- If your patch is not getting reviewed or you need a specific person to review it, please do one of the following: request a reviewer in the pull request (upper right corner of the PR), @-reply a reviewer in a comment, request a review in [#aardwolf-discussion:matrix.org](https://riot.im/app/#/room/#aardwolf-discussion:matrix.org) or on the mailing list `aardwolf-development@lists.riseup.net` (you do not need to join the mailing list to do this).
- Add relevant tests to the fixed bug or new feature. For a DOM change, this will usually be a web platform test; for layout, a reftest. **TIP:** You may find [Servo's testing guide](https://github.com/servo/servo/wiki/Testing) helpful.
- **TIP:** Check out [@mikeizbicki's Git Cheat Sheet](https://github.com/mikeizbicki/ucr-cs100/blob/2015winter/textbook/cheatsheets/git-cheatsheet.md) and [Gun.io's How to Github](https://www.gun.io/blog/how-to-github-fork-branch-and-pull-request).

## Communication

- Most of the core contributors use Riot/Matrix room [#aardwolf-discussion:matrix.org](https://riot.im/app/#/room/#aardwolf-discussion:matrix.org)
- You can also join the [aardwolf-development mailing list](https://lists.riseup.net/www/info/aardwolf-development).
- @-Mentioning someone here will also receive a prompt response.
