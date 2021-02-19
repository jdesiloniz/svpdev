; A label defining the start of ROM so we can compute the total size.
ROM_Start:

;**************************************************************
; CPU VECTOR TABLE
;**************************************************************
; A table of addresses that the CPU needs to know about -
; things like stack address, "main()" function address,
; vertical/horizontal interrupt addresses, etc.
;**************************************************************
; For any interrupts we don't want to handle in this demo,
; we specify INT_Null (an interrupt at the bottom of the
; file that doesn't do anything).
;**************************************************************
; This must be the very first thing in the ROM, since the CPU
; reads it from 0x0000 on bootup.
;**************************************************************
	dc.l   0x00FFE000			; Initial stack pointer value
	dc.l   CPU_EntryPoint		; Start of program
	dc.l   CPU_Exception 		; Bus error
	dc.l   CPU_Exception 		; Address error
	dc.l   CPU_Exception 		; Illegal instruction
	dc.l   CPU_Exception 		; Division by zero
	dc.l   CPU_Exception 		; CHK CPU_Exception
	dc.l   CPU_Exception 		; TRAPV CPU_Exception
	dc.l   CPU_Exception 		; Privilege violation
	dc.l   INT_Null				; TRACE exception
	dc.l   INT_Null				; Line-A emulator
	dc.l   INT_Null				; Line-F emulator
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Spurious exception
	dc.l   INT_Null				; IRQ level 1
	dc.l   INT_Null				; IRQ level 2
	dc.l   INT_Null				; IRQ level 3
	dc.l   INT_HInterrupt		; IRQ level 4 (horizontal retrace interrupt)
	dc.l   INT_Null  			; IRQ level 5
	dc.l   INT_VInterrupt		; IRQ level 6 (vertical retrace interrupt)
	dc.l   INT_Null				; IRQ level 7
	dc.l   INT_Null				; TRAP #00 exception
	dc.l   INT_Null				; TRAP #01 exception
	dc.l   INT_Null				; TRAP #02 exception
	dc.l   INT_Null				; TRAP #03 exception
	dc.l   INT_Null				; TRAP #04 exception
	dc.l   INT_Null				; TRAP #05 exception
	dc.l   INT_Null				; TRAP #06 exception
	dc.l   INT_Null				; TRAP #07 exception
	dc.l   INT_Null				; TRAP #08 exception
	dc.l   INT_Null				; TRAP #09 exception
	dc.l   INT_Null				; TRAP #10 exception
	dc.l   INT_Null				; TRAP #11 exception
	dc.l   INT_Null				; TRAP #12 exception
	dc.l   INT_Null				; TRAP #13 exception
	dc.l   INT_Null				; TRAP #14 exception
	dc.l   INT_Null				; TRAP #15 exception
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	dc.l   INT_Null				; Unused (reserved)
	
;**********************************************************************
; SEGA MEGA DRIVE ROM HEADER
;**********************************************************************
; A structure that specifies some metadata about the ROM, like
; its name, author, version number, release date, region,
; and any special peripherals used.
; Note that the Mega Drive console itself doesn't read any of
; this, it's more a convenience for the programmer, but
; most emulators will read the title and region.
;**********************************************************************
; If your emulator doesn't show the correct ROM name, then this
; table is in the wrong place or in the wrong format.
;**********************************************************************
; It's not required for execution on hardware (still unconfirmed),
; but emulators that support SVP emulation use part of these fields
; to detect Virtua Racing and thus start the emulated DSP:
;
; - MiSTer relies on the the Version number field to be GM MK-1229
; - KEGA uses the notes field, originally in Shift-JIS.
;
; If you want emulators to support this ROM, please don't change these
; (unless these are updated to give developers the option to enable 
; SVP support manually.)
;
;**********************************************************************
	dc.b "SEGA MEGA DRIVE "                                 ; Console name
	dc.b "Javi Taiyou     "                                 ; Copyright holder and release date
	dc.b "SVP DRAW TEST                                   " ; Domestic name
	dc.b "SVP DRAW TEST                                   " ; International name
	dc.b "GM MK-1229 -00"                                   ; Version number - need to use the original from Virtua Racing for compatibility with MiSTer
	dc.w 0x0000                                             ; Checksum
	dc.b "J               "                                 ; I/O support
	dc.l ROM_Start                                          ; Start address of ROM
	dc.l ROM_End-1                                          ; End address of ROM
	dc.l 0x00FF0000                                         ; Start address of RAM
	dc.l 0x00FF0000+0x0000FFFF                              ; End address of RAM
	dc.l 0x00000000                                         ; SRAM enabled
	dc.l 0x00000000                                         ; Unused
	dc.l 0x00000000                                         ; Start address of SRAM
	dc.l 0x00000000                                         ; End address of SRAM
	dc.l 0x00000000                                         ; Unused
	dc.l 0x00000000                                         ; Unused
	dc.l 0x53560000											; Security check to pass SVP boot-up process
	dc.w 0x2000
	dc.w 0x0400												; Code entry point at 0400
	dc.l 0x20202020
	dc.l 0x20202020
	dc.l 0x20202020
	dc.l 0x20202020
	dc.l 0x20202020
	dc.l 0x20202020
	dc.l 0x20202020
	dc.l 0x20202020
	dc.b "  E             "                                 ; Country codes