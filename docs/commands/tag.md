```lll tag <hash> -n <name>``` attaches a tag to the given commit. Upon
attaching, this tag can be used as a bookmark in the commit log. 

When running this command on a commit that already has a tag, the command
will not do anything. To overwrite a tag, the ```-f``` flag must be used.

If ```<hash>``` is not given, it defaults to choosing the current commit.

To delete a tag completely, run ```lll tag <hash> -d```

For a more detailed guide on how ```<hash>``` can be written, go to
docs/commands/log.md