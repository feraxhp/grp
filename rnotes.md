## Whats new

### Create Pull Requests

You can know create pr's directly with grp, 
to be able to use the 100% of the `grp pr create` 
you should have your repos configured to work by default 
with grp.

various new flows are added.
~~~bash
# 1. create a pr from the current branch to the default branch on the remote
grp pr create
# 2. force the selected remote (asuming gj is for gitlab)
grp pr create gj
# 3. create a pr on a foreing repo from your current fork
grp pr create gh:zed-industries/zed?main
# 4. create a pr on a foreing repo from your remote fork
grp pr create gh:zed-industries/zed?main --origin feraxhp/zed?feat/render-colrv1-emojis
# 5. create a pr passing the title directly
grp pr create - "This will autodetect the target and origin branches" 
~~~

> [!tip]
> Note that in the **example `#4`** you do not need to have the repo cloned
> `grp` will trust that the information is correct fron your side.

> [!important]
> Currently the autodetect feature will not work with `ssh@hots:path/[...]` remotes, only works with
> url based remotes

> [!tip]
> Note that in the **example `#5`** the `-` is in place of the target parameter

> [!caution]
> This was tested on github y forgejo, more than on gitlab, if you got an issue with **gitlab** 
> please report it as soon as posible.


by @feraxhp

---
**Full Changelog**: https://github.com/feraxhp/grp/compare/v0.15.1...v0.16.0
