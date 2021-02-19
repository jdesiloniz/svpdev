;**************************************************************
; SVP Tests
;**************************************************************
; by Javi Taiyou
;**************************************************************
; This sample is a way to run basic code tests on the SVP
; in order to find out more about its behavior.
;
; This is WIP at this moment and just performs a basic comms check.
;
; You can assemble the M68000 part of this sample with `SNASM68K.EXE`:
;
;    SNASM68K.EXE /p main.asm,output.map,output.lst,output.bin
;
; main.asm = this source file
; output.bin = Mega Drive/Genesis part of this ROM, to be later used by the SSP16xx (SVP) assembler.
; output.lst = listing file, shows assembled addresses alongside your source code (useful for debugging).
; output.map = symbol map file for linking (unused)
;
; The M68000 part of the binary needs to be later fed up to the SSP16xx assembler to build
; the SVP part of the code on top of it. You can download it from: 
;
; https://github.com/jdesiloniz/svpdev/tools/ssp16asm
;
; To assemble the SVP part and generate the final ROM:
;
; ssp16asm -b output.bin tests.svp rom_svp.bin
;
; svp_gfx.svp = source containing the SVP part of this sample.
; output.bin = M68000 binary file.
; rom_svp.bin = final ROM containing both M68000 and SSP16xx code to be run on hardware
; 				or emulators that support SVP emulation!
;

;**************************************************************
; ACKNOWLEDGEMENT
;**************************************************************
;
; Many parts in the M68000 side of this sample are based on the work of other developers in 
; the Mega Drive/Genesis scene, specially the following samples by Matt Phillips/BigEvilCorporation:
;
; https://github.com/BigEvilCorporation/megadrive_samples/
;
; The amazing technical documentation found in the Plutiedev site were a big help too: https://plutiedev.com/
; 
; Finally the reverse engineering on the SVP originally made by Tasco Deluxe and Grazvydas Ignotas (notaz),
; which obviously helped a lot for the development of the SSP16xx assembler, and the understanding of its behavior.

;**************************************************************
; INCLUDES
;**************************************************************
; Constants + routines.
; Warning: this section shouldn't ever exceed 0x800 bytes
; SVP will try to access 0x800 as its own entry point, so ROM
; space between 0x800-0x1FFFF should be left out for SVP code.
;**************************************************************

	include 'init.asm'			; ROM header and initialization routines

;**************************************************************
; SVP CODE PADDING
;**************************************************************
; SVP will start looking for code at address 0x800, let's leave
; space until address 0x20000 (as Virtua Racing did) so that
; we can later put SSP1601 code in the following blanks:
;***************************************************************
SVP_PaddingStart:

	DCB.b 0x1FFF4-SVP_PaddingStart,0x00
	DC.l  0xDEADBEEF
	DC.l  0xDEADBEEF
	DC.l  0xDEADBEEF

	include 'constants.asm'		; Constants
	include 'macros.asm'		; VDP macros
	include 'text.asm'			; Text drawing routines
	include 'pixelfont.asm'		; Font
	include 'vdp_utils.asm'		; VDP utils
	include 'svp.asm'			; SVP comms
	include 'dump.asm' 			; dump routines
	include 'pads.asm'

;**************************************************************
; MEMORY MANAGEMENT
;**************************************************************
	RSSET 0x00FF0000				; Start a new offset table from beginning of RAM
page_initial_address		rs.w 1	; initial address to display in this screen (for the indices to show at the left for reference)
pad_press    				rs.w 1	; stores previous pad to check if buttons have changed

