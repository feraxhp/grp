# Fast _bug fix_

- `*!` fix forgejo parsing for user / orgs

## Whats new

- `*+` add `issues` subcomand to list and view issues.
- `*+` add `cripto` subcomand to encrip tokens

### Token Encryption

`grp` now implements a encription algoritm, to avoid having 
the tokens in plain text. 

by default it will not do the encription though, you have to tell 
grp that you will encript that especific `pconf`. for simplicity 
all the `pconf`s will be encripted with the same password.

#### Password

to stop asking constanly the password to the user, grp will 
use the keyring of the os to save the password needed to encript 
and decript the tokens.

#### Commands

- encript all the `pconf`s
    ~~~bash
    grp config cripto encript --all
    ~~~
- encript one `pconf`
    ~~~bash
    grp config cripto encript <pconf>
    ~~~
- decript all the `pconf`s
    ~~~bash
    grp config cripto decript --all
    ~~~
- decript one `pconf`
    ~~~bash
    grp config cripto decript <pconf>
    ~~~

### Issues

- `++` support listing and viewing issues:
  - list user issues (all issues by default):
    ~~~bash
    grp issues ls
    ~~~
  - list user issues filtered by assigned or involved:
    ~~~bash
    grp issues ls -a # or --assigned
    grp issues ls -i # or --involved
    ~~~
  - list issues from a specific repository:
    ~~~bash
    grp issues ls repo <repo>
    ~~~
  - view issue details and comments with rich markdown output:
    ~~~bash
    grp issues view <repo>#<number>
    ~~~

## Internal

- `* ` migrate error handling to `mynox` in other to consolidate my projects.
- `++` add unit and integration test suite to `grp-core` covering `org`, `repo`, `user`, and `issue` endpoints across GitHub, GitLab, and Forgejo.

by @feraxhp
---
**Full Changelog**: https://github.com/feraxhp/grp/compare/v0.13.2...v0.15.0
