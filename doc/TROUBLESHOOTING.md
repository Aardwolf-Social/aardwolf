# TROUBLESHOOTING
## Errors that have been seen and resolved.

### Could not compile `aardwolf-templates`
**Error Message:**
'''
error: proc macro panicked  --> aardwolf-templates/src/lib.rs:42:1
   |
42 | compile_i18n!();
   | ^^^^^^^^^^^^^^^^
   |   = help: message: Couldn't update PO file: Os { code: 2, kind: NotFound, message: "No such file or directory" }

error: aborting due to previous error

error: could not compile `aardwolf-templates`.
'''

**Resolution:**
Install `gettext` package for your OS

Debian/Ubunut: `apt install gettext`
