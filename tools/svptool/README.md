## SVPTOOL

This tool allows to swap the endianness and split the resulting assembled files so that they can be stored in multiple EEPROMs and be run in real hardware. Even though it's named that way I guess this can be used for any kind of binary files as long as they're 16 bit in nature.

## Usage

`SSPTOOL` requires an input filename and an output prefix for the resulting splitted files. By default files will be splitted in 512KB sized files and will retain their original endianness. The following parameters allow to change this behavior:

- `-e`: reverses the original endianness for the resulting binary files.
- `-s`: splits the original binary file into multiple files (not using this is useful if you just want to reverse the original endianness of the input file).
- `-k`: specifies the size (in KB) for the resulting binary files. Defaults to 512.

## License

This code is MIT-licensed. Also take into account the following conditions of use:

* Please use this code for good. Also for fun. But good fun, not evil fun.
* If you build something really cool (moderately cool also works) please drop me a comment at `taiyou[at]gmail.com`.
* You're not forced, but if you use this code I'd appreciate if you could acknowledge me :).