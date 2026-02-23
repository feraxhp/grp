> You might visit first [Basic knowledge](/usage/basic)

## Deleting repositories

The basic command looks like this
~~~bash
grp delete [OPTIONS] <repo>
~~~

### RepoStructure: `<repo>`

> see: [repostructure](/usage/basic?id=repostructure) for more information 
> in this case **the pconf is mandatory** for security.

### Flags

- `-y, --yes`: Skip the confirmation prompt
- `-s, --soft`: just mark the repo for delition [gitlab]

### Examples

1. Delete the grp2 repository interactively.
~~~bash
grp delete gh:feraxhp/grp2
~~~
2. Delete the grp2 repository non-interactively.
~~~bash
grp delete -y gh:feraxhp/grp2
~~~
3. Delete the grp2 repository in gitlab non-interactively.
~~~bash
grp delete -y gl:feraxhp/testing/grp2
~~~
