# SVP samples

A collection of code samples targetting SEGA's SVP chip found in the Mega Drive/Genesis version of Virtua Racing.

## Samples

- **SVP_basic_gfx**: a basic example showcasing basic Mega Drive/SVP communications + creating pixel data in SVP + sending data back to the Mega Drive side.
- **SVP_tests**: a collection of "unit tests" for the SVP code (WIP: currently just tests basic communications between SVP and Mega Drive).
- **SVP_mem_reader**: allows browsing program memory through the internal view in SVP. This is what allowed discovery of the [SVP Internal ROM](https://github.com/jdesiloniz/svpdev/wiki/Internal-ROM).
- **SVP_speed_test**: runs a basic benchmark to research on how fast code runs from IRAM, ROM and IROM.

## Warning

I'm not, by any means, a Mega Drive developer. So please be aware that most of the Motorola 68000 routines contained in these samples are just retrieved from other ones or just naive reinterpretations of documentation found on the Internet and could really use an optimization. They're released with the only intention to help people wanting a quick way to start doing SVP code on their own.

Also note that these samples run without an SVP will behave differently depending on where you try to run them: in a real Mega Drive (or in MiSTer/Mega SG) the code will freeze (it seems that attempting to access external registers outside the Virtua Racing cartridge leaves the 68000 trying to access these without luck), but software emulators allow these to run without freezing (but obviously not behaving as expected).

## Acnowledgement

Many parts in the M68000 side of this sample are based on the work of other developers in 
the Mega Drive/Genesis scene, specially the [following samples](https://github.com/BigEvilCorporation/megadrive_samples/) by Matt Phillips/BigEvilCorporation.

The amazing technical documentation found in the [Plutiedev site](https://plutiedev.com/) were a big help too.

Finally the reverse engineering on the SVP originally made by Tasco Deluxe and Grazvydas Ignotas (notaz),
which obviously helped a lot for the development of the SSP16xx assembler, and the understanding of its behavior.

## License

This code is MIT-licensed. Also take into account the following conditions of use:

* Please use these code samples for good. Also for fun. But good fun, not evil fun. 
* If you build something really cool (moderately cool also works) please drop me a comment at `taiyou[at]gmail.com`.
* You're not forced, but if you use this code I'd appreciate if you could acknowledge me :).