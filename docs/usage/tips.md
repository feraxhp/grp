
## Tips and Tricks

### Aliases

Every subcommand that matches the names above can be call with one of its respective aliases.
- `create`: has the followin aliases `["+", "cr", "crt", "add"]`
- `delete`: has the followin aliases `["rm", "del", "remove"]`
- `list`: has the followin aliases `["ls"]`
- `clone`: has the followin aliases `["cl"]`
- `pull`: has the followin aliases `["j"]`
- `push`: has the followin aliases `["p"]`

### Completions

_grp_ does want to give as much help as posible... why? 
becouse i hate to type... (an engeenier that hates to type... _ironic_).

To acomplish this _grp_ will cache the results to the list commands...
this is not sync online, it is stored on the device... will it be encripted... 
at least for now... NO!

#### Config completions

##### bash
~~~bash
source <(COMPLETE=bash grp)
~~~

Currently bash has a little problem with the autocompletion... 
so, in order to solve the problem for auto completion of the repostructure 
you may whant to add the completion to a file, and replace the last 5 lines 
for the next: `COMPLETE=bash grp > grp_complete.sh`

~~~bash
if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    COMP_WORDBREAKS=${COMP_WORDBREAKS//:} complete -o nospace -o bashdefault -o nosort -F _clap_complete_grp dgrp
else
    COMP_WORDBREAKS=${COMP_WORDBREAKS//:} complete -o nospace -o bashdefault -F _clap_complete_grp dgrp
fi
~~~

<!--> [!note]-->
> I already open a feature request in [clap-rs](https://github.com/clap-rs/clap/) 
> if you find this a _Headache_ please help me by comenting in this [issue](https://github.com/clap-rs/clap/issues/6280) 

##### zsh
~~~zsh
source <(COMPLETE=zsh grp)
~~~

##### fish
~~~sh
COMPLETE=fish grp | source
~~~
