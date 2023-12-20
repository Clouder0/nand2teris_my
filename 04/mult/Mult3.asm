@R2
M=0  // initialize result
@temp
M=0 // initialize temp


// goto SKIP_SWAP if R1 - R0 <= 0
@R1
D=M
@R0
D=D-M // now D = R1 - R0
@SKIP_SWAP
D;JLE // goto SKIP_SWAP if R1 - R0 <= 0


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


(SKIP_SWAP)

// temp = R0
@R0
D=M
@temp
M=D

// bitmask = 1
@bitmask
M=1


(LOOP)

// goto END if R1 - bitmask < 0
@R1
D=M  // load R1 value into D register
@bitmask
D=D-M // R1 - bitmask
@END
D;JLT // goto END if R1 - bitmask < 0

// goto SKIP_ADD if R1 & bitmask = 0
@R1
D=M
@bitmask
D=D&M // R1 & bitmask
@SKIP_ADD
D;JEQ // goto SKIP_ADD if R1 & bitmask = 0

// R2 = R2 + temp
@temp
D=M
@R2
M=D+M

(SKIP_ADD)

// temp = temp + temp
@temp
D=M
M=D+M

// bitmask = bitmask + bitmask
@bitmask
D=M
M=D+M

// goto LOOP
@LOOP
0;JMP


(END)
@END
0;JMP  // infinite loop