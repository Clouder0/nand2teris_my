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

@3
D=A;
@1
D=D+A;

A=D;

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

@SP
A=M;
M=D;
@SP
M=M+1;

@0
D=A;

@THAT
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
@1
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@1
D=A;

@THAT
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

@2
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

@SP
M=M-1;
A=M;
D=M;

@$compute_element
D;JNE
@$end
0;JMP
($compute_element)
@0
D=A;

@THAT
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@1
D=A;

@THAT
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


@2
D=A;

@THAT
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
@3
D=A;
@1
D=D+A;

A=D;

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
D=D+M;

@SP
A=M;
M=D;
@SP
M=M+1;


@3
D=A;
@1
D=D+A;

A=D;

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
@$loop
0;JMP
($end)
