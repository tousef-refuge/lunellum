```lll view <hash>``` sets the currently viewed commit to the commit
with the given hash. This may add, delete or overwrite the required
files for each commit, so it must be used with great care.

Files that should NOT be modified by ```lll view``` can be put in
the .lllignore file. However, if an older commit has a file that now
exists inside .lllignore, it may cause unpredictable behavior, so be
careful with what you put in .lllignore.

For a more detailed guide on how ```<hash>``` can be written, go to
docs/commands/log.md