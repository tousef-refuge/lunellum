The .lllignore file has a similar pattern-matching style to .lllinclude,
however it works completely differently.

The .lllignore file tells the app to completely leave specific files alone.
This is needed for specific commands like ```lll view``` that completely
override all files in that directory, so you can put certain files you don't
want to lose (like specific directories, or even the .lllignore itself) to
protect them from getting deleted by mistake when running such destructive
commands.

When initializing a directory, .lllignore always starts by ignoring itself,
to prevent it from being deleted by other commands. You can remove this if
you want however I do not recommend it, as it may cause unexpected behavior
if accidentally deleted.

In most commands, .lllignore takes priority over .lllinclude, so if a file
matches the patterns in both these files, it will be ignored.
