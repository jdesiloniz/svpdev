pad_ctrl_a				equ 0x00A10009	; IO port A control port
pad_ctrl_b				equ 0x00A1000B	; IO port B control port
pad_data_a				equ 0x00A10003	; IO port A data port
pad_data_b				equ 0x00A10005	; IO port B data port

pad_byte_latch			equ 0x40

pad_button_up			equ 0x0
pad_button_down			equ 0x1
pad_button_left			equ 0x2
pad_button_right		equ 0x3
pad_button_a			equ 0xC
pad_button_b			equ 0x4
pad_button_c			equ 0x5
pad_button_start		equ 0xD

; All gamepad button bits (for masking)
pad_button_all			equ 0x303F

PAD_InitPads:

	; Initialise both gamepad IO ports by writing the latch bit
	; to each pad's control port.
	move.b #pad_byte_latch, pad_ctrl_a
	move.b #pad_byte_latch, pad_ctrl_b

	rts

PAD_ReadPadA:
	; Returns: d0 (word) - pad A state in format 00SA0000 00CBRLDU

	; To read a gamepad, we need to read one byte at a time from
	; address A10003 (gamepad port 1) or A10005 (gamepad port 2).
	; To do this, we write to the port first to tell it whether we
	; we want the first or the second byte of data, then read from it.
	;
	; The first byte contains the Start and A button states (in binary
	; format 00SA0000), and the second byte contains C, B, RIGHT, LEFT,
	; UP, and DOWN button states (in binary format 00CBRLDU).
	;
	; 6-button pads are a little more complex, and are beyond the
	; scope of this sample.
	
	; First, write 0 to the data port for pad A to tell it we want
	; the first byte (clears the "latch" bit).
	move.b  #0x0, pad_data_a

	; Delay by 2 NOPs (opcodes that do nothing) to ensure the
	; request was received before continuing. This was recommended
	; by a SEGA developer bulletin in response to some rare cases
	; where the data port was returning incorrect data.
	nop
	nop

	; Read the first byte of data from the data port
	move.b  pad_data_a, d0

	; Shift the byte into place in register d0 (we are returning
	; both bytes as a single word from this routine).
	lsl.w   #0x8, d0

	; Write the "latch" bit, to tell it we want to read the second
	; byte next.
	move.b  #pad_byte_latch, pad_data_a

	; 2-NOP delay to respond to change
	nop
	nop

	; Read the second byte of data from data port
	move.b  pad_data_a, d0
	
	; Invert and mask all bytes received.
	; The data port returns the button state bits as 1=button up,
	; 0=button down, which doesnt make sense when using it in game code.
	;
	; We also clear any unused bits, so we can determine if ANY buttons
	; are held by checking if the returned word is non-zero.
	neg.w   d0
	subq.w  #0x1, d0
	andi.w  #pad_button_all, d0

	rts