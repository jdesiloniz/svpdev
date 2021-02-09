	include 'charmap.asm'

	; Align 8 bytes
	nop 0,8

DrawText:
    ; d0 (w) - First tile ID of font
    ; a0 (l) - String address
    ; requires previously setting text position through: SetVRAMWrite vram_addr_plane_a+(((text_pos_y*vdp_plane_width)+text_pos_x)*size_word)

	lea ASCIIMap, a1           	; Load address of ASCII map into a1
	clr.l 	d2
	clr.l 	d3
@CharLookup:
	move.b   (a0)+, d2              ; Move ASCII byte to lower byte of d2
	cmp.b    #0x0, d2               ; Test if byte is zero (string terminator)
	beq.b    @CharEnd               ; If byte was zero, branch to end

	sub.b    #ASCIIStart, d2        ; Subtract first ASCII code to get table entry index
	move.b  (a1,d2.w), d3           ; Move tile ID from table (index in lower word of d2) to lower byte of d3
	move.w 	d3, vdp_data			; draw letter
	bra @CharLookup
@CharEnd:
	rts

DrawNumber:
	; d4 (w) - Number
	; d0 (w) - First tile ID of font
	; requires previously setting text position through: SetVRAMWrite vram_addr_plane_a+(((text_pos_y*vdp_plane_width)+text_pos_x)*size_word)

	clr.l 	d2
	clr.l 	d3
	clr.l 	d5
	clr.l 	d6
	move.w 	 #12, d6					; number of shifts to apply to the original number, we start with the bigger figure	
	lea      ASCIIMap, a1           ; Load address of ASCII map into a1

@number_calculation_loop:
	move.w 	d4, d5					; Copy original number
	lsr.w 	d6, d5					; Shift figure and mask it to draw
	and.w 	#0xF, d5
	
	cmp.b #10, d5		; check if number is bigger than 9
	bge @bigger_num
@small_num:	
	add.w #16, d5		; get char for this number
	bra @draw_num
@bigger_num:
	add.w #23, d5
@draw_num 
	move.b   (a1,d5.w), d3          ; Move tile ID from table (index in lower word of d2) to lower byte of d3
	move.w   d3, vdp_data           ; Move palette and pattern IDs to VDP data port

	subq #4, d6
	bmi @end
	bra @number_calculation_loop
@end:
	rts