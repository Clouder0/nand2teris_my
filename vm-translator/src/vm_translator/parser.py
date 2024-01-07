from typing import Type
from vm_translator import model


def parse(ctx: model.Context, source: str):
    command_map: dict[str, Type[model.ICommand]] = {}
    for x in model.cmd_list:
        command_map[x.getName()] = x

    print(source)
    lines = source.splitlines()
    output: list[model.ICommand] = []

    for line in lines:
        print(line)
        comment_start = line.find("//")
        if comment_start != -1:
            line = line[:comment_start]
        words = line.strip().lower().split()
        if len(words) == 0:
            continue
        print(words)
        command_name = words[0]
        args = words[1:]
        if command_name not in command_map:
            raise NotImplementedError(f"Command {command_name} not implemented")
        output.append(command_map[command_name].parse(ctx, args))
    return output
