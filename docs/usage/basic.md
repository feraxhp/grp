## Basic knowledge

### What is a _Pconf_ ?

A __pconf__ _(platform configuration)_ is a structre defined in the [configuration file](/guides/config) 
where you add all the basic information needed for GRP to conect.
You will find that _pconf_ is the most important parameter in grp, because it allows the app to know 
where to send the request and which token to use. 

> When the **pconf** is needed in the cli parameter it refers only to the name of it.

#### Example
~~~json
{
    "name": "gh",
    "owner": "feraxhp",
    "token": "<token generated>",
    "type": "github",
    "endpoint": "api.github.com"
}
~~~

### The owner

The _owner_ is an organization or a user that owns a repository. If you whant to, for example, 
clone grp, the owner is goin to be __feraxhp__.

### RepoStructure

The _repo structure_ is a simple syntaxis designed to identify a repository. 

It looks like:
~~~RepoStructure
<pconf>:<owner(/\w)*>/<repo>
~~~

- pconf: name of the _pconf_
- owner: organization or user that owns the repo, for gitlab it could be nested so you have to provide the full path
- repo: the name of the repo

> If the platform does not support nested owners, it will fail parsing the structure.

#### Examples

> ##### assuming:
> - `github`:  is a pconf for **github**
> - `gh`:  is a pconf for **github**
> - `gl`:  is a pconf for **gitlab**

1. `github:feraxhp/grp`: yes! its this project on github.
2. `gh:feraxhp/grp`: yes! its this project on github.
3. `gl:gitlab-org/charts/gitlab`: its [this repo](https://gitlab.com/gitlab-org/charts/gitlab). here the owner is gitlab-org/charts, and the name of the repo is gitlab.

Ass you can see, the pconf resolves to the **endpoint configured** on the specific pconf.

## Command conventions

1. A parameter surrounded square brakets `[]` means it is _optional_.
2. A parameter surrounded with bigger than less than `<>` means it is _mandatory_.
    - You might found 2 repo structures as
        * `[pconf]:<owner(/\w)*>/<repo>`: where pconf is optional
        * `<pconf>:<owner(/\w)*>/<repo>`: where pconf is mandatory
