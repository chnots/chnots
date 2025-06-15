import re
import typing
import os

type_map = {}


class RustStruct:
    struct_name: str
    fields: typing.Dict[str, str]  # field_name, field_type

    def __init__(self, struct_name: str = "", fields: typing.Dict[str, str] = None):
        self.struct_name = struct_name
        self.fields = fields if fields is not None else {}

    def __repr__(self):
        fields_str = (
            "{" + ", ".join([f"'{k}': '{v}'" for k, v in self.fields.items()]) + "}"
        )
        return f"RustStruct(struct_name='{self.struct_name}', fields={fields_str})"


def parse_rust_struct_block(struct_block_code: str) -> RustStruct:
    lines = struct_block_code.splitlines()
    rust_struct = RustStruct()
    in_struct = False

    struct_decl_pattern = re.compile(r"pub(?:\(crate\))?\s+struct\s+(\w+)\s*{")

    field_name_pattern = re.compile(
        r"pub(?:\(crate\))?\s+(\w+):\s*([a-zA-Z0-9_:<>,]+),?"
    )

    for line in lines:
        line = line.strip()

        if not in_struct:
            struct_match = struct_decl_pattern.match(line)
            if struct_match:
                rust_struct.struct_name = struct_match.group(1)
                in_struct = True
                continue
        else:
            if line == "}":
                if rust_struct.struct_name:
                    return rust_struct
                else:
                    return None

            if line.startswith("#["):
                continue

            field_match = field_name_pattern.match(line)
            if field_match:
                field_name = field_match.group(1)
                field_type = field_match.group(2)
                rust_struct.fields[field_name] = field_type
            elif line.startswith("pub"):
                parts = re.split(r":\s*", line, 1)
                if len(parts) == 2:
                    name_part = parts[0]
                    type_part = parts[1].strip()

                    name_match = re.search(r"pub(?:\(crate\))?\s+(\w+)", name_part)
                    if name_match:
                        field_name = name_match.group(1)
                        if type_part.endswith(","):
                            field_type = type_part[:-1].strip()
                        else:
                            field_type = type_part.strip()
                        rust_struct.fields[field_name] = field_type

    return rust_struct


def parse_rust_file_structs(file_path: str) -> typing.List[RustStruct]:
    if not os.path.exists(file_path):
        print(f"错误: 文件不存在 - {file_path}")
        return []

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

    parsed_structs = []

    struct_blocks = []
    lines = content.splitlines()
    in_struct_block = False
    brace_level = 0
    current_struct_lines = []

    for line_num, line in enumerate(lines):
        stripped_line = line.strip()

        if not in_struct_block and re.match(
            r"pub(?:\(crate\))?\s+struct\s+\w+\s*{", stripped_line
        ):
            in_struct_block = True
            brace_level = 0
            current_struct_lines = []

        if in_struct_block:
            current_struct_lines.append(line)
            brace_level += stripped_line.count("{")
            brace_level -= stripped_line.count("}")

            if brace_level == 0 and "}" in stripped_line:
                struct_blocks.append("\n".join(current_struct_lines))
                in_struct_block = False
                current_struct_lines = []

    for block in struct_blocks:
        parsed_struct = parse_rust_struct_block(block)
        if parsed_struct and parsed_struct.struct_name:
            parsed_structs.append(parsed_struct)

    return parsed_structs


def work_and_do(directory_path):
    found_files = []
    try:
        for root, dirs, files in os.walk(directory_path, topdown=False):
            for name in files:
                fullpath = os.path.join(root, name)
                if name == "dto.rs":
                    print(parse_rust_file_structs(fullpath))
    except FileNotFoundError:
        print(f"错误：目录 '{directory_path}' 不存在。")
    except Exception as e:
        print(f"错误：{e}")
    return found_files


work_and_do("../server/src")
