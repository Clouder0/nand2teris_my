import click
from pathlib import Path
from vm_translator import compiler


@click.command()
@click.argument("path", type=click.Path(exists=True))
def main(path: str):
    click.echo(f"Translating {path}")
    compiler.compile(Path(path))
    click.echo("Done")
