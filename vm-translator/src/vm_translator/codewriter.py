from vm_translator import model
from vm_translator.hack import Translator


def generate(ctx: model.Context, cmds: list[model.ICommand]) -> str:
    output = ""
    translator = Translator(ctx)
    for cmd in cmds:
        output += translator.translate(cmd)
    return output
