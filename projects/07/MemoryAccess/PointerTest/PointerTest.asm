// L8: push constant 3030
@3030
D=A
@0
A=M
M=D
@0
M=M+1
// L9: pop pointer 0
@0
M=M-1
A=M
D=M
@3
M=D
// L10: push constant 3040
@3040
D=A
@0
A=M
M=D
@0
M=M+1
// L11: pop pointer 1
@0
M=M-1
A=M
D=M
@4
M=D
// L12: push constant 32
@32
D=A
@0
A=M
M=D
@0
M=M+1
// L13: pop this 2
@0
M=M-1
A=M
D=M
@13
M=D
@3
D=M
@2
D=D+A
@14
M=D
@13
D=M
@14
A=M
M=D
// L14: push constant 46
@46
D=A
@0
A=M
M=D
@0
M=M+1
// L15: pop that 6
@0
M=M-1
A=M
D=M
@13
M=D
@4
D=M
@6
D=D+A
@14
M=D
@13
D=M
@14
A=M
M=D
// L16: push pointer 0
@3
D=M
@0
A=M
M=D
@0
M=M+1
// L17: push pointer 1
@4
D=M
@0
A=M
M=D
@0
M=M+1
// L18: add
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
// L19: push this 2
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
// L20: sub
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
// L21: push that 6
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
// L22: add
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
