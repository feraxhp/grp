## Cripto

`grp` now implements a encription algoritm, to avoid having 
the tokens in plain text. 

by default it will not do the encription though, you have to tell 
grp that you will encript that especific `pconf`. for simplicity 
all the `pconf`s will be encripted with the same password.

### Password

to stop asking constanly the password to the user, grp will 
use the keyring of the os to save the password needed to encript 
and decript the tokens.

### Commands

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

> [!tip]
> You also can change the password
> ~~~bash
> grp config cripto change
> ~~~
> this will change it for **all the current pconfs**

### Configuration

#### disable keyring

some people may whant to avoid using the keyring, although it is 
not recomended, you absolutly can, this will cause, though, that 
the password for the encripted tokens, got asked every time you try 
to do someting

> maybe in the future, I will try to implement a gpg like password save.

to disable the keyring you will have to add: 
~~~json
"keyring": false
~~~ 
to the config file

#### password promtp

So much people gets anxious when the prompt of the password 
shows asterisk, or some other kind of featback. So, by default 
grp will show the mask for the platform, if you hate it, just add 
~~~json
"hidepass": true
~~~ 
to the config file

