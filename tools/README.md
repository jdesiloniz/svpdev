# ssp16asm

A quick and dirty assembler for the Samsung SSP16xx family of DSPs, notably known for its use (contained within the **SVP** chip - *Sega Virtua Processor*) in the Mega Drive/Genesis version of the game **Virtua Racing**.

## Assembler

Further documentation for the assembler can be found in the [assembler source folder](./ssp16asm).

## SVPTOOL

An additional tool is available in the [svptool](./svptool) folder. It allows to swap the endianness and split the resulting assembled files so that they can be stored in multiple EEPROMs and be run in real hardware. Even though it's named that way I guess this can be used for any kind of binary files as long as they're 16 bit in nature.

## License

This code is MIT-licensed. Also take into account the following conditions of use:

* Please use this code for good. Also for fun. But good fun, not evil fun.
* If you build something really cool (moderately cool also works) please drop me a comment at `taiyou[at]gmail.com`.
* You're not forced, but if you use this code I'd appreciate if you could acknowledge me :).