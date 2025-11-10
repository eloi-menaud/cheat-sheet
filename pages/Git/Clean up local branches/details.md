```bash
git fetch --prune \ 
&& git for-each-ref --format='%(refname:short) %(upstream:track)' refs/heads/ \
| awk '$2=="gone"{print $1}' \
| xargs -r git branch -D
```

- `git fetch --prune` : updates remote-tracking references and removes those that no longer exist on the remote 
- `git for-each-ref [...] heads/`: lists local branches with their upstream status, returning "gone" if the tracked remote branch has been deleted.
- `awk '$2=="gone"{print $1}'` : selects only local branches whose remote no longer exists.
- `xargs -r git branch -D` : force-deletes these local branches, doing nothing if the list is empty (-r).


## Créer un alias `clean-local-branches` :
```sh
git config --global alias.clean-local-branches "git fetch --prune && git for-each-ref --format='%(refname:short) %(upstream:track)' refs/heads/ | awk '\$2==\"gone\"{print \$1}' | xargs -r git branch -D"
```
use it :
```sh
git clean-local-branches
```