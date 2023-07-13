# TROUBLESHOOTING
## Errors that have been seen and resolved.

### Could not compile `aardwolf-templates`
**Error Message:**
```
error: proc macro panicked  --> aardwolf-templates/src/lib.rs:42:1
   |
42 | compile_i18n!();
   | ^^^^^^^^^^^^^^^^
   |   = help: message: Couldn't update PO file: Os { code: 2, kind: NotFound, message: "No such file or directory" }

error: aborting due to previous error

error: could not compile \`aardwolf-templates\`.
```

**Resolution:**
Install `gettext` package for your OS

Debian/Ubunutu: `apt install gettext`


**Error Message:**
```
error: linking with `cc` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="... 
  
  ## HUGE WALL OF TEXT ##

  = note: ld: library not found for -lpq
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
          
error: could not compile `aardwolf` (bin "aardwolf-server" test) due to previous error

```

**Resolution:**
The key here after the wall of text --> `note: ld: library not found for -lpq`. The `lpq` package is normally installed as part of PostgreSQL (Lib PQ)

Debian/Ubunutu: `apt install libpq`
Mac OS (Homebrew):
- Option 1: Install complete Postgres: `brew install postgres`
- Option 2: Install the libpq library only, and use `RUSTFLAGS="-L /usr/local/opt/libpq/lib" cargo [command] [package-name]`