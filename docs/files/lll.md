The .lll/ directory is the heart of every Lunellum repository. It is
created when calling ```lll init``` and stores everything important
about the repository, especially commits.

You can backup this folder by copying it and pasting it to a separate
directory in case you want to do something like ```lll update```.
This prevents you from losing your commit data.

You do not need to add .lll/ to your .lllignore, as this folder is
hard-coded to be skipped by the app. If for some reason you have .lll/
in your .lllinclude, it won't be seen by the app either.

The files in this folder are not meant to be readable by humans, so
don't worry about anything inside it. More importantly, **DO NOT**
modify any files here, as this can corrupt everything. This folder
should simply just be ignored unless you wish to backup your repository.
