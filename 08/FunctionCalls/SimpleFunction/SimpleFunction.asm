(simplefunction.test)
D=0
@SP
A=M;
M=D;
@SP
M=M+1;

@SP
A=M;
M=D;
@SP
M=M+1;

@0
D=A;

@LCL
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@1
D=A;

@LCL
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@SP
M=M-1;
A=M;
D=M;

@R13
M=D;
@SP
M=M-1;
A=M;
D=M;

@R13
D=D+M;

@SP
A=M;
M=D;
@SP
M=M+1;


@SP
M=M-1;
A=M;
D=M;

D=!D;
@SP
A=M;
M=D;
@SP
M=M+1;

@0
D=A;

@ARG
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@SP
M=M-1;
A=M;
D=M;

@R13
M=D;
@SP
M=M-1;
A=M;
D=M;

@R13
D=D+M;

@SP
A=M;
M=D;
@SP
M=M+1;


@1
D=A;

@ARG
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@SP
M=M-1;
A=M;
D=M;

@R13
M=D;
@SP
M=M-1;
A=M;
D=M;

@R13
D=D-M;

@SP
A=M;
M=D;
@SP
M=M+1;



@SP
M=M-1;
A=M;
D=M;

@ARG
A=M
M=D
@ARG
D=M
@R13
M=D
@LCL
D=M
@SP
M=D
@SP
M=M-1;
A=M;
D=M;

@THAT
M=D
@SP
M=M-1;
A=M;
D=M;

@THIS
M=D
@SP
M=M-1;
A=M;
D=M;

@ARG
M=D
@SP
M=M-1;
A=M;
D=M;

@LCL
M=D
@SP
M=M-1;
A=M;
D=M;

@R14
M=D
@R13
D=M
@SP
M=D+1
@R14
A=M
0; JMP
