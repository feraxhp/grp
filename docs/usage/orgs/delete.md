> You might visit first [Basic knowledge](/usage/basic)

## Deleting organizations

The basic command looks like this
~~~bash
grp orgs delete [OPTIONS] <pconf> <name>
~~~

### Flags

- `-y, --yes`: Skip the confirmation prompt
- `-s, --soft`: just mark the org for delition [gitlab]

### Examples

1. Delete the test organization interactively.
~~~bash
grp delete gitlab test
~~~
2. Delete the test organization non-interactively.
~~~bash
grp delete -y gitlab test
~~~
3. Mark the test organization for deletion on gitlab.
~~~bash
grp delete -sy gitlab test
~~~
