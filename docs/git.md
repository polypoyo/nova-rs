# Git Formalities

In order to keep some semblance of an organized git history, there are
a few rules that you must follow to keep things proper.

## Commit Messages

Commit messages must be in the imperative mood and follow all guidelines
put forward in [Chris Beams' Guide](https://chris.beams.io/posts/git-commit/).
This is good reading to get the intent of having proper git history.

Additionally, we "namespace" our commits. Each commit should only concern
one particular module. This module should be one of the following. If
a change could fall under multiple, it should be split up to keep commits
as small in scope as possible.

* `[all]` = affects many different modules/crates
* `[cargo]` = changes to the build
* `[ci]` = CI on all platforms.
  * `[ci-nix]` = CI on Mac and Linux.
    * `[ci-osx]` = CI on OSX.
    * `[ci-linux]` = CI on Linux.
  * `[ci-win]` = CI on Windows.
* `[docs]` = documentation changes.
* `[ide]` = changes to accommodate a particular ide
* `[git]` = git specific changes (.gitignore etc).
* `[*]` = (fill in * with the name) nova module `nova-*`. (ex. `nova-shaderpack` would be `[shaderpack]`) 
* `[tools]` = changes to assorted tooling

## Merge Style

You should clean up your branch before merge and rebase away all issues
with the history. This history will be merged with a **merge commit**, so
having a sane history is important.
