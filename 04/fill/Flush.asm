@R0
M=0  // the variable that indicates what we want to write

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
@LOOP_END
0; JMP