
// L1: push constant 7
@7
D=A
@0
A=M
M=D
@0
M=M+1

// L2: push constant 8
@8
D=A
@0
A=M
M=D
@0
M=M+1

// L3: add
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

