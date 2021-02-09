;********************************************************************************
; VDP UTILITY FUNCTIONS
;********************************************************************************
; Subroutines to initialise the TMSS, and load all VDP registers
;********************************************************************************

    ;**************************************************************
	; Clear VRAM (video memory)
	;**************************************************************
VDP_ClearVRAM:
	; Setup the VDP to write to VRAM address 0x0000 (start of VRAM)
	SetVRAMWrite 0x0000

	; Write 0's across all of VRAM
	move.w #(0x00010000/size_word)-1, d0	; Loop counter = 64kb, in words (-1 for DBRA loop)
	@ClrVramLp:								; Start of loop
	move.w #0x0, vdp_data					; Write a 0x0000 (word size) to VRAM
	dbra   d0, @ClrVramLp					; Decrement d0 and loop until finished (when d0 reaches -1)

    rts

    ;**************************************************************
	; Write the palette to CRAM (colour memory)
	;**************************************************************
VDP_ClearCRAM:
	; Setup the VDP to write to CRAM address 0x0000 (first palette)
	SetCRAMWrite 0x0000
	
	; Write the palette to CRAM
	lea    Palette, a0				; Move palette address to a0
	move.w #size_palette_w-1, d0	; Loop counter = 8 words in palette (-1 for DBRA loop)
	@PalLp:							; Start of loop
	move.w (a0)+, vdp_data			; Write palette entry, post-increment address
	dbra d0, @PalLp					; Decrement d0 and loop until finished (when d0 reaches -1)

    rts

	;**************************************************************
	; TMSS handling
	;**************************************************************
VDP_WriteTMSS:

	; The TMSS (Trademark Security System) locks up the VDP if we don't
	; write the string 'SEGA' to a special address. This was to discourage
	; unlicensed developers, since doing this displays the "LICENSED BY SEGA
	; ENTERPRISES LTD" message to screen (on Mega Drive models 1 and higher).
	;
	; First, we need to check if we're running on a model 1+, then write
	; 'SEGA' to hardware address 0xA14000.

	move.b hardware_ver_address, d0			; Move Megadrive hardware version to d0
	andi.b #0x0F, d0						; The version is stored in last four bits, so mask it with 0F
	beq    @SkipTMSS						; If version is equal to 0, skip TMSS signature
	move.l #tmss_signature, tmss_address	; Move the string "SEGA" to 0xA14000
	@SkipTMSS:

	; Check VDP
	move.w vdp_control, d0					; Read VDP status register (hangs if no access)
	
	rts

	;**************************************************************
	; Initialising the VDP
	;**************************************************************
VDP_LoadRegisters:
	; To initialise the VDP, we write all of its initial register values from
	; the table at the top of the file, using a loop.
	;
	; To write a register, we write a word to the control port.
	; The top bit must be set to 1 (so 0x8000), bits 8-12 specify the register
	; number to write to, and the bottom byte is the value to set.
	;
	; In binary:
	;   100X XXXX YYYY YYYY
	;   X = register number
	;   Y = value to write

	; Set VDP registers
	lea    VDPRegisters, a0		; Load address of register table into a0
	move.w #0x18-1, d0			; 24 registers to write (-1 for loop counter)
	move.w #0x8000, d1			; 'Set register 0' command to d1

	@CopyRegLp:
	move.b (a0)+, d1			; Move register value from table to lower byte of d1 (and post-increment the table address for next time)
	move.w d1, vdp_control		; Write command and value to VDP control port
	addi.w #0x0100, d1			; Increment register #
	dbra   d0, @CopyRegLp		; Decrement d0, and jump back to top of loop if d0 is still >= 0
	
	rts

	;**************************************************************
	; Initialise status register and set interrupt level.
	; This begins firing vertical and horizontal interrupts.
	;
	; Since the vinterrupt does something meaningful in this
	; demo, we start this AFTER setting up the VDP to draw and
	; intialising the variables in RAM.
	;**************************************************************

VDP_InitStatusRegister:
	move.w #0x2300, sr

	rts