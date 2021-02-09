;**************************************************************
; SVP Basic GFX Sample
;**************************************************************
; by Javi Taiyou
;**************************************************************
; The goal of this sample is to offer an example of how to:
;
; 1) Handle communications between the Mega Drive/Genesis and the SVP chip.
; 2) Have the Mega Drive/Genesis side to request the SVP to perform operations.
; 3) How to generate simple graphical data from SVP to create a small framebuffer.
; 4) How to request graphical data from the SVP so that can drawn by the VDP.
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
; ssp16asm -b output.bin svp_gfx.svp rom_svp.bin
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
	include 'macros.asm'		; VDP macros

;**************************************************************
; MEMORY MANAGEMENT
;**************************************************************
	RSSET 0x00FF0000			; Start a new offset table from beginning of RAM
ram_current_color		rs.w 1	; 1 table entry of current color to be displayed in the frame buffer

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

	; Load the initial VDP registers and clear VRAM memory
	jsr    	VDP_LoadRegisters
	jsr 	VDP_ClearVRAM
	jsr 	VDP_ClearCRAM
	
	;********************************************
	; Write font tiles to VRAM for printing text
	;********************************************
	; Setup the VDP to write to VRAM address 0x0000 (the address of the first graphics tile, index 0)
	SetVRAMWrite vram_addr_tiles
	
	; Write the font glyph tiles to VRAM
	lea PixelFont, a0
	move.w #(text_tile_count*size_tile_l)-1, d0		; Loop counter = 8 longwords per tile * num tiles (-1 for DBRA loop)
	@CharLp:										; Start of loop
	move.l (a0)+, vdp_data							; Write tile line (4 bytes per line), and post-increment address
	dbra d0, @CharLp								; Decrement d0 and loop until finished (when d0 reaches -1)
	
	;*****************************************************************************************************
	; Write tilemap for plane B to hold the tiles generated in SVP 
	;*****************************************************************************************************
	; SVP will be generating tile data for us, but we need a tilemap in VRAM so that the VDP knows
	; which tiles to draw on the screen. This call generates a set of sequential tile indices
	; (starting from index 0x50) to create a framebuffer of 8x8 tiles to be filled by SVP graphical data.
	;*****************************************************************************************************
	lea vram_addr_plane_b, a0
	move.w #0x50, d0	; offset for data in tileset (after font tiles)
	jsr PrepareTilemapForSVP

	;***************************
	; Draw main title in screen
	;***************************
	SetVRAMWrite vram_addr_plane_a+((((text_pos_y_title)*vdp_plane_width)+text_pos_x_title)*size_word)
	lea StringTitle, a0			; address for string
	jsr DrawText

	; *************************************************************************************************
	; Prepare parameters for SVP operation
	; *************************************************************************************************
	; During the execution of this sample, the Mega Drive/Genesis will be sending a command to the SVP
	; to create a set of a tiles of an specific color (one different each frame). `d0` register will
	; hold the color number (as its index in the palette contained in CRAM). We also store the current
	; color index in RAM.
	; *************************************************************************************************
	clr.l d0
	addi #1, d0
	move.w d0, ram_current_color		; initialize ram value for current color

	; *******************************************
	; Request SVP for the first set of tile data
	; *******************************************
	jsr AskSVPForTileData

	jsr VDP_InitStatusRegister	; init interrupts

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
; Main communications with the SVP happen during this part of the code:
;
; 1) By this point, the SVP should have our tiles generated and stored in the external DRAM
;    available in the game cartridge. We need to bring these tiles to the VRAM using DMA so
;    that the VDP can display them during the next frame.
; 
; 2) Update the color to be drawn the following frame and submit it to the SVP so that it can
; 	 start generating the next tiles.
;
; ********************************************************************************************
INT_VInterrupt:
	; Prepare required data for the DMA transfer.
	; Source - tile data in DRAM: 0x301000.
	; Destination - tile graphic space in VRAM: 0x0A00 (from tile index 0x50).
	; Size: 0x1000 (4096 bytes).

	clr.l d0
	clr.l d6
	clr.l d7
	lea vdp_control, a0
	move.w 	#dramTilesSize, d0
	lsr.w   #1, d0
    
    move.l 	#dramTilesOrigin, d7
    lsr.l   #1, d7
    and.l   #0x7FFFFF, d7

	move.w  #vdpreg_dmalen_l, d6
    move.b  d0, d6
    move.w  d6, (a0)				; writing DMA_LENGTH low byte command
    
    lsr.w   #8, d0					; getting high byte
    move.w  #vdpreg_dmalen_h, d6
    move.b  d0, d6
    move.w  d6, (a0)				; writing DMA_LENGTH high byte command

	move.w  #vdpreg_dmasrc_l, d6
    move.b  d7, d6
    move.w  d6, (a0)				; writing DMA_SOURCE low byte command
    
    lsr.l   #8, d7
    move.w  #vdpreg_dmasrc_m, d6
    move.b  d7, d6
    move.w  d6, (a0)				; writing DMA_SOURCE middle byte command
    
    lsr.l   #8, d7
    move.w  #vdpreg_dmasrc_h, d6
    move.b  d7, d6
    move.w  d6, (a0)				; writing DMA_SOURCE low byte command

	; Calculate destination address format
	move.l 	#vramTilesDestination, d1
	and.l   #0xFFFF, d1
    lsl.l   #2, d1
    lsr.w   #2, d1
    swap    d1
    or.l    #vram_dma_cmd, d1

	move.l d1, (a0)					; perform actual transfer

	; Tile data should be at VRAM at this point, generate a new color and
	; submit it to the SVP so that it can generate more tiles:
	clr.l d0
	move.w ram_current_color, d0
	add.b #1, d0
	and.w #0x000F, d0
	move.w d0, ram_current_color
	jsr AskSVPForTileData

	; Write debug data on screen:
	; show value of XST
	move.w regXST, d4
	SetVRAMWrite vram_addr_plane_a+((((text_pos_y_xst)*vdp_plane_width)+text_pos_x_xst)*size_word)
	jsr DrawNumber
	
	; show value of XST_state:
	SetVRAMWrite vram_addr_plane_a+((((text_pos_y_xst_state)*vdp_plane_width)+text_pos_x_xst_state)*size_word)
	move.w  regXSTState, d4
	jsr DrawNumber

	rte

