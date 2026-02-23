> You might visit first [Basic knowledge](/usage/basic)

## Creating repositories

The basic command looks like this
~~~bash
grp create [OPTIONS] <repo> [description]
~~~

### RepoStructure: `<repo>`

> see: [repostructure](/usage/basic?id=repostructure) for more information 
> in this case **the pconf is optional**.

### Flags

- `-p, --public`: Make the repository public
- `-a, --add-to-local`: Add the remote to the current repository
- `-r, --remote[=<path>]`: Add the remote to a local repository
- `...`: more to be added.

### Examples

1. Create the grp2 repository _private_.
~~~bash
grp create gh:feraxhp/grp2
~~~
2. Create the grp2 repository _public_.
~~~bash
grp create -p gh:feraxhp/grp2
~~~
3. Create the grp2 repository and add a remote to the current repository.
~~~bash
grp create -a gh:feraxhp/grp2
~~~
4. Create the grp2 repository with a description.
~~~bash
grp create gh:feraxhp/grp2 "an example repo to show up the functionality"
~~~
