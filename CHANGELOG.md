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
