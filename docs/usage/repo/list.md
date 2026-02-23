> You might visit first [Basic knowledge](/usage/basic)

## Listing repositories

The basic command looks like this
~~~bash
grp list [OPTIONS] [pconf] [owner]
~~~

### Default _pconf_.
as you can see the pconf is optional, that means that you can simply run:
~~~bash
grp list
~~~
and it will return all the repos that belongs to the owner configured in the 
default pconf

> [!caution]
> if no default pconf is configured it will not succed.

you might also whant to use the _default pconf_ with a given owner, so 
you can simply add a point _"."_ to tell the grp parser to use the default pconf.

~~~bash
grp list . clap-rs
~~~

### Flags

- `-s --show-errors`: Show the erros when they happen during the paggination request.
- `...`: more to be added.
