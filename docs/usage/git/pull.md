> You might visit first [Basic knowledge](/usage/basic)

## Pulling repository changes

The basic command looks like this
~~~bash
grp pull [OPTIONS] [pconf] [remote] [branch]
~~~

grp tries to imitate as much as posible the behavior of `git pull` 
if you need some other functionality, fell free to open an issue.

As you can see, the _pconf_ is optional, this means that if you do not provide 
a pconf. grp will try to get the pconf that is currently 
set on the given branch.

> This behaviour was meant to reduce the number of arguments to do a push for the repo. 
> this way you can have a diffrent remote url than the default pconf and still avoid to 
> splicitly call `grp pull other`

### Flags

- `-f, --force`: Do a force push
- `-r, --rebase`: Do a pull --rebase
- `-n, --dry-run`: Do everything except actually send the updates.
- `-u, --set-upstream <remote> <branch> `:  the name of the remote as default upstream for a branch
- `-p, --path <path>`: Path to the repository
- `...`: more to be added.

### Examples

#### Pull the changes with the pconf named "gh".

Imagine youy reponse of `git remote -v` to be:
~~~bash
● git remote -v
gh	https://github.com/feraxhp/grp.git (fetch)
gh	https://github.com/feraxhp/grp.git (push)
~~~

you have to run:
~~~bash
grp pull
~~~

> this is the best way posible to work with `push`, 
> and its important that you try to use this as much as posible.

#### Push the changes with the pconf named "gh".

Imagine youy reponse of `git remote -v` to be:
~~~bash
● git remote -v
origin	https://github.com/feraxhp/grp.git (fetch)
origin	https://github.com/feraxhp/grp.git (push)
~~~

you have to run:
~~~bash
grp pull gh
~~~

> this way `"gh"` will override the search for the pconf named 
> as the remote.

#### set-upstream

You often will need to set the upstream while pushing and you 
can doit by runing:

Imagine youy reponse of `git remote -v` to be:
~~~bash
● git remote -v
gitlab	https://github.com/feraxhp/grp.git (fetch)
gitlab	https://github.com/feraxhp/grp.git (push)
~~~

> yes, you can name the pconf however you want, in dis case, the pconf named gitlab 
> is a github configuration

you have to run:
~~~bash
grp pull -u gitlab main
# -----------------^^^^ -> this is the branch that you whant to push
~~~

and yes you can override the pconf by runing:
~~~bash
grp pull override -u gitlab main
# --------------------------^^^^ -> this is the branch that you whant to push
# -------^^^^^^^^--------------- -> the pconf is called "override"
~~~