; Horizontal interrupt - run once per N scanlines (N = specified in VDP register 0xA)
INT_HInterrupt:
	; Doesn't do anything in this sample
	rte

; NULL interrupt - for interrupts we don't care about
INT_Null:
	rte

; Exception interrupt - called if an error has occured
CPU_Exception:
	; Print "EXC" and try to go back:
	SetVRAMWrite vram_addr_plane_a+((((text_pos_y_exc)*vdp_plane_width)+text_pos_x_exc)*size_word)
	lea StringException, a0			; address for string
	jsr DrawText

	rte
	
;******************************
; Strings and Text coordinates
;******************************
StringTitle:
	dc.b "SVP DRAW TEST",0
StringException:
	dc.b "EXC", 0

; Text draw position (in tiles)
text_pos_y_title			equ 0x01
text_pos_x_title			equ 0x0D
text_pos_y_xst				equ 0x18
text_pos_x_xst				equ 0x02
text_pos_y_xst_state		equ 0x19
text_pos_x_xst_state		equ 0x02
text_pos_y_dram_init		equ 0x1A
text_pos_x_dram_init		equ 0x02
text_pos_y_exc				equ 0x00
text_pos_x_exc				equ 0x00

	include 'constants.asm'		; Constants
	include 'text.asm'			; Text drawing routines
	include 'pixelfont.asm'		; Font
	include 'vdp_utils.asm'		; VDP utils
	include 'svp.asm'			; SVP comms

; A label defining the end of ROM so we can compute the total size.
ROM_End:
