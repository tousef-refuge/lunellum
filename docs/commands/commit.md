```lll commit``` is the most important command for this app, and you
will call it basically 99% of the time. It takes a snapshot of all the
current files detected by ```lll status``` and stores them as a commit.
You can look through these commits and see relevant information about
them via other commands. To get a list of all commits, you can call
```lll log``` which shows a full list of each commit and their hashes.

Each commit also comes with a message, which is usually a short description
about what the commit is about. This can be stored by calling 
```lll commit <MESSAGE>```. Alternatively, ```lll commit -m <MESSAGE>```
is also a valid way to run this command for anyone who is too used to
git. ~~its me~~

This command will not work if you are currently viewing an old commit.