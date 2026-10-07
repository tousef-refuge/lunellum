<!--suppress CheckImageSize -->
<div style="text-align: center;">
    <img src="./logo.png" alt=" " width="256"><br/>
</div>

## Overview

Lunellum is a [version control system](https://en.wikipedia.org/wiki/Version_control) 
built specifically for writers. It's built to be as easy as possible to use even for
people with little to no programming experience. It stores changes made to your story
in files of your choice, and allows for easy viewing access between changes. Besides
writing you can use this app for taking notes or even as a side-grade for Git.

You can learn how to use this app in detail by reading the files in docs/, but do
keep in mind new versions may break old .lll files, so always make a backup when
possible. Also, some features may or may not work properly so if you encounter any 
problems, a pull request would help a lot and I'd really appreciate it.

## Setup

There's two ways to do this, but if possible I really REALLY recommend the first
method, and you'll see why.

**Method 1: By downloading github releases (only works for Windows as of v1.0.0):**

- Download a .zip with the version you want and the right OS.
- Extract it somewhere, like your desktop.
- Copy the address of the zip you just extracted and put it in environmental path
  variables (for reference, at least on Windows the path should look something like
  ```C:\Users\tousef-refuge\lunellum-v1.0.0-x86_64-windows\```). To learn how to
  actually do this search up a video or something cause apparently it differs
  from OS to OS.
- Once downloaded, the setup is basically complete! You can now call ```lll``` in
  your terminal from ANY path to run commands. To update lunellum, you can simply run
  ```lll update``` and it should do it automatically.

**Method 2: By manually cloning (NOT RECOMMENDED):**

- Make sure you have cargo and git installed first.
- Use ```cd dir``` to go to any directory of your choice and then run
  ```git clone https://github.com/tousef-refuge/lunellum.git```. After that run
  ```cd dir lunellum```
- Run ```cargo install --path . --force``` followed by ```cargo build --release```
- If everything is right you should have the required binary in target/release/,
  and it will be named ```lunellum``` ending with the correct extension depending
  on your OS (.exe for Windows for example)
- Rename this file to ```lll``` (or don't, but it's called lll for faster typing so
  for your own experience you should do that), and move the binary somewhere else.
- Finally put the path of the binary in environmental variables like in the previous
  step, and the setup should be finished. Do note that ```lll update``` will NOT work
  if you do this approach, and for each new release you will need to do this whole
  process all over again.
