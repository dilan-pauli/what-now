# What Now?

A rust cli app that is used to display to the user what they should be working
on for the next hour.

I have an issue where I have lots of things that I want to do in my personal
life and I have trouble starting any of them. This app will let the
user load a local file with things that they want to so then when they called
the app it will randomly pick one to display to the user for them
to work on. The file is located in ~/.config/whatnow/config.tomal

`whatnow add "Read"`
`whatnow add "Yoga"`
`whatnow add "Run"`
`whatnow add "Meditate"`

`whatnow`

"Read"

`whatnow list`

"Read"
"Yoga"
"Run"
"Meditate"

## Architecture

Hand coded Rust used to increase the programmers knowledge of Rust language.
Using as little dependancies as possible.

STDIO, and File Manipulation is required as well as working with random.
