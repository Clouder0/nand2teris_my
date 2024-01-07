import click
from vm_translator import parser, codewriter
from vm_translator import model
from pathlib import Path


def compile(input_path: Path):
    ctx = model.Context(input_path)
    with open(input_path, "r") as f:
        cmds = parser.parse(ctx, f.read())
        click.echo(cmds)
        output = codewriter.generate(ctx, cmds)
        input_path.with_suffix(".asm").write_text(output)
