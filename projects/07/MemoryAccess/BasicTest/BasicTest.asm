// L1: push constant 10
@10
D=A
@0
A=M
M=D
@0
M=M+1

// L2: pop local 0
M=M-1
A=M
D=M
@R13
M=D
@1
D=M
@0
D=D+A
@R14
M=D
@R13
D=M
@R14
A=M
M=D

// L3: push constant 21
@21
D=A
@0
A=M
M=D
@0
M=M+1

// L4: push constant 22
@22
D=A
@0
A=M
M=D
@0
M=M+1

// L5: pop argument 2
M=M-1
A=M
D=M
@R13
M=D
@2
D=M
D=D+A
@R14
M=D
@R13
D=M
@R14
A=M
M=D

// L6: pop argument 1
@0
M=M-1
A=M
D=M
@R13
M=D
@2
D=M
@1
D=D+A
@R14
M=D
@R13
D=M
@R14
A=M
M=D

// L7: push constant 36
@36
D=A
@0
A=M
M=D
@0
M=M+1

// L8: pop this 6
M=M-1
A=M
D=M
@R13
M=D
@3
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

// L9: push constant 42
@42
D=A
@0
A=M
M=D
@0
M=M+1

// L10: push constant 45
@45
D=A
@0
A=M
M=D
@0
M=M+1

// L11: pop that 5
M=M-1
A=M
D=M
@R13
M=D
@4
D=M
@5
D=D+A
@R14
M=D
@R13
D=M
@R14
A=M
M=D

// L12: pop that 2
@0
M=M-1
A=M
D=M
@R13
M=D
@4
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

// L13: push constant 510
@510
D=A
@0
A=M
M=D
@0
M=M+1

// L14: pop temp 6
M=M-1
A=M
D=M
@11
M=D

// L15: push local 0
@1
D=M
@0
D=D+A
A=D
D=M
@0
A=M
M=D
@0
M=M+1

// L16: push that 5
@4
D=M
@5
D=D+A
A=D
D=M
@0
A=M
M=D
@0
M=M+1

// L17: add
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

// L18: push argument 1
@2
D=M
@1
D=D+A
A=D
D=M
@0
A=M
M=D
@0
M=M+1

// L19: sub
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

// L20: push this 6
@3
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

// L21: push this 6
@3
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

// L22: add
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

// L23: sub
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

// L24: push temp 6
@11
D=M
@0
A=M
M=D
@0
M=M+1

// L25: add
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
