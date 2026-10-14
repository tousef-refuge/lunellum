```lll details <hash>``` prints a list of all the file changes that a
commit has, including new files, deleted files, and changes made to
already existing files. For most files, it gives the state of the file
before the change, and the state of the file after the change.

When showing details, for each file, changes will be color-coded in
the old and new data (red for deleted data, green for added data).
To stop this, you can run the command with the ```--no-color``` flag.

Certain file types will be printed differently in details. To show
the characters EXACTLY as they are, the ```--raw``` flag can be added
to display them. However, this may result in printing a lot of unnecessary
data from large files, so use this flag with caution.

Binary files such as images will not have their actual data displayed
(obviously) so it will only show that changes have been made.

Markdown files (*.md) will be rendered correctly. However, despite
being plain text, they will not display color-coded changes unless
the command is called with the ```--raw``` flag.

For a more detailed guide on how ```<hash>``` can be written, go to 
docs/commands/log.md
