@256
D=A
@SP
M=D
@sys.init$ret.0
D=A
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

@THAT
D=M
@SP
A=M;
M=D;
@SP
M=M+1;

@5
D=A;

@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@sys.init
0; JMP
(sys.init$ret.0)

(main.fibonacci)
D=0
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
@Main_internal_1
D;JLT
D=0;
@Main_internal_0
0;JMP
(Main_internal_1)
D=-1;
(Main_internal_0)

@SP
A=M;
M=D;
@SP
M=M+1;


@SP
M=M-1;
A=M;
D=M;

@main.fibonacci$n_lt_2
D;JNE
@main.fibonacci$n_ge_2
0;JMP
(main.fibonacci$n_lt_2)
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

@R15
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
@R15
D=M
@R13
A=M
M=D
@R14
A=M
0; JMP
(main.fibonacci$n_ge_2)
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


@main.fibonacci$ret.0
D=A
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

@THAT
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
(main.fibonacci$ret.0)
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


@main.fibonacci$ret.1
D=A
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

@THAT
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
(main.fibonacci$ret.1)
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

@R15
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
@R15
D=M
@R13
A=M
M=D
@R14
A=M
0; JMP
(sys.init)
D=0
@4
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@main.fibonacci$ret.2
D=A
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

@THAT
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
(main.fibonacci$ret.2)
(sys.init$end)
@sys.init$end
0;JMP
