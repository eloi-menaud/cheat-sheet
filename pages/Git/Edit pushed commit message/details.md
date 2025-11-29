Edit a specific commit message in your Git history

```shell
printf "\n\nCommit ID : " && read COMMIT_ID; \
SHORT_COMMIT=$(git rev-parse --short "$COMMIT_ID"); \
GIT_SEQUENCE_EDITOR="sed -i.bak 's/^pick $SHORT_COMMIT/reword $SHORT_COMMIT/'" \
git rebase -i $(git rev-parse "$COMMIT_ID"^) && \
git push --force-with-lease
````

# Steps

```shell
printf "\n\nCommit ID : " && read COMMIT_ID;
```

Prompt the user to enter a commit ID.

---

```shell
SHORT_COMMIT=$(git rev-parse --short "$COMMIT_ID"); 
```

Convert the commit ID to its short form.
`git rebase -i` displays instruction lines using short commit hashes.

---

```shell
GIT_SEQUENCE_EDITOR="sed -i.bak 's/^pick $SHORT_COMMIT/reword $SHORT_COMMIT/'"
```

Override the `GIT_SEQUENCE_EDITOR` variable to automatically edit the rebase instruction file.
The selected commit is changed from `pick` to `reword` to indicate that we want to edit its message.

---

```shell
git rebase -i $(git rev-parse "$COMMIT_ID"^)
```

Start an interactive rebase from the parent of the targeted commit.

---

```shell
git push --force-with-lease
```

Force-push the updated history (using the safer `--force-with-lease` option).

