;**************************************************************
; VRAM WRITE MACROS
;**************************************************************
; Some utility macros to help generate addresses and commands for
; writing data to video memory, since they're tricky (and
; error prone) to calculate manually.
; The resulting command and address is written to the VDP's
; control port, ready to accept data in the data port.
;**************************************************************
	
; Set the VRAM (video RAM) address to write to next
SetVRAMWrite: macro addr
	move.l  #(vdp_cmd_vram_write)|((\addr)&$3FFF)<<16|(\addr)>>14, vdp_control
	endm
	
; Set the CRAM (colour RAM) address to write to next
SetCRAMWrite: macro addr
	move.l  #(vdp_cmd_cram_write)|((\addr)&$3FFF)<<16|(\addr)>>14, vdp_control
	endm

; Set the VSRAM (vertical scroll RAM) address to write to next
SetVSRAMWrite: macro addr
	move.l  #(vdp_cmd_vsram_write)|((\addr)&$3FFF)<<16|(\addr)>>14, vdp_control
	endm

SetXramAddrReg: macro reg, cmd
    and.l   #0xFFFF, \reg
    lsl.l   #2, \reg
    lsr.w   #2, \reg
    swap    \reg
    or.l    #cmd, \reg
    move.l  \reg, (vdp_control)
    endm

SetVramAddrReg: macro reg
    SetXramAddrReg \reg, vdp_cmd_vram_write
    endm

SetCramAddrReg: macro reg
    SetXramAddrReg \reg, vdp_cmd_cram_write
    endm

SetVsramAddrReg: macro reg
    SetXramAddrReg \reg, vdp_cmd_vsram_write
    endm
