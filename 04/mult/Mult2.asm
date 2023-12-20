// optimized version for mult, swap if R0 > R1

@i
M=0  // initialize variable i
@R2
M=0  // initialize result
@temp
M=0 // initialize temp

@R1
D=M
@R0
D=D-M // now D = R1 - R0
@LOOP
D;JLE // goto LOOP if R1 - R0 <= 0
// swap R0, R1

// temp = R0
@R0
D=M
@temp
M=D

// R0 = R1
@R1
D=M
@R0
M=D

// R1 = temp
@temp
D=M
@R1
M=D


(LOOP)

@R1
D=M  // load R1 value into D register
@i
D=M-D
@END
D;JGE  // check i - R1 >= 0 <=> i >= R1, if true then goto END
@i
M=M+1  // i = i + 1

@R0
D=M  // load R0 value into D register
@R2
M=M+D  // R2 = R2 + R0

@LOOP
0; JMP // go back to loop

(END)
@END
0;JMP  // infinite loop
