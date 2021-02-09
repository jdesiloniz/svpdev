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
	include 'constants.asm'		; Constants
	include 'macros.asm'		; VDP macros
	include 'text.asm'			; Text drawing routines
	include 'pixelfont.asm'		; Font
	include 'vdp_utils.asm'		; VDP utils
	include 'svp.asm'			; SVP comms

;**************************************************************
; SVP CODE PADDING
;**************************************************************
; SVP will start looking for code at address 0x800, let's leave
; space until address 0x20000 (as Virtua Racing did) so that
; we can later put SSP1601 code in the following blanks:
;***************************************************************
SVP_PaddingStart:

	DCB.b 0x20000-SVP_PaddingStart,0x00

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
	jsr 	VDP_ClearVRAM
	jsr 	VDP_ClearCRAM
	jsr 	LoadTextTiles
	
	;**************
	; Write titles
	;**************

	; Draw main title:
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_title)*vdp_plane_width)+text_pos_x_title)*size_word)
	lea StringTitle, a0			; address for string
	jsr DrawTextPlaneANew

	; Draw test #1 title
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_test1_title)*vdp_plane_width)+text_pos_x_test1_title)*size_word)
	lea StringCommTestTitle, a0			; address for string
	jsr DrawTextPlaneANew

	;****************
	; SVP test calls
	;****************
	; Execute test 0x100 (comm test with SVP)
	move.w #0x0100, d2
	move.w #0xFFFF, d1	; number of comm check retries.
	jsr 		CheckSVPTestResult

	; show value of XST (we should update this maybe?)
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_xst)*vdp_plane_width)+text_pos_x_xst)*size_word)
	jsr DrawNumberTextPlaneA
	
	; show value of XST_state:
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_xst_state)*vdp_plane_width)+text_pos_x_xst_state)*size_word)
	move.w  regXSTState, d4
	jsr DrawNumberTextPlaneA
	
	; Show what's on DRAM now:
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_dram_init)*vdp_plane_width)+text_pos_x_dram_init)*size_word)
	lea 0x00300000, a0
    move.w (a0), d4     ; copy test results from DRAM
	jsr DrawNumberTextPlaneA
    
	cmp.w #0xFFAA, d4
	beq @TestSuccess00
	bne @TestFailure00

	@TestSuccess00:
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_test1_result)*vdp_plane_width)+text_pos_x_test1_result)*size_word)
	lea StringOK, a0			; address for string
	jsr DrawTextPlaneANew
	bra @Stop

	@TestFailure00:
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_test1_result)*vdp_plane_width)+text_pos_x_test1_result)*size_word)
	lea StringError, a0			; address for string
	jsr DrawTextPlaneANew
	bra @Stop

	@Stop:
	bra @stop

	jsr VDP_InitStatusRegister

	; Finished!
	
; ********************************************************************************************
; INTERRUPT ROUTINES
; ********************************************************************************************
; The interrupt routines, as specified in the vector table at the top of the file.
; Note that we use `rte` to return from an interrupt, not `rts` like a subroutine.
; ********************************************************************************************

; Vertical interrupt - run once per frame (50hz in PAL, 60hz in NTSC)
INT_VInterrupt:
	rte

; Horizontal interrupt - run once per N scanlines (N = specified in VDP register 0xA)
INT_HInterrupt:
	; Doesn't do anything in this demo
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
	dc.b "SVP TEST ROM",0
StringCommTestTitle:
	dc.b "BASIC SVP COMM TEST",0
StringOK:
	dc.b "OK",0
StringError:
	dc.b "ERR",0
StringException:
	dc.b "EXC", 0

; Text draw position (in tiles)
text_pos_y_title			equ 0x01
text_pos_x_title			equ 0x0D
text_pos_y_test1_title		equ 0x03
text_pos_x_test1_title		equ 0x02
text_pos_y_test1_result		equ 0x03
text_pos_x_test1_result		equ 0x20
text_pos_y_xst				equ 0x18
text_pos_x_xst				equ 0x02
text_pos_y_xst_state		equ 0x19
text_pos_x_xst_state		equ 0x02
text_pos_y_dram_init		equ 0x1A
text_pos_x_dram_init		equ 0x02
text_pos_y_exc				equ 0x00
text_pos_x_exc				equ 0x00

; A label defining the end of ROM so we can compute the total size.
ROM_End:
