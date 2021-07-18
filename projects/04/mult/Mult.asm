// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Strategy for Multiplication:
//
// let a = R0
// let b = R1
//
// If a > b { add a + a b times }
// If b < a { add b + b a times }
//
// Start by determining which number is smaller, this will be our
// max_iterations. Then we will start by doubling the larger number
// and our i counter until i*2 > max_iterations. At this point,
// instead of doubling, we will move to incrementing i by one at a
// time.

// PsuedoCode for Multiplication:
//
//
//  int min(int a, int b)
//  {
//      if (a > b) { return a; }
//      
//      return b;
//  }
//
//  int multiply(int a, int b)
//  {
//      if (a == 0 || b == 0) { return 0; }
//
//      int smaller = min(a, b);
//      int larger = a + b - smaller;
//
//      int i = 1;
//      int result = larger;
//
//      // Double result each round until we exhaust smaller
//      while (i + i <= smaller)
//      {
//          result += result;
//          i += i;
//          
//          printf("%d\n", result);
//      }
//
//      // Increment result by one lot of larger each round until we exhuast smaller
//      while (i < smaller)
//      {
//          result += larger;
//          i += 1;
//
//          printf("%d\n", result);
//      }
//
//      return result;
//  }

// NOTE:
//  This algorithm can worst case run for O(N^2 / 2) where N is number of bits
//  Implementing long multiplication in binary is a little too tedious for me.

// SOURCE:
// Initialize product to zero
@R2
M=0

// If either input is zero, output is zero, goto @END
@R0                 // if R0 == 0 { goto @END; }
D=M
@END
D;JEQ

@R1                 // if R1 == 0 { goto @END; }
D=M
@END
D;JEQ

// Determine which number is smaller
@R0                 // if (R0 - R1) { goto @A_SMALLER } else { goto @B_SMALLER }
D=M
@R1
D=D-M;
@A_SMALLER
D;JLE
@B_SMALLER
0;JMP

// Save a in @smaller, b in @larger
(A_SMALLER)
@R0
D=M
@smaller
M=D

@R1
D=M
@larger
M=D

@INIT_LOOP_DOUBLING
0;JMP

// Save b in @smaller, a in @larger
(B_SMALLER)
@R1
D=M
@smaller
M=D

@R0
D=M
@larger
M=D

// result = larger
// i = 1;
// while (i + i < smaller)
// {
//     result += result;
//     i += i;
// }
(INIT_LOOP_DOUBLING)
@i
M=1
@R2
M=D                 // In all jumps to INIT_LOOP_DOUBLING we have D set to larger

(LOOP_DOUBLING)
@i                  // Jump to LOOP_INC if i > smaller
D=M
D=D+M
@smaller
D=M-D
@LOOP_INC
D;JLE

@R2                 // Double the value of result
D=M
M=D+M

@i                  // Double the value of i
D=M
M=D+M

@LOOP_DOUBLING      // Jump to start of current loop
0;JMP


// Increment i by one and result by larger until i === smaller
// while (i < smaller)
// {
//     result += larger;
//     i += 1;
// }
(LOOP_INC)
@i                  // If i <= smaller, jump to end
D=M
@smaller
D=M-D
@END
D;JLE

@larger             // result += larger
D=M
@R2
M=D+M

@i
M=M+1               // i += 1

@LOOP_INC           // Jump to start of current loop
0;JMP

// Infinite Loop:
(END)
    @END
    0;JMP
