> Using **main** as target merge branch, replace all **main** by your target branch

```sh
git reset --soft $(git merge-base origin/main HEAD)
git add -A
git commit
git fetch origin
git rebase origin/main
git push --force
```
