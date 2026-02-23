> You might visit first [Basic knowledge](/usage/basic)

## Listing organizations

The basic command looks like this
~~~bash
grp orgs list [OPTIONS] [pconf]
~~~

### Default _pconf_.
as you can see the pconf is optional, that means that you can simply run:
~~~bash
grp orgs list
~~~
and it will return all the orgs that belongs to the owner configured in the 
default pconf.

> if you do not have access it will fail

### Flags

- `-s --show-errors`: Show the erros when they happen during the paggination request.
- `...`: more to be added.
