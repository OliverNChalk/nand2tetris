// L7: push constant 111
@111
D=A
@0
A=M
M=D
@0
M=M+1
// L8: push constant 333
@333
D=A
@0
A=M
M=D
@0
M=M+1
// L9: push constant 888
@888
D=A
@0
A=M
M=D
@0
M=M+1
// L10: pop static 8
@0
M=M-1
A=M
D=M
@24
M=D
// L11: pop static 3
@0
M=M-1
A=M
D=M
@19
M=D
// L12: pop static 1
@0
M=M-1
A=M
D=M
@17
M=D
// L13: push static 3
@19
D=M
@0
A=M
M=D
@0
M=M+1
// L14: push static 1
@17
D=M
@0
A=M
M=D
@0
M=M+1
// L15: sub
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
// L16: push static 8
@24
D=M
@0
A=M
M=D
@0
M=M+1
// L17: add
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
