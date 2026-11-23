## Changelog

All notable changes will be documented on this file.

### v1.0.0 (October 7th 2026)

**Added:**

- This changelog
- The 8 basic commands (commit, details, init, log, reset, status, view, update)
- .lllignore and .lllinclude implementation

### v1.1.0 (October 7th 2026)

**Added:**

- Two new commands (tag, reword)
- Tags for easy commit access and bookmarking
- Two new flags for ```lll details``` (--raw and --no-color)

**Changed:**

- When showing commit details, additionally shows where each change happens in a
  file via color-coding
- Markdown files are printed with the correct format
- A link to the repository's docs folder is attached to the help page
- ```lll commit``` does not require the -m flag to run
- .lllinclude files additionally include markdown files upon initialization

### v1.1.1 (November 23rd 2026)

**Changed:**

- ```lll log``` no longer throws an error when called on an empty repository
- The directory which stores tags in the .lll/ is now properly initialized
- The .lll continuously regenerates its directories if it detects one that
  is missing (though currently, it may still run into errors when processing
  corrupted .lll/ folders, such as tags that don't point to any commit)
- Removed a repeated line in the output of ```lll update```
