from abc import abstractmethod
from vm_translator import model


class ICmdImpl(model.ICommand):
    @abstractmethod
    def translate(self) -> str:
        pass


class Helper:
    def __init__(self, ctx: model.Context):
        self.ctx = ctx

    def gen_label(self):
        label = f"{self.ctx.filename}_internal_{self.ctx.label_counter}"
        self.ctx.label_counter = self.ctx.label_counter + 1
        return label

    def cadd(self, a: int, b: int):
        return f"@{a}\nD=A;\n@{b}\nD=D+A;\n"

    def constval(self, x: int):
        return f"@{x}\nD=A;\n"

    def seekBaseAddr(self, base_addr: int, index: int) -> str:
        return f"{self.cadd(base_addr,index)}\nA=D;\n"

    def seekBasePointer(self, base_addr_pointer: str, index: int) -> str:
        return f"{self.constval(index)}\n@{base_addr_pointer}\nA=M+D;\n"

    seg_symbol_map = {"local": "LCL", "argument": "ARG", "this": "THIS", "that": "THAT"}

    def seek(self, segment: str, index: int) -> str:
        if segment in self.seg_symbol_map.keys():
            return self.seekBasePointer(self.seg_symbol_map[segment], index)
        if segment == "pointer":
            return self.seekBaseAddr(3, index)
        if segment == "temp":
            return self.seekBaseAddr(5, index)
        if segment == "static":
            return f"@{self.ctx.filename}.{index}\n"
        raise NotImplementedError(f"Segment {segment} not implemented")

    def stpop(self):
        return f"@SP\nM=M-1;\nA=M;\nD=M;\n"

    def stpush(self):
        return f"@SP\nA=M;\nM=D;\n@SP\nM=M+1;\n"

    def opt_two_args(self, op: str):
        return f"""{self.stpop()}
@R13
M=D;
{self.stpop()}
@R13
{op}
{self.stpush()}

"""

    def opt_logical(self, comp: str):
        label_end = self.gen_label()
        label_true = self.gen_label()
        return self.opt_two_args(
            f"""D=D-M;
@{label_true}
D;{comp}
D=0;
@{label_end}
0;JMP
({label_true})
D=-1;
({label_end})
"""
        )


class Translator:
    def __init__(self, ctx: model.Context):
        self.ctx = ctx
        self.helper = Helper(ctx)

    def translate(self, cmd: model.ICommand) -> str:
        return getattr(self, f"translate_{cmd.getName().replace('-','_')}")(cmd)

    def bootstrap(self) -> str:
        return f"""@256
D=A
@SP
M=D
{self.translate_call(model.C_CALL(self.ctx, "sys.init", 0))}
"""

    def translate_push(self, cmd: model.C_PUSH) -> str:
        if cmd.segment == "constant":
            return f"{self.helper.constval(cmd.index)}\n{self.helper.stpush()}\n"
        return f"{self.helper.seek(cmd.segment, cmd.index)}\nD=M;\n{self.helper.stpush()}\n"

    def translate_pop(self, cmd: model.C_POP) -> str:
        return f"{self.helper.seek(cmd.segment, cmd.index)}\nD=A;\n@R13\nM=D;\n{self.helper.stpop()}\n@R13\nA=M;\nM=D;\n"

    def translate_add(self, cmd: model.C_ADD) -> str:
        return self.helper.opt_two_args("D=D+M;\n")

    def translate_sub(self, cmd: model.C_SUB) -> str:
        return self.helper.opt_two_args("D=D-M;\n")

    def translate_neg(self, cmd: model.C_NEG) -> str:
        return f"{self.helper.stpop()}\nD=-D;\n{self.helper.stpush()}\n"

    def translate_eq(self, cmd: model.C_EQ) -> str:
        return self.helper.opt_logical("JEQ")

    def translate_gt(self, cmd: model.C_GT) -> str:
        return self.helper.opt_logical("JGT")

    def translate_lt(self, cmd: model.C_LT) -> str:
        return self.helper.opt_logical("JLT")

    def translate_and(self, cmd: model.C_AND) -> str:
        return self.helper.opt_two_args("D=D&M;\n")

    def translate_or(self, cmd: model.C_OR) -> str:
        return self.helper.opt_two_args("D=D|M;\n")

    def translate_not(self, cmd: model.C_NOT) -> str:
        return f"{self.helper.stpop()}\nD=!D;\n{self.helper.stpush()}\n"

    def translate_label(self, cmd: model.C_LABEL) -> str:
        return f"({self.ctx.function_name}${cmd.label})\n"

    def translate_goto(self, cmd: model.C_GOTO) -> str:
        return f"@{self.ctx.function_name}${cmd.label}\n0;JMP\n"

    def translate_if_goto(self, cmd: model.C_IF_GOTO) -> str:
        return f"{self.helper.stpop()}\n@{self.ctx.function_name}${cmd.label}\nD;JNE\n"

    def translate_function(self, cmd: model.C_FUNCTION) -> str:
        output = f"({cmd.function_name})\nD=0\n"
        for _ in range(cmd.num_locals):
            output += f"{self.helper.stpush()}\n"
        return output

    def translate_call(self, cmd: model.C_CALL) -> str:
        ret_id = self.ctx.function_ret_counter[cmd.function_name]
        self.ctx.function_ret_counter[cmd.function_name] = ret_id + 1
        return f"""@{cmd.function_name}$ret.{ret_id}
D=A
{self.helper.stpush()}
@LCL
D=M
{self.helper.stpush()}
@ARG
D=M
{self.helper.stpush()}
@THIS
D=M
{self.helper.stpush()}
@THAT
D=M
{self.helper.stpush()}
{self.helper.constval(5 + cmd.num_args)}
@SP
D=M-D
@ARG
M=D
@SP
D=M
@LCL
M=D
@{cmd.function_name}
0; JMP
({cmd.function_name}$ret.{ret_id})
"""

    def translate_return(self, cmd: model.C_RETURN) -> str:
        return f"""
{self.helper.stpop()}
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
{self.helper.stpop()}
@THAT
M=D
{self.helper.stpop()}
@THIS
M=D
{self.helper.stpop()}
@ARG
M=D
{self.helper.stpop()}
@LCL
M=D
{self.helper.stpop()}
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
"""
