; ***********************
; * SVP access code
; ***********************

regXST          equ 0x00A15000
regXSTState     equ 0x00A15004
regXSTState_L   equ 0x00A15005
regInterrupt    equ 0x00A15006

dramInit        equ 0x00300000

pramStartAddress equ 0xFC00
;pramStartAddress equ 0x0400

InitSVP:
    move.w #0x1001, regXST           ; Wakey wakey, hams with backey
    rts

SVPDumpData:
    ; page_initial_address: address (in SVP word space) to dump data from:

    move (page_initial_address), d0
    move.w d0, regXST           ; Hey SVP, here's the start data, please dump

    move.w #0xFFFF, d0          ; Wait until the SVP confirms everything is in DRAM
    @CheckLoop:
    subq.w #1, d0
    beq @FinishErrorLoadData
    move.b regXSTState_L, d2
    btst #0x00, d2              ; is bit 0 in XST_State on? (that means the SVP is over doing its thing)
    beq @CheckLoop              ; bit 0 is off
    
    @FinishSuccessLoadData:
        rts
    @FinishErrorLoadData:
        rts