;**************************************************************
; CODE ENTRY POINT
;**************************************************************
; The "main()" function. Your code starts here. Once the CPU
; has finished initialising, it will jump to this entry point
; (specified in the vector table at the top of the file).
;
; This should usually be address 0x20000, right after the space
; reserved for SVP code inside the ROM.
;**************************************************************
CPU_EntryPoint:

	;***************************
	; Initialise the Mega Drive
	;***************************

	; Write the TMSS signature (if a model 1+ Mega Drive)
	jsr    	VDP_WriteTMSS

	; Load the initial VDP registers
	jsr    	VDP_LoadRegisters
	jsr 	PAD_InitPads
	jsr 	VDP_ClearVRAM
	jsr 	VDP_ClearCRAM
	jsr 	LoadTextTiles
	
	;**************
	; Write titles
	;**************

	; Init flag for buttons pressed
	move.w #0x0000, (pad_press)

	; Draw main title:
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_title)*vdp_plane_width)+text_pos_x_title)*size_word)
	lea StringTitle, a0			; address for string
	jsr DrawTextPlaneANew

	; Tell SVP we're starting to operate with it
	jsr InitSVP

	; Store initial address
	move.w #pramStartAddress, (page_initial_address)

	; Ask data for first batch of data in DRAM:
	jsr SVPDumpData

	jsr VDP_InitStatusRegister

	@Stop:
	bra @stop

	; Finished!
	
; ********************************************************************************************
; INTERRUPT ROUTINES
; ********************************************************************************************
; The interrupt routines, as specified in the vector table at the top of the file.
; Note that we use `rte` to return from an interrupt, not `rts` like a subroutine.
; ********************************************************************************************

; Vertical interrupt - run once per frame (50hz in PAL, 60hz in NTSC)
INT_VInterrupt:
	; Print data already in DRAM:
	jsr PrintDumpedData

	; Read pad and check if we need to change "page"
	jsr    PAD_ReadPadA
	
	move.w (pad_press), d1
	cmp.w  d1, d0
	bne @HandleButtonPressChange

	rte

@HandleButtonPressChange
	move.w d0, (pad_press)
	btst   #pad_button_right, d0
	bne    @IncreasePage

	btst 	#pad_button_left, d0
	bne 	@DecreasePage

	rte

@IncreasePage:   ; max = FF66
    move.w (page_initial_address), d0    
    add.w #0x9A, d0
	move.w d0, d1
	sub.w #0x0001, d1
    cmp.w #0xFF66, d1
    bcc @IncreasePageMax
    move d0, (page_initial_address)
    bra @IncreasePageDump

@IncreasePageMax:
    move.w #0xFF66, (page_initial_address)

@IncreasePageDump:
	; if we changed page, ask SVP for the proper data to show next frame
    jsr SVPDumpData

@IncreasePageExit:
    rte

@DecreasePage:
    move.w (page_initial_address), d0    
    sub.w #0x9A, d0
	cmp.w #0x0000, d0
    bls @DecreasePageMin
    move d0, (page_initial_address)
    bra @DecreasePageDump

@DecreasePageMin:
    move.w #0x0000, (page_initial_address)

@DecreasePageDump:
	; if we changed page, ask SVP for the proper data to show next frame
    jsr SVPDumpData

@DecreasePageExit:
    rte

; Horizontal interrupt - run once per N scanlines (N = specified in VDP register 0xA)
INT_HInterrupt:
	rte

; NULL interrupt - for interrupts we don't care about
INT_Null:
	rte

; Exception interrupt - called if an error has occured
CPU_Exception:
	; Print "EXC" and try to go back:
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_exc)*vdp_plane_width)+text_pos_x_exc)*size_word)
	lea StringException, a0			; address for string
	jsr DrawTextPlaneANew

	rte
	
;******************************
; Strings and Text coordinates
;******************************
StringTitle:
	dc.b "SVP MEM BROWSER",0
StringException:
	dc.b "EXC", 0

; Text draw position (in tiles)
text_pos_y_title			equ 0x01
text_pos_x_title			equ 0x0C
text_pos_y_reg_pad_data_pr 	equ 0x01
text_pos_x_reg_pad_data_pr	equ 0x01
text_pos_y_reg_pad_data 	equ 0x01
text_pos_x_reg_pad_data		equ 0x10

text_pos_y_exc				equ 0x00
text_pos_x_exc				equ 0x00


; A label defining the end of ROM so we can compute the total size.
ROM_End:
