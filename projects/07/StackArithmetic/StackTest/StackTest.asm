
// L1: push constant 17
@17
D=A
@0
A=M
M=D
@0
M=M+1

// L2: push constant 17
@17
D=A
@0
A=M
M=D
@0
M=M+1

// L3: eq
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL1
D;JEQ
D=0
@LOW_LEVEL_LABEL2
0;JMP
(LOW_LEVEL_LABEL1)
D=-1
(LOW_LEVEL_LABEL2)
@0
A=M
M=D
@0
M=M+1

// L4: push constant 17
@17
D=A
@0
A=M
M=D
@0
M=M+1

// L5: push constant 16
@16
D=A
@0
A=M
M=D
@0
M=M+1

// L6: eq
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL3
D;JEQ
D=0
@LOW_LEVEL_LABEL4
0;JMP
(LOW_LEVEL_LABEL3)
D=-1
(LOW_LEVEL_LABEL4)
@0
A=M
M=D
@0
M=M+1

// L7: push constant 16
@16
D=A
@0
A=M
M=D
@0
M=M+1

// L8: push constant 17
@17
D=A
@0
A=M
M=D
@0
M=M+1

// L9: eq
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL5
D;JEQ
D=0
@LOW_LEVEL_LABEL6
0;JMP
(LOW_LEVEL_LABEL5)
D=-1
(LOW_LEVEL_LABEL6)
@0
A=M
M=D
@0
M=M+1

// L10: push constant 892
@892
D=A
@0
A=M
M=D
@0
M=M+1

// L11: push constant 891
@891
D=A
@0
A=M
M=D
@0
M=M+1

// L12: lt
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL7
D;JLT
D=0
@LOW_LEVEL_LABEL8
0;JMP
(LOW_LEVEL_LABEL7)
D=-1
(LOW_LEVEL_LABEL8)
@0
A=M
M=D
@0
M=M+1

// L13: push constant 891
@891
D=A
@0
A=M
M=D
@0
M=M+1

// L14: push constant 892
@892
D=A
@0
A=M
M=D
@0
M=M+1

// L15: lt
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL9
D;JLT
D=0
@LOW_LEVEL_LABEL10
0;JMP
(LOW_LEVEL_LABEL9)
D=-1
(LOW_LEVEL_LABEL10)
@0
A=M
M=D
@0
M=M+1

// L16: push constant 891
@891
D=A
@0
A=M
M=D
@0
M=M+1

// L17: push constant 891
@891
D=A
@0
A=M
M=D
@0
M=M+1

// L18: lt
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL11
D;JLT
D=0
@LOW_LEVEL_LABEL12
0;JMP
(LOW_LEVEL_LABEL11)
D=-1
(LOW_LEVEL_LABEL12)
@0
A=M
M=D
@0
M=M+1

// L19: push constant 32767
@32767
D=A
@0
A=M
M=D
@0
M=M+1

// L20: push constant 32766
@32766
D=A
@0
A=M
M=D
@0
M=M+1

// L21: gt
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL13
D;JGT
D=0
@LOW_LEVEL_LABEL14
0;JMP
(LOW_LEVEL_LABEL13)
D=-1
(LOW_LEVEL_LABEL14)
@0
A=M
M=D
@0
M=M+1

// L22: push constant 32766
@32766
D=A
@0
A=M
M=D
@0
M=M+1

// L23: push constant 32767
@32767
D=A
@0
A=M
M=D
@0
M=M+1

// L24: gt
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL15
D;JGT
D=0
@LOW_LEVEL_LABEL16
0;JMP
(LOW_LEVEL_LABEL15)
D=-1
(LOW_LEVEL_LABEL16)
@0
A=M
M=D
@0
M=M+1

// L25: push constant 32766
@32766
D=A
@0
A=M
M=D
@0
M=M+1

// L26: push constant 32766
@32766
D=A
@0
A=M
M=D
@0
M=M+1

// L27: gt
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@LOW_LEVEL_LABEL17
D;JGT
D=0
@LOW_LEVEL_LABEL18
0;JMP
(LOW_LEVEL_LABEL17)
D=-1
(LOW_LEVEL_LABEL18)
@0
A=M
M=D
@0
M=M+1

// L28: push constant 57
@57
D=A
@0
A=M
M=D
@0
M=M+1

// L29: push constant 31
@31
D=A
@0
A=M
M=D
@0
M=M+1

// L30: push constant 53
@53
D=A
@0
A=M
M=D
@0
M=M+1

// L31: add
@0
M=M-1
A=M
D=M
@0
M=M-1
A=M
D=D+M
@0
A=M
M=D
@0
M=M+1

// L32: push constant 112
@112
D=A
@0
A=M
M=D
@0
M=M+1

// L33: sub
@0
M=M-1
A=M
D=-M
@0
M=M-1
A=M
D=D+M
@0
A=M
M=D
@0
M=M+1

// L34: neg
@0
M=M-1
A=M
D=-M
@0
A=M
M=D
@0
M=M+1

// L35: and
@0
M=M-1
A=M
D=M
@0
M=M-1
A=M
D=D&M
@0
A=M
M=D
@0
M=M+1

// L36: push constant 82
@82
D=A
@0
A=M
M=D
@0
M=M+1

// L37: or
@0
M=M-1
A=M
D=M
@0
M=M-1
A=M
D=D|M
@0
A=M
M=D
@0
M=M+1

// L38: not
@0
M=M-1
A=M
D=!M
@0
A=M
M=D
@0
M=M+1

