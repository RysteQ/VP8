LDA #$00
STA $00
LDA #$02
STA $01

loop:
    LDA $FE
    STA ($00),Y
    CPY #$FF
    INY
    BNE loop
    
    INX
    CPX #$04
    BNE increase_offset
    BRK

increase_offset:
    LDA $01
    ADC #$01
    STA $01
    JMP loop