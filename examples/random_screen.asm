LDA #$00
STA $00
LDA #$01
STA $01

loop:
    LDA $FE
    STA ($00),Y
    CPY #$FF
    INY
    BNE loop
    
    INX
    CPX #$40
    BNE increase_offset
    
    DRW
    BRK

increase_offset:
    LDA $01
    ADC #$01
    STA $01
    JMP loop