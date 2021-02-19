
PrintDumpedData:
    ; page_initial_address = value of the address in SVP format (word address) to show at the left as reference

    ; internal values:
    ; a3 = initial address to dump data from
    ; a2 = pointer to current address to read (in SVP format)
    ; d7: current column counter
    ; d0: current row counter

    lea dramInit, a3

    move.w #0x0002, d0
    move.w #0x0001, d7

    move.w (page_initial_address), a2     ; store initial address (in SVP format) to iterate with, to show indices

OuterLoop:
    add.w #0x0001, d0       ; update row value
    move.w #0x0000, d7      ; reset column value
    cmpi.w #0x0019, d0      ; have we drawn the last row already
    beq DrawEnd

    jsr SetVRAMCoords

    move.w a2, d4               ; get initial page address value
    jsr DrawNumberTextPlaneA    ; draw index

WriteValues:
    ; Max column value = 0x1A (26)
    ; Max row value = 0x06

@InnerLoop:
    add.w #0x0005, d7           ; update column value
    cmpi.w #0x0024, d7          ; was this the last column? (1 + 35 used columns)
    bge OuterLoop               ; if that's so go back

    jsr SetVRAMCoords

    move.w (a3)+, d4            ; read dumped data from DRAM and post-increment +2
    
    jsr DrawNumberTextPlaneA    ; draw value in screen

    add.w #0x0001, a2               ; update index to show at the left when this row is over
    
    bra @InnerLoop                  ; Carry on with next value

DrawEnd:
    rts


SetVRAMCoords:
    ;vram_addr_plane_b+((((text_pos_y_title)*vdp_plane_width)+text_pos_x_title)*size_word)

    ; d0 = row
    ; d7 = column
    move.w #vdp_plane_width, d1     ; d1 = vdp_plane_width
    mulu.w d0, d1                   ; d1 = vdp_plane_width * y
    add.w d7, d1                    ; d1 = (vdp_plane_width * y) + x
    lsl.w #1, d1                    ; d1 = ((vdp_plane_width * y) + x) * size_word (2)
    add.w #vram_addr_plane_b, d1    ; d1 = vram_addr_plane_b + (((vdp_plane_width * y) + x) * size_word)
    SetVramAddrReg d1               ; set VRAM position

    rts