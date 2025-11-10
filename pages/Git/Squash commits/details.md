> Using **main** as target merge branch, replace all **main** by your target branch

### 1 Reset your branch to the common ancestor with target branch
```sh
git reset --soft $(git merge-base origin/main HEAD)
```

### 2 Create a clean commit (or multiple well-structured ones)
adding everything but you can fine grain
```sh
git add -A
```
commit your changes
```sh
git commit
```

### 3 (Optional) Rebase your work on top of the latest main
Ensures any conflicts are resolved locally, making the merge seamless
```sh
git fetch origin
git rebase origin/main
```

### 4 Force-push your rewritten commit history
```sh
git push --force
```
