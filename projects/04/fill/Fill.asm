// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.


// Load the value from the keyboard
// If the value is not zero, blacken the screen, goto start
// If the value is zero, whiten the screen, goto start
// Hello world! Cplayground is an online sandbox that makes it easy to try out
// code.

// C PsuedoCode:
//
// void FillScreen()
// {
//     int screen[8192];
// 
//     for (int i = 0; i < 8192; ++i)
//     {
//         screen[i] = 1;
//     }
// }
// 
// void ClearScreen()
// {
//     int screen[8192];
// 
//     for (int i = 0; i < 8192; ++i)
//     {
//         screen[i] = 0;
//     }
// }
// 
// int main()
// {
//     int keyboard = 0;
//     bool screen_filled = false;
// 
//     while (true)
//     {
//         if (keyboard != 0 && !screen_filled)
//         {
//             FillScreen();
//         }
//         else if (keyboard == 0 && screen_filled)
//         {
//             ClearScreen();
//         }
//     }
// 
//     return 0;
// }

(START)
@KBD                        // if (KBD != 0) { GOTO @KBD_ACTIVE }
D=M
@KBD_ACTIVE
D;JNE

@KBD_INACTIVE               // else { GOTO @KBD_INACTIVE; }
0;JMP

(KBD_ACTIVE)
@colour
M=-1
@PAINT_SCREEN
0;JMP

(KBD_INACTIVE)
@colour
M=0

// Paint the screen with our selected colour
(PAINT_SCREEN)
@SCREEN                     // @index = *@KBD
D=A
@index
M=D

(PAINT_LOOP)
@colour                     // RAM[index] = @colour
D=M
@index
A=M
M=D

@index                      // ++index
M=M+1
D=M

@KBD                        // if (@index == *@KBD) { GOTO START; }
D=D-A
@START
D;JEQ

@PAINT_LOOP                 // GOTO @PAINT_LOOP
0;JMP
