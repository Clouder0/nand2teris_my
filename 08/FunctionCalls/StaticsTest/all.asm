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

(class2.set)
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

@Class2.0

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

@ARG
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@Class2.1

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
(class2.get)
D=0
@Class2.0

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@Class2.1

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
(class1.set)
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

@Class1.0

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

@ARG
A=M+D;

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@Class1.1

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
(class1.get)
D=0
@Class1.0

D=M;
@SP
A=M;
M=D;
@SP
M=M+1;

@Class1.1

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
@6
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@8
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@class1.set$ret.0
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

@7
D=A;

@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@class1.set
0; JMP
(class1.set$ret.0)
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
@23
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@15
D=A;

@SP
A=M;
M=D;
@SP
M=M+1;

@class2.set$ret.0
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

@7
D=A;

@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@class2.set
0; JMP
(class2.set$ret.0)
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
@class1.get$ret.0
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
@class1.get
0; JMP
(class1.get$ret.0)
@class2.get$ret.0
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
@class2.get
0; JMP
(class2.get$ret.0)
(sys.init$end)
@sys.init$end
0;JMP
