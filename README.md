# SVP Development Tools

A collection of development tools targetting SEGA's SVP chip found in the Mega Drive/Genesis version of Virtua Racing.

## Project structure

- **boards**: KiCAD format files of boards that allow running custom code on an actual SVP chip.
- **samples**: simple examples containing both DSP/68000 code to help developers not to start from scratch.
- **tools**: (kinda crude) tools to create software for the SVP, containing an SSP160x assembler and a ROM file manipulation tool.

## Goals

- Be able to run arbitrary code on the real SVP chip in order to research many unknowns in its behavior, and document these.
- Allow an "easy" access for people interested in this piece of videogame history to run code on it too, whether it's additional research, demos or actual new games (looking forward to it!).

## License

This code is MIT-licensed. Also take into account the following conditions of use:

* Please use these tools for good. Also for fun. But good fun, not evil fun. 
* If you build something really cool (moderately cool also works for me) please drop a comment at `taiyou[at]gmail.com`.
* You're not forced, but if you use these code/tools I'd appreciate if you could acknowledge me :).