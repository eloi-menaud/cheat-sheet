Delete all local branches whose remote tracking branch no longer exists
```sh
git fetch --prune && git for-each-ref --format='%(refname:short) %(upstream:track)' refs/heads/ | awk '$2=="gone"{print $1}' | xargs -r git branch -D
```