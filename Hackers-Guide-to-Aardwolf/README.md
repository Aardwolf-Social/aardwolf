# Hackers Guide to Aardwolf

### Intro

Welcome to the Hackers Guide to Aardwolf!  Hopefully this document will serve as a reasonable walkthough of the project as a whole, what it needs, and perhaps even some ideas about the future.
The idea here is to have a UNIFIED place to keep track of TODO's so that we can free up the Issues list for actual issues ;)  The numbering scheme below does not *necessarily* define priority although it may also not be that far off.  The numbers also correspond to other documents within this directory that go into much greater detail. Completed items will probably hang around for a while, just as a reference point, and to keep the numbering tidy.

We use Rocket to render the application UI templates are HTML-based, and Bulma CSS framework for styling.  Fork-Awesome is also there to provide cool icons for things.


### The "Tour" (AKA: Where to FIND things)

 - **General Project Documentation** -- Ideally documentation will go into the [wiki](https://github.com/BanjoFox/aardwolf/wiki) there is also the public-facing [homepage](https://aardwolf.social)
 - **Technical Documentation** -- The [INSTALL.md](https://github.com/BanjoFox/aardwolf/blob/master/INSTALL.md) is the resource for getting an Aardwolf instance up, and running.  Supporting documentation can be found in the [/doc](https://github.com/BanjoFox/aardwolf/tree/master/doc) directory.
 - **/config** -- This is where the default, and example configurations hang out
 - **/lang** -- All, TOML language translation files go into here.  These are used to display the interface in different languages.
 - **/migrations** -- DB stuff... 
 - **/src** -- Where all of the rust source files are located.  This is sub-divided into src/bin, and src/aardwolf, whereby bin contains the "backend" application, and Aardwolf has items relating to the front end (app routing).
 - **/Templates** -- This directory houses all of the Tera templates that Rocket uses to render the HTML.  The routing for this is defined in the /src/bin/main.rs, as well as /src/aardwolf/app/routes.rs.
 - **/tests** -- Files related to software testing.
 - **/web** -- Static assets for the application, including: stylesheets, images, ForkAwesome, etc... **hopefully** JavaScript will not be necessary ;)


### The TODO List
Completed tasks will be struckthrough with tildes like ~~this~~.  Or maybe we should just use the checkboxes... HRM dunno yet. 

- [ ] 1. E-mailing auth_tokens (Crates.io has Rust libraries for sending mail.  [Mailstrom](https://crates.io/crates/mailstrom), and [Lettre](https://crates.io/crates/lettre) are two examples
- [ ] 2. Documentation review (correctness/accuracy), organization, creation (app needs more writers).
- [ ] 3. Tying languages to Templates.  The templates will not look right, without text ;).... to be fair, Banjo should probably put filler text in -.-
- [ ] 4. UI Development.  Clean up the existing templates, flesh out ones that haven't been created yet. [Issue #29](https://github.com/BanjoFox/aardwolf/issues/29) tracks most of the progress (unlikely), and there is a separate repo for [UI-hacking](https://github.com/BanjoFox/aardwolf-interface).
- [ ] 5. Data flow diagrams.  How does data move through the application?  This will help to visualize connections between code segments.
- [ ] 6. API Development
- [ ] 7. Implementing more I/O to create a shoutbox-like demo for the live server. 
- [ ] 8. Implement more of the ActivityPub & Mastodon-compatible functionality.
