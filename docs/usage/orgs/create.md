> You might visit first [Basic knowledge](/usage/basic)

## Creating organizations

The basic command looks like this
~~~bash
grp orgs create [OPTIONS] <pconf> <name>
~~~

### Flags

- `-r, --recursive`: Create groups recursively for [gitlab]
- `-s, --show-errors`: Show the erros when they happen during recursive orgs creation
- `...`: more to be added.

### Disclaimer

Some platforms do not support sertain operations throw the API.
in this case _github_ does not allows create organizations, and gitlab 
could have disabled the option to create organizations at the root.

> If the platform do not support org creation, the operation will fail.

### Examples

1. Create the test organization.
~~~bash
grp orgs create gitea test
~~~
2. Create the test organization inside myorg (myorg should exist).
~~~bash
grp orgs create gitlab myorg/test
~~~
3. Create the test organization inside myorg recursively (myorg will be created).
~~~bash
grp orgs create -r gitlab myorg/test
~~~
4. Show all the errors if the operation#3 failds.
~~~bash
grp orgs create -sr gitlab myorg/test
~~~
