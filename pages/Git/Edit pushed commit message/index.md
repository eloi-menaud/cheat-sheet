Edit a spcific commit message in the history

## Interactive Command
command to copy/paste, will ask you the commit id during execution (use default git editor to edit the message)
```shell
printf "\n\nCommit ID : " && read COMMIT_ID; \
SHORT_COMMIT=$(git rev-parse --short "$COMMIT_ID"); \
GIT_SEQUENCE_EDITOR="sed -i.bak 's/^pick $SHORT_COMMIT/reword $SHORT_COMMIT/'" \
git rebase -i $(git rev-parse "$COMMIT_ID"^) && \
git push --force-with-lease
```

## Shell function
Shell function that takes `<commit id>` as parameter (use default git editor to edit the message).
```shell
git-edit-commit-message(){
    COMMIT=$1 
    test -z "$COMMIT" &&
        { echo "Missing <commit id> arg : $0 <commit id> <message>"; exit 1; } 
    SHORT_COMMIT=$(git rev-parse --short $COMMIT)
    
    GIT_SEQUENCE_EDITOR="sed -i.bak 's/^pick $SHORT_COMMIT/reword $SHORT_COMMIT/'" \
    git rebase -i $(git rev-parse $1^)
    
    git push --force-with-lease
}
```