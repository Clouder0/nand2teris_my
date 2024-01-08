@256
D=A;
@SP
M=D;
@sys.init
0;JMP
(sys.init)
D=0
@4000
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@3
D=A;
@0
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
@5000
D=A;

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
@sys.main$ret.0
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

@ARG
D=M
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
@sys.main
0; JMP
(sys.main$ret.0)
@5
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
(sys.init$loop)
@sys.init$loop
0;JMP
(sys.main)
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

@SP
A=M;
M=D;
@SP
M=M+1;

@4001
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@3
D=A;
@0
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
@5001
D=A;

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
@200
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@1
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
@40
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@2
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
@6
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@3
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
@123
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@sys.add12$ret.0
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

@ARG
D=M
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
@sys.add12
0; JMP
(sys.add12$ret.0)
@5
D=A;
@0
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

@2
D=A;

@LCL
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@3
D=A;

@LCL
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@4
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
(sys.add12)
D=0
@4002
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@3
D=A;
@0
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
@5002
D=A;

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

@12
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
