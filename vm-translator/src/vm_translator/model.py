from __future__ import annotations
from abc import ABC, abstractmethod
from dataclasses import dataclass
import pathlib
from typing import ClassVar, Type


@dataclass
class Context:
    filename: pathlib.Path | None = None
    label_counter: int = 0


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
]
