@pressed
M=0 // init pressed

(MAIN)
@KBD
D=M

@JUDGE_WHITE
D; JEQ // KBD == 0 -> goto JUDGE_WHITE
// else goto JUDGE_BLACK:
(JUDGE_BLACK)
@pressed
D=M
@FLUSH_BLACK
D; JEQ // KBD != 0 && pressed == 0 -> goto  FLUSH_BLACK
@MAIN
0; JMP

(JUDGE_WHITE)
@pressed
D=M
@FLUSH_WHITE
D; JNE // KBD == 0 && pressed != 0 -> goto FLUSH_WHITE
@MAIN
0; JMP

(FLUSH_WHITE)
@pressed
M=0
@R0
M=0
@FLUSH
0; JMP

(FLUSH_BLACK)
@pressed
M=1
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