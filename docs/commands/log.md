```lll log``` prints a list of all currently stored commits and their
hashes. For each commit, it stores information in the following format:

``` [hash] <exact date and time of the commit> : <commit message> <tag if available>```

The commit you are currently on is marked by a blue asterisk at the beginning
of the line. The hashes are very important as a lot of other commands
depend on these hashes to know which commit to select.

For commands that ask for a commit's hash, you do not need to give the entire
hash. The first couple of letters is enough to be detected by the app,
as long as no two commits share the same starting letters.

If you don't want to run ```lll log```, you can instead replace
```<hash>``` with ```HEAD~<num>``` where ```<num>``` is a positive or
negative number and it will return the commit that is ```<num>``` commits
ahead (or behind if negative) of the current commit being viewed.
Alternatively, ```HEAD~0``` simply returns the current commit.

Replacing ```<hash>``` with ```LATEST``` will return the latest commit.

Replacing ```<hash>``` with the name of a tag in quotation marks will return
a commit with the given tag. If two commits have the same tag however,
it will choose an arbitrary commit.

This command will not work if there are no commits.