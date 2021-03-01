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

	DCB.b 0x20000-SVP_PaddingStart,0x00

; Rest of includes
	include 'constants.asm'		; Constants
	include 'macros.asm'		; VDP macros
	include 'text.asm'			; Text drawing routines
	include 'pixelfont.asm'		; Font
	include 'vdp_utils.asm'		; VDP utils
	include 'svp.asm'			; SVP comms
	include 'pads.asm'			; Control pads

;**************************************************************
; MEMORY MANAGEMENT
;**************************************************************
	RSSET 0x00FF0000					; Start a new offset table from beginning of RAM
ram_current_state		rs.w 1		; 1 table entry for current UI state
ram_svp_done_flag 		rs.w 1		; flag to note if SVP has initialized tests
ram_test_frames			rs.w 1 		; number of executed test frames
ram_test_acc			rs.w 1 		; number of executed test frames once they've exceeded overflow value
pad_press    			rs.w 1		; stores previous pad to check if buttons have changed


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

	; Initial UI status
	move.w #ui_init, (ram_current_state)
	move.w #0x0000, (ram_svp_done_flag)
	move.w #0x0000, (ram_test_frames)
	move.w #0x0000, (ram_test_acc)
	move.w #0x0000, (pad_press)
	
	;**************
	; Write titles
	;**************

	; Draw main title:
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_title)*vdp_plane_width)+text_pos_x_title)*size_word)
	lea StringTitle, a0			; address for string
	jsr DrawTextPlaneANew

	; Ask SVP to initialize test data 
	move.w #0x0001, regXST

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
	; Draw UI text
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_status)*vdp_plane_width)+text_pos_x_status)*size_word)
	
	cmp.w #ui_init, (ram_current_state)
	beq @IsUIStateInit

	cmp.w #ui_test_rom, (ram_current_state)
	beq @IsUIStateROMTest

	cmp.w #ui_test_iram, (ram_current_state)
	beq @IsUIStateIRAMTest

	cmp.w #ui_test_irom, (ram_current_state)
	beq @IsUIStateIROMTest

@IsUIStateInit
	lea StringUIActions, a0
	bra @DrawTitle

@IsUIStateROMTest
	lea StringRunningTestROM, a0
	bra @DrawTitle

@IsUIStateIRAMTest
	lea StringRunningTestIRAM, a0
	bra @DrawTitle

@IsUIStateIROMTest
	lea StringRunningTestIROM, a0
	bra @DrawTitle

@DrawTitle
	jsr DrawTextPlaneANew

	; Draw current state of XST and test frames
	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_test_res)*vdp_plane_width)+text_pos_x_test_res)*size_word)
	move.w dramTestResult, d4
	jsr DrawNumberTextPlaneA

	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_test_acc)*vdp_plane_width)+text_pos_x_test_acc)*size_word)
	move.w dramTestResultAcc, d4
	jsr DrawNumberTextPlaneA

	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_frames)*vdp_plane_width)+text_pos_x_frames)*size_word)
	move.w (ram_test_frames), d4
	jsr DrawNumberTextPlaneA

	cmp.w #ui_init, (ram_current_state)
	bne @HandleTestState				; if we're in the middle of a test, we don't allow pad presses

@UI_Handle_Pad
	jsr IsSVPDone
	cmp.w #0x1010, d4
	bne @EndLoopVBlank					; make sure the tests are ready to go before testing anything

	SetVRAMWrite vram_addr_plane_b+((((text_pos_y_pad)*vdp_plane_width)+text_pos_x_pad)*size_word)
	move.w d0, d4
	jsr DrawNumberTextPlaneA
	
	; Read pad:
	jsr    PAD_ReadPadA
	move.w (pad_press), d1
	cmp.w  d1, d0
	bne @HandleButtonPressChange

	rte

@HandleButtonPressChange
	move.w 	d0, (pad_press)

	btst   	#pad_button_a, d0
	bne    	@RunTestROM

	btst 	#pad_button_b, d0
	bne 	@RunTestIRAM

	btst 	#pad_button_c, d0
	bne 	@RunTestIROM

	rte

@RunTestROM
	move.w #0x0000, (ram_test_acc)
	move.w #ui_test_rom, (ram_current_state)
	move.w #0x0100, regXST
	bra @EndLoopVBlank
	
@RunTestIRAM
	move.w #0x0000, (ram_test_acc)
	move.w #ui_test_iram, (ram_current_state)
	move.w #0x0200, regXST
	bra @EndLoopVBlank

@RunTestIROM
	move.w #0x0000, (ram_test_acc)
	move.w #ui_test_irom, (ram_current_state)
	move.w #0x0300, regXST
	bra @EndLoopVBlank	

	rte

@HandleTestState
	add.w #0x0001, (ram_test_frames)
	cmp.w #0xF000, (dramTestResult)		; Using a simple offset beyond FF00, may require change
	bgt @Update_Acc_TestFrames
	bra @Check_TestFrames

@Update_Acc_TestFrames
	add.w #0x0001, (ram_test_acc)

@Check_TestFrames
	cmp.w #test_max_frames, (ram_test_frames)
	bge @EndCurrentTest

	rte

@EndCurrentTest
	move.w #0xFFFF, regXST 			; After running each test, the SVP will stop if MD side has written anything to XST
	move.w #0x0000, (ram_test_frames)
	move.w #ui_init, (ram_current_state)
	rte

@EndLoopVBlank
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
	dc.b "SVP SPEED TEST",0
StringUIActions:
	dc.b "A:ROM  B:IRAM  C:IROM",0
StringRunningTestROM:
	dc.b "RUNNING ROM TEST       ",0
StringRunningTestIRAM:
	dc.b "RUNNING IRAM TEST      ",0
StringRunningTestIROM:
	dc.b "RUNNING IROM TEST      ",0
StringException:
	dc.b "EXC", 0

; Text draw position (in tiles)
text_pos_y_title			equ 0x01
text_pos_x_title			equ 0x0A
text_pos_y_status			equ 0x03
text_pos_x_status			equ 0x02
text_pos_y_frames			equ 0x18
text_pos_x_frames			equ 0x02
text_pos_y_test_res			equ 0x19
text_pos_x_test_res 		equ 0x02
text_pos_y_test_acc			equ 0x19
text_pos_x_test_acc 		equ 0x07
text_pos_y_pad				equ 0x1A
text_pos_x_pad				equ 0x02
text_pos_y_exc				equ 0x00
text_pos_x_exc				equ 0x00

; UI status
ui_init 					equ 0x0000
ui_test_rom					equ 0x0001
ui_test_iram				equ 0x0002
ui_test_irom 				equ 0x0003

; Test constants
test_max_frames				equ 0x00FA 		; 5 seconds in PAL MD (250 frames)

; A label defining the end of ROM so we can compute the total size.
ROM_End:
