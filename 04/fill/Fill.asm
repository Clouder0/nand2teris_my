// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen
// by writing 'black' in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen by writing
// 'white' in every pixel;
// the screen should remain fully clear as long as no key is pressed.

//// Replace this comment with your code.

(MAIN)
@KBD
D=M
@FLUSH_BLACK
D; JNE // jump to FLUSH_BLACK if keycode != 0

(FLUSH_WHITE)
@R0
M=0
@FLUSH
0; JMP

(FLUSH_BLACK)
@R0
M=-1
@FLUSH
0; JMP

// func FLUSH
(FLUSH)
@i
M=0
(LOOP)
@8192
D=A
@i
D=M-D
@LOOP_END
D; JGE  // i >= 8192 -> goto LOOP_END
@i
D=M
@SCREEN
D=A+D // jump to SCREEN[i]
@R1
M=D // save addr of SCREEN[i] to R1
@R0
D=M
@R1
A=M // jump back to SCREEN[i]
M=D // SCREEN[i] = R1
@i
M=M+1
@LOOP
0; JMP

(LOOP_END)

@MAIN
0; JMP
