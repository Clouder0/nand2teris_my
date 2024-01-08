import click
from vm_translator import hack, parser
from vm_translator import model
from pathlib import Path


def compile(input_path: Path):
    ctx = model.Context()
    translator = hack.Translator(ctx)
    output = translator.bootstrap()
    if input_path.is_dir():
        # find all files under such dir
        files = input_path.glob("*.vm")
        for file in files:
            output += compileFile(ctx, file, translator)
        with open(input_path / f"all.asm", "w") as f:
            f.write(output)
    else:
        output += compileFile(ctx, input_path, translator)
        with open(input_path.with_suffix(".asm"), "w") as f:
            f.write(output)


def compileFile(ctx: model.Context, input_path: Path, translator):
    with open(input_path, "r") as f:
        ctx.filename = input_path.stem
        cmds = parser.parse(ctx, f.read())
        output = ""
        for cmd in cmds:
            if cmd.getName() == "function":
                ctx.function_name = cmd.function_name
            output += translator.translate(cmd)
        return output
