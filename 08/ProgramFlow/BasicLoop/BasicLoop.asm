@0
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@0
D=A;

@LCL
A=M+D;

D=A;
@R13
M=D;
@SP
M=M-1;
A=M;
D=M;

@R13
A=M;
M=D;
($loop)
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


@0
D=A;

@LCL
A=M+D;

D=A;
@R13
M=D;
@SP
M=M-1;
A=M;
D=M;

@R13
A=M;
M=D;
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

@1
D=A;

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


@0
D=A;

@ARG
A=M+D;

D=A;
@R13
M=D;
@SP
M=M-1;
A=M;
D=M;

@R13
A=M;
M=D;
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

@$loop
D;JNE
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

