The .lllinclude file is a file generated alongside .lllignore whenever
```lll init``` is called. The way they track files is similar to 
[.gitignore](https://git-scm.com/docs/gitignore#_pattern_format),
but how they actually function is pretty different.

The .lllinclude file tells the app to track specific files for things like
commiting and viewing details. Therefore, unlike traditional version control
systems that need you to manually add files for staging, Lunellum allows you
to skip that step and automatically adds any files for you, as long as they
have a pattern available in .lllinclude

When initializing a repository, they always start by containing ```*.txt```, 
meaning the repository currently only tracks .txt files, and will ignore 
everything else in MOST operations. The operations that don't get covered by
this file can be covered via .lllignore
