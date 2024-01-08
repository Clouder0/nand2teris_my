from __future__ import annotations
from abc import ABC, abstractmethod
from collections import defaultdict
from dataclasses import dataclass, field
import pathlib
from typing import ClassVar, Type


@dataclass
class Context:
    filename: str | None = None
    function_name: str = ""
    label_counter: int = 0
    function_ret_counter: dict[str, int] = field(
        default_factory=lambda: defaultdict(int)
    )


class ICommand:
    name: ClassVar[str]

    def __init__(self, ctx: Context):
        self.ctx = ctx

    @classmethod
    def getName(cls) -> str:
        if cls.name is None:
            raise NotImplementedError("Subclasses must define name")
        return cls.name

    @classmethod
    def parse(cls, ctx: Context, args) -> ICommand:
        return cls(ctx, *args)


class C_PUSH(ICommand):
    name = "push"

    def __init__(self, ctx, segment: str, index: int):
        super().__init__(ctx)
        self.segment = segment
        self.index = index

    @classmethod
    def parse(cls, ctx: Context, args):
        return cls(ctx, segment=args[0], index=int(args[1]))


class C_POP(ICommand):
    name = "pop"

    def __init__(self, ctx, segment: str, index: int):
        super().__init__(ctx)
        self.segment = segment
        self.index = index

    @classmethod
    def parse(cls, ctx, args):
        return cls(ctx, segment=args[0], index=int(args[1]))


class C_ADD(ICommand):
    name = "add"


class C_SUB(ICommand):
    name = "sub"


class C_NEG(ICommand):
    name = "neg"


class C_EQ(ICommand):
    name = "eq"


class C_GT(ICommand):
    name = "gt"


class C_LT(ICommand):
    name = "lt"


class C_AND(ICommand):
    name = "and"


class C_OR(ICommand):
    name = "or"


class C_NOT(ICommand):
    name = "not"


class C_LABEL(ICommand):
    name = "label"

    def __init__(self, ctx, label: str):
        super().__init__(ctx)
        self.label = label

    @classmethod
    def parse(cls, ctx, args):
        return cls(ctx, label=args[0])


class C_GOTO(ICommand):
    name = "goto"

    def __init__(self, ctx, label: str):
        super().__init__(ctx)
        self.label = label

    @classmethod
    def parse(cls, ctx, args):
        return cls(ctx, label=args[0])


class C_IF_GOTO(ICommand):
    name = "if-goto"

    def __init__(self, ctx, label: str):
        super().__init__(ctx)
        self.label = label

    @classmethod
    def parse(cls, ctx, args):
        return cls(ctx, label=args[0])


class C_FUNCTION(ICommand):
    name = "function"

    def __init__(self, ctx, function_name: str, num_locals: int):
        super().__init__(ctx)
        self.function_name = function_name
        self.num_locals = num_locals

    @classmethod
    def parse(cls, ctx, args):
        return cls(ctx, function_name=args[0], num_locals=int(args[1]))


class C_CALL(ICommand):
    name = "call"

    def __init__(self, ctx, function_name: str, num_args: int):
        super().__init__(ctx)
        self.function_name = function_name
        self.num_args = num_args

    @classmethod
    def parse(cls, ctx, args):
        return cls(ctx, function_name=args[0], num_args=int(args[1]))


class C_RETURN(ICommand):
    name = "return"


cmd_list: list[Type[ICommand]] = [
    C_PUSH,
    C_POP,
    C_ADD,
    C_SUB,
    C_NEG,
    C_EQ,
    C_GT,
    C_LT,
    C_AND,
    C_OR,
    C_NOT,
    C_LABEL,
    C_GOTO,
    C_IF_GOTO,
    C_FUNCTION,
    C_CALL,
    C_RETURN,
]
