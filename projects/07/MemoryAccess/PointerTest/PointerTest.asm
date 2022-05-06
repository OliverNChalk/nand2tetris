// L1: push constant 3030
@3030
D=A
@0
A=M
M=D
@0
M=M+1

// L2: pop pointer 0
M=M-1
A=M
D=M
@3
M=D

// L3: push constant 3040
@3040
D=A
@0
A=M
M=D
@0
M=M+1

// L4: pop pointer 1
M=M-1
A=M
D=M
@4
M=D

// L5: push constant 32
@32
D=A
@0
A=M
M=D
@0
M=M+1

// L6: pop this 2
M=M-1
A=M
D=M
@R13
M=D
@3
D=M
@2
D=D+A
@R14
M=D
@R13
D=M
@R14
A=M
M=D

// L7: push constant 46
@46
D=A
@0
A=M
M=D
@0
M=M+1

// L8: pop that 6
M=M-1
A=M
D=M
@R13
M=D
@4
D=M
@6
D=D+A
@R14
M=D
@R13
D=M
@R14
A=M
M=D

// L9: push pointer 0
@3
D=M
@0
A=M
M=D
@0
M=M+1

// L10: push pointer 1
@4
D=M
@0
A=M
M=D
@0
M=M+1

// L11: add
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

// L12: push this 2
@3
D=M
@2
D=D+A
A=D
D=M
@0
A=M
M=D
@0
M=M+1

// L13: sub
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

// L14: push that 6
@4
D=M
@6
D=D+A
A=D
D=M
@0
A=M
M=D
@0
M=M+1

// L15: add
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
