// L11: push argument 1
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
// L12: pop pointer 1           
@0
M=M-1
A=M
D=M
@4
M=D
// L14: push constant 0
@0
D=A
@0
A=M
M=D
@0
M=M+1
// L15: pop that 0              
@0
M=M-1
A=M
D=M
@13
M=D
@4
D=M
@0
D=D+A
@14
M=D
@13
D=M
@14
A=M
M=D
// L16: push constant 1
@1
D=A
@0
A=M
M=D
@0
M=M+1
// L17: pop that 1              
@0
M=M-1
A=M
D=M
@13
M=D
@4
D=M
@1
D=D+A
@14
M=D
@13
D=M
@14
A=M
M=D
// L19: push argument 0
@2
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
// L20: push constant 2
@2
D=A
@0
A=M
M=D
@0
M=M+1
// L21: sub
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
// L22: pop argument 0          
@0
M=M-1
A=M
D=M
@13
M=D
@2
D=M
@0
D=D+A
@14
M=D
@13
D=M
@14
A=M
M=D
// L24: label MAIN_LOOP_START
(MAIN_LOOP_START)
// L26: push argument 0
@2
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
// L27: if-goto COMPUTE_ELEMENT 
@0
M=M-1
@COMPUTE_ELEMENT
D;JNE
// L28: goto END_PROGRAM        
@END_PROGRAM
0;JMP
// L30: label COMPUTE_ELEMENT
(COMPUTE_ELEMENT)
// L32: push that 0
@4
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
// L33: push that 1
@4
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
// L34: add
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
// L35: pop that 2              
@0
M=M-1
A=M
D=M
@13
M=D
@4
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
// L37: push pointer 1
@4
D=M
@0
A=M
M=D
@0
M=M+1
// L38: push constant 1
@1
D=A
@0
A=M
M=D
@0
M=M+1
// L39: add
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
// L40: pop pointer 1           
@0
M=M-1
A=M
D=M
@4
M=D
// L42: push argument 0
@2
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
// L43: push constant 1
@1
D=A
@0
A=M
M=D
@0
M=M+1
// L44: sub
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
// L45: pop argument 0          
@0
M=M-1
A=M
D=M
@13
M=D
@2
D=M
@0
D=D+A
@14
M=D
@13
D=M
@14
A=M
M=D
// L47: goto MAIN_LOOP_START
@MAIN_LOOP_START
0;JMP
// L49: label END_PROGRAM
(END_PROGRAM)