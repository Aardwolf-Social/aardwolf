# Making Aardwolf available in your language
> NOTE: This README has been shamelessly hijacked from [Plume](https://github.com/Plume-org/Plume) which is one of the many 
(possibly the first) project that Aardwolf is targeting to work with closely.

*You will need to have basic git and GitHub knownledge to follow this guide. But we plan to setup a more user-friendly translation tool in the future.*

To translate Aardwolf in your language, you'll first need to make sure it is listed in the `po/LINGUAS` file. If it is not, you can ask anybody with a development environment to add it (or do it yourself if you have a development environment). Once it will be here, Aardwolf must be launched once to generate all the needed files.

Then you can start translating. Find the file corresponding to your locale, which is `po/YOUR_LOCALE.po`, and open it. Inside, you have a list of strings to translate. There are two kind of translatable strings.

## Simple strings

They look like this:

```po
msgid "Hello, world"
msgstr ""
```

What is next to `msgid` is the string in English. To translate it, just fill the `msgstr` field with the translation.

## Strings with plural forms

Sometimes, strings may change depending on a number (for instance, a post counter). In the `.po` files, these strings look like this:

```
msgid "One post"
msgid_plural "{{ count }} posts"
msgstr[0] ""
msgstr[1] ""
```

Then you should fill the two `msgstr` field, one with the singular form, the second with the plural one. If your language as more than two forms, you can add another one by following the same pattern (`msgstr[n] ""`).

## Interpolation

Strings you translate may contain data from Aardwolf (a username for instance). To tell Aardwolf where to put these data, surround their identifier by `{{` and `}}`. The identifier is also present in this form in the English string to translate (this what you can see above, with the `{{ count }} posts` message).

## Note

When translating, please try to be as inclusive as possible.
