;***********
; CONSTANTS
;***********
; Defines names for commonly used values and addresses to make
; the code more readable.
;**************************************************************

;*****************************
; INITIAL VDP REGISTER VALUES
;*****************************
; 24 register values to be copied to the VDP during initialisation.
; These specify things like initial width/height of the planes,
; addresses within VRAM to find scroll/sprite data, the
; background palette/colour index, whether or not the display
; is on, and clears initial values for things like DMA.
;==============================================================
VDPRegisters:
	dc.b 0x14 ; 0x00:  H interrupt on, palettes on
	dc.b 0x74 ; 0x01:  V interrupt on, display on, DMA on, Genesis mode on
	dc.b 0x30 ; 0x02:  Pattern table for Scroll Plane A at VRAM 0xC000 (bits 3-5 = bits 13-15)
	dc.b 0x00 ; 0x03:  Pattern table for Window Plane at VRAM 0x0000 (disabled) (bits 1-5 = bits 11-15)
	dc.b 0x07 ; 0x04:  Pattern table for Scroll Plane B at VRAM 0xE000 (bits 0-2 = bits 11-15)
	dc.b 0x78 ; 0x05:  Sprite table at VRAM 0xF000 (bits 0-6 = bits 9-15)
	dc.b 0x00 ; 0x06:  Unused
	dc.b 0x00 ; 0x07:  Background colour: bits 0-3 = colour, bits 4-5 = palette
	dc.b 0x00 ; 0x08:  Unused
	dc.b 0x00 ; 0x09:  Unused
	dc.b 0x08 ; 0x0A: Frequency of Horiz. interrupt in Rasters (number of lines travelled by the beam)
	dc.b 0x00 ; 0x0B: External interrupts off, V scroll per-page, H scroll per-page
	dc.b 0x81 ; 0x0C: Shadows and highlights off, interlace off, H40 mode (320 x 224 screen res)
	dc.b 0x3F ; 0x0D: Horiz. scroll table at VRAM 0xFC00 (bits 0-5)
	dc.b 0x00 ; 0x0E: Unused
	dc.b 0x02 ; 0x0F: Autoincrement 2 bytes
	dc.b 0x01 ; 0x10: Scroll plane size: 64x32 tiles
	dc.b 0x00 ; 0x11: Window Plane X pos 0 left (pos in bits 0-4, left/right in bit 7)
	dc.b 0x00 ; 0x12: Window Plane Y pos 0 up (pos in bits 0-4, up/down in bit 7)
	dc.b 0xFF ; 0x13: DMA length lo byte
	dc.b 0xFF ; 0x14: DMA length hi byte
	dc.b 0x00 ; 0x15: DMA source address lo byte
	dc.b 0x00 ; 0x16: DMA source address mid byte
	dc.b 0x80 ; 0x17: DMA source address hi byte, memory-to-VRAM mode (bits 6-7)
	
	even

; VDP port addresses
vdp_control				equ 0x00C00004
vdp_data				equ 0x00C00000

; VDP commands
vdp_cmd_vram_write		equ 0x40000000
vdp_cmd_cram_write		equ 0xC0000000
vdp_cmd_vsram_write		equ 0x40000010	; NEW to this demo - Vertical Scroll RAM address

; VDP memory addresses
; according to VDP registers 0x2, 0x4, and 0xD (see table above)
vram_addr_tiles			equ 0x0000
vram_addr_plane_a		equ 0xC000
vram_addr_plane_b		equ 0xE000

; Screen width and height (in pixels)
vdp_screen_width		equ 0x0140
vdp_screen_height		equ 0x00F0

; The plane width and height (in tiles)
; according to VDP register 0x10 (see table above)
vdp_plane_width			equ 0x40
vdp_plane_height		equ 0x20

; VDP register commands to handle DMA:
vdpreg_dmalen_l  		equ 0x9300  ; DMA length (low)
vdpreg_dmalen_h  		equ 0x9400  ; DMA length (high)
vdpreg_dmasrc_l			equ 0x9500  ; DMA source (low)
vdpreg_dmasrc_m			equ 0x9600  ; DMA source (mid)
vdpreg_dmasrc_h			equ 0x9700  ; DMA source (high)
vram_dma_cmd   			equ 0x40000080
cram_dma_cmd  			equ 0xC0000080
vsram_dma_cmd  			equ 0x40000090

; Hardware version address
hardware_ver_address	equ 0x00A10001

; TMSS
tmss_address			equ 0x00A14000
tmss_signature			equ 'SEGA'

; The size of a word and longword
size_word				equ 2
size_long				equ 4

; The size of one palette (in bytes, words, and longwords)
size_palette_b			equ 0x20
size_palette_w			equ size_palette_b/size_word
size_palette_l			equ size_palette_b/size_long

; The size of one graphics tile (in bytes, words, and longwords)
size_tile_b				equ 0x20
size_tile_w				equ size_tile_b/size_word
size_tile_l				equ size_tile_b/size_long

; Text draw position (in tiles)
text_pos_x				equ 0x08
text_pos_y				equ 0x10

; Speed (in pixels per frame) to move our scroll planes
plane_a_scroll_speed_x	equ 0x2
plane_b_scroll_speed_y	equ 0x1

;*********
; PALETTE
;*********
; A single colour palette (16 colours) we'll be using to draw text.
; Colour #0 is always transparent, no matter what colour value
; you specify.
;==============================================================
; Each colour is in binary format 0000 BBB0 GGG0 RRR0,
; so 0x0000 is black, 0x0EEE is white (NOT 0x0FFF, since the
; bottom bit is discarded), 0x000E is red, 0x00E0 is green, and
; 0x0E00 is blue.
;
; For this sample we cycle through a list of some of these,
; they're repeated through it, but I'm not really good at
; deciding colors anyway :'D.
;==============================================================
Palette:
	dc.w 0x0000	; Colour 0 = Transparent
	dc.w 0x0000	; Colour 1 = Black
	dc.w 0x0EEE	; Colour 2 = White
	dc.w 0x000E	; Colour 3 = Red
	dc.w 0x00E0	; Colour 4 = Blue
	dc.w 0x0E00	; Colour 5 = Green
	dc.w 0x0E0E	; Colour 6 = Pink
	dc.w 0x0000	; Colour 7 = Black
	dc.w 0x0EEE	; Colour 8 = White
	dc.w 0x000E	; Colour 9 = Red
	dc.w 0x00E0	; Colour 10 = Blue
	dc.w 0x0E00	; Colour 11 = Green
	dc.w 0x0E0E	; Colour 12 = Pink
	dc.w 0x000E	; Colour 13 = Red
	dc.w 0x00E0	; Colour 14 = Blue
	dc.w 0x0E00	; Colour 15 = Green