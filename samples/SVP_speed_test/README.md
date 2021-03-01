# SVP Speed Test

This code sample executes a routine in the SVP side multiple times for a specific set of time (determined by the Mega Drive side - in this example during 250 frames or 5 seconds in an European console), in order to determine how fast code is run from the IRAM and IROM compared to the external ROM. At boot-up you can choose from which source to run the test (by pressing A, B or C in the control pad).

## Code structure

- **main.asm**: main file for the 68000 side of the sample. It allows sending "test commands" to the SVP to run the speed tests from the different sources, showing the results on screen.
- **tests.svp**: contains the code to run the different tests. The routine being run is taken from the SVP's internal ROM disassembly to be able to test the speed of code run from there too, but adapted to refer to the proper jump addresses in both the ROM and IRAM sides.

Besides these, other files handle VDP initialization, printing text in screen, etc...

## How to build

### Motorola 68000 side

You can assemble the M68000 part of this sample with `SNASM68K.EXE`:

`SNASM68K.EXE /p main.asm,output.map,output.lst,output.bin`

### Samsung SSP160x

The Motorola 68000 part of the binary needs to be later fed up to the [SSP16xx assembler](../../tools/ssp16asm) in order to build the SVP part of the code on top of it. To assemble the SVP part and generate the final ROM:

`ssp16asm -b output.bin tests.svp rom_svp.bin`

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