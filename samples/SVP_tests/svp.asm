; ***********************
; * SVP access code
; ***********************

regXST          equ 0x00A15000
regXSTState     equ 0x00A15004
regXSTState_L   equ 0x00A15005
regInterrupt    equ 0x00A15006

CheckSVPTestResult:
    ; d1 = number of retries before giving up
    ; d2 = number of test to perform
    ; returns: d4 - result of current test: 0xFFFF failure, 0xFFAA success
    ;move.w #0xa, regInterrupt
    ;move.w #0x0, regInterrupt
    move.w d2, regXST
    move.w #0xFFFF, d0
    @CheckLoop:
    subq.w #1, d0
    beq @FinishError
    move.b regXSTState_L, d2
    btst #0x00, d2      ; is bit 0 in XST_State on? (that means the SVP is over doing its thing)
    beq @CheckLoop      ; bit 0 is off
    
    @FinishSuccess:
        move.w regXST, d4
        rts
    @FinishError:
        move.w #0xFFFF, d4  ; copy failed result and return
        rts