> You might visit first [Basic knowledge](/usage/basic)

## Cloning repositories

The basic command looks like this
~~~bash
grp clone [OPTIONS] <repo> [path]
~~~

### Remote config 
Git used to create the remote with the name _origin_ but 
grp creates the remote with the name of the 
__pconf__ used to clone the repo.

> This behaviour was meant to reduce the number of arguments to do a push for the repo. 
> this way you can have a diffrent remote url than the default pconf and still avoid to 
> splicitly call the working pconf.

### Flags

- `-u, --url <pconf> <url>`: Clones a url instead of a repo_structure
- `-b, --branch [<name>]`: The name of the branch
- `-B, --bare`: Clone as bare repo
- `...`: more to be added.

### Examples

1. Clone the this repository.
~~~bash
grp clone github:feraxhp/grp
~~~
2. Clone the this repository with url.
~~~bash
grp clone -u github https://github.com/feraxhp/grp
~~~
