(Sys.sys.init)
D=0
@4
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@Sys.main.fibonacci$ret.0
@SP
A=M;
M=D;
@SP
M=M+1;

@LCL
D=M
@SP
A=M;
M=D;
@SP
M=M+1;

@ARG
D=M
@SP
A=M;
M=D;
@SP
M=M+1;

@THIS
D=M
@SP
A=M;
M=D;
@SP
M=M+1;

@ARG
D=M
@SP
A=M;
M=D;
@SP
M=M+1;

@6
D=A;

@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@main.fibonacci
0; JMP
(Sys.main.fibonacci$ret.0)
(Sys.sys.init$end)
@Sys.sys.init$end
0;JMP
