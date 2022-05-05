
// L1: push constant 111
@111
D=A
@0
A=M
M=D
@0
M=M+1

// L2: push constant 333
@333
D=A
@0
A=M
M=D
@0
M=M+1

// L3: push constant 888
@888
D=A
@0
A=M
M=D
@0
M=M+1

// L4: pop static 8
@0
M=M-1
A=M
D=M
@13
M=D

// L5: pop static 3
@0
M=M-1
A=M
D=M
@8
M=D

// L6: pop static 1
@0
M=M-1
A=M
D=M
@6
M=D

// L7: push static 3
@8
D=M
@0
A=M
M=D
@0
M=M+1

// L8: push static 1
@6
D=M
@0
A=M
M=D
@0
M=M+1

// L9: sub
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

// L10: push static 8
@13
D=M
@0
A=M
M=D
@0
M=M+1

// L11: add
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

