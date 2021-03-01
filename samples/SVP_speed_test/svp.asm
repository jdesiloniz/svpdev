; ***********************
; * SVP access code
; ***********************

regXST          equ 0x00A15000
regXSTState     equ 0x00A15004
regXSTState_L   equ 0x00A15005
regInterrupt    equ 0x00A15006

dramTestResult       equ 0x00300000
dramTestResultAcc    equ 0x00300002

CheckSVPTestResult:
    ; d1 = number of retries before giving up
    ; d2 = number of test to perform
    ; returns: d4 - result of current test: 0xFFFF failure, 0xFFAA success
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

IsSVPDone:
    cmp.w #0x0001, (ram_svp_done_flag)
    beq @AlreadyDone
    move.b regXSTState_L, d2
    btst #0x00, d2      ; is bit 0 in XST_State on? (that means the SVP is over doing its thing)
    beq @NopeDone       ; bit 0 is off

@AlreadyDone
    move.w #0x1010, d4
    move.w #0x0001, (ram_svp_done_flag)
    rts
@NopeDone
    move.w #0xFFFF, d4
    rts