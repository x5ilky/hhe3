# Haunted House Engine
**HHE3** is the *third* iteration of the haunted house engines.
It is a program that allows [stories](#story) to be played.

## Story
For a bit of a history lesson. This all started with my friends making games we called "haunted houses".

All stories go in a `stories/` folder relative to the current working directory. They are in the format as follows:
* `stories/` directory
  * `foo-story/` folder
    * `meta.toml` configuration file
    * `rooms/` directory of rooms
      * `bar.hh3` - different [rooms](#room)

## Room
Each room has three parts:
* Pre
* Content
* Post

The *pre* and *post* are all portions of lisp.
> Documentation for lisp can be found [here](./DOCS.md#documentation).

The *content* is text, with lisp contained in backticks.
The room's id is the filename without extension.
