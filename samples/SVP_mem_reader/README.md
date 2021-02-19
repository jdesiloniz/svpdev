# SVP Mem Reader

This sample allows browsing program memory through the internal view in SVP. Pages in the memory results can be changed using **left** and **right** buttons of the Mega Drive pad. This ROM is what allowed discovery of the [SVP Internal ROM](https://github.com/jdesiloniz/svpdev/wiki/Internal-ROM).

## Code structure

- **main.asm**: main file for the 68000 side of the sample. Sends requests to the SVP to gather data from its internal view of the program memory and write it up to DRAM to be read by the M68000 side.
- **svp.asm**: additional routines to communicate with the SVP side.
- **svp_prog_access.svp**: main file for the SVP side of the sample. Allows receiving requests from the Mega Drive side and copy data from the internal view of the program memory inside the SVP to DRAM.

Besides these, other files handle VDP initialization, printing text in screen, etc...

## How to build

### Motorola 68000 side

You can assemble the M68000 part of this sample with `SNASM68K.EXE`:

`SNASM68K.EXE /p main.asm,output.map,output.lst,output.bin`

### Samsung SSP160x

The Motorola 68000 part of the binary needs to be later fed up to the [SSP16xx assembler](../../tools/ssp16asm) in order to build the SVP part of the code on top of it. To assemble the SVP part and generate the final ROM:

`ssp16asm -b output.bin svp_prog_access.svp rom_svp.bin`

## Acnowledgement

Many parts in the M68000 side of this sample are based on the work of other developers in 
the Mega Drive/Genesis scene, specially the [following samples](https://github.com/BigEvilCorporation/megadrive_samples/) by Matt Phillips/BigEvilCorporation.

The amazing technical documentation found in the [Plutiedev site](https://plutiedev.com/) were a big help too.

Finally the reverse engineering on the SVP originally made by Tasco Deluxe and Grazvydas Ignotas (notaz),
which obviously helped a lot for the development of the SSP16xx assembler, and the understanding of its behavior.

## License

This code is MIT-licensed. Also take into account the following conditions of use:

* Please use this code sample for good. Also for fun. But good fun, not evil fun. 
* If you build something really cool (moderately cool also works) please drop me a comment at `taiyou[at]gmail.com`.
* You're not forced, but if you use this code I'd appreciate if you could acknowledge me :).