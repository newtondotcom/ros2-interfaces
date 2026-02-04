import re
import sys
from dataclasses import dataclass, field as dataclass_field
from glob import glob
from pathlib import Path
from typing import List, Optional, Tuple, Set

# Define Rust reserved keywords
RUST_KEYWORDS = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "try",
    "typeof",
    "unsized",
    "virtual",
    "yield",
}

# Define types that implement the Copy trait
COPY_TRAIT_TYPES = {
    "bool",
    "i8",
    "i16",
    "i32",
    "i64",
    "u8",
    "u16",
    "u32",
    "u64",
    "f32",
    "f64",
    "char",
}


def main(packages_dir: Path, crate_dir: Path, version: str):
    assert crate_dir.is_dir() and crate_dir.exists(), (
        f"Invalid or non-existing crate directory: {crate_dir}"
    )

    ignore_list = [
        "tuw_object_msgs",
        "social_nav_msgs",
        "cras_msgs",  # 6 errors
        "rosbag2_test_msgdefs",  # 6 errors
        "mrpt_msgs",  # 6 errors
        "mrpt_nav_interfaces"
    ]

    # Collect all packages (directories) in the packages_dir
    all_packages = [
        d.stem
        for d in packages_dir.iterdir()
        if d.is_dir() and d.stem not in ignore_list
    ]
    available_packages = set(all_packages)  # Set of all available package names

    # Keep track of package dependencies
    package_dependencies = {}
    processed_packages = []

    src_dir = crate_dir / "src"
    src_dir.mkdir(exist_ok=True)

    for package in all_packages:
        package_path = packages_dir / package
        dependencies, has_msgs_or_srvs = process_package(
            package_path, src_dir, available_packages
        )
        if has_msgs_or_srvs:
            package_dependencies[package] = dependencies
            processed_packages.append(package)
        else:
            print(
                f"Package '{package}' has no .msg or .srv files. Excluding from lib.rs."
            )

    # Clean up directories for unprocessed packages
    cleanup_unprocessed_packages(src_dir, all_packages, processed_packages)

    # Generate lib.rs and Cargo.toml using only processed packages
    generate_lib_rs(src_dir, processed_packages)
    update_cargo_toml(
        crate_dir / "Cargo.toml",
        processed_packages,
        package_dependencies,
        version,
    )


@dataclass
class Constant:
    type: str
    name: str
    value: str


@dataclass
class Field:
    type: str
    name: str
    default_value: Optional[str] = None
    is_array: bool = False
    array_size: Optional[int] = None
    array_upper_bound: Optional[int] = None
    is_bounded_string: bool = False
    string_upper_bound: Optional[int] = None


@dataclass
class MessageSpec:
    constants: List[Constant] = dataclass_field(default_factory=list)
    fields: List[Field] = dataclass_field(default_factory=list)


def parse_msg_file(file_path: str) -> MessageSpec:
    with open(file_path, "r") as f:
        lines = f.readlines()
    return parse_msg_from_lines(lines)


def parse_msg_from_lines(lines: List[str]) -> MessageSpec:
    message_spec = MessageSpec()

    for line in lines:
        line = line.strip()
        # Ignore comments and empty lines
        if not line or line.startswith("#"):
            continue

        # Handle comments at the end of a line
        if "#" in line:
            line = line.split("#", 1)[0].strip()

        # Check for constants (type NAME=value)
        const_match = re.match(r"^(\w[\w/<>]*)\s+([A-Z][A-Z0-9_]*)\s*=\s*(.+)$", line)
        if const_match:
            type_str, name, value = const_match.groups()
            message_spec.constants.append(
                Constant(type=type_str.strip(), name=name.strip(), value=value.strip())
            )
            continue

        # Check for field definitions with optional default values
        field_match = re.match(r"^(\w[\w/<>[\]=]*)\s+(\w+)(?:\s+(.+))?$", line)
        if field_match:
            type_str, name, default_value = field_match.groups()
            field = parse_field(
                type_str.strip(),
                name.strip(),
                default_value.strip() if default_value else None,
            )
            message_spec.fields.append(field)
            continue

        raise ValueError(f"Invalid line in message content: '{line}'")

    return message_spec


def parse_srv_file(file_path: str) -> Tuple[MessageSpec, MessageSpec]:
    with open(file_path, "r") as f:
        lines = f.readlines()

    # Find the line that contains exactly '---'
    separator_index = None
    for i, line in enumerate(lines):
        if line.strip() == "---":
            separator_index = i
            break

    if separator_index is None:
        raise ValueError(
            f"Service file {file_path} does not contain a '---' separator line."
        )

    # Split into request and response parts based on the found separator line
    request_lines = [ll for ll in lines[:separator_index]]
    response_lines = [ll for ll in lines[separator_index + 1 :]]

    # Parse request
    request_spec = parse_msg_from_lines([ll.strip() for ll in request_lines])

    # Parse response
    response_spec = parse_msg_from_lines([ll.strip() for ll in response_lines])

    return request_spec, response_spec


def parse_field(type_str: str, name: str, default_value: Optional[str]) -> Field:
    # Handle arrays
    array_match = re.match(r"^(.+?)\[(.*?)\]$", type_str)
    if array_match:
        base_type, array_spec = array_match.groups()
        is_array = True
        array_size = None
        array_upper_bound = None

        if array_spec == "":
            # Unbounded array
            pass
        elif array_spec.startswith("<="):
            # Bounded array
            array_upper_bound = int(array_spec[2:])
        else:
            # Fixed-size array
            array_size = int(array_spec)

        # Handle bounded strings in base_type
        bounded_string_match = re.match(r"^(wstring|string)<=([0-9]+)$", base_type)
        if bounded_string_match:
            base_type_str, string_upper_bound = bounded_string_match.groups()
            return Field(
                type=base_type_str,
                name=name,
                default_value=default_value,
                is_array=is_array,
                array_size=array_size,
                array_upper_bound=array_upper_bound,
                is_bounded_string=True,
                string_upper_bound=int(string_upper_bound),
            )
        else:
            return Field(
                type=base_type,
                name=name,
                default_value=default_value,
                is_array=is_array,
                array_size=array_size,
                array_upper_bound=array_upper_bound,
            )

    # Handle bounded strings: string<=N
    bounded_string_match = re.match(r"^(wstring|string)<=([0-9]+)$", type_str)
    if bounded_string_match:
        base_type, string_upper_bound = bounded_string_match.groups()
        return Field(
            type=base_type,
            name=name,
            default_value=default_value,
            is_bounded_string=True,
            string_upper_bound=int(string_upper_bound),
        )

    # Regular field
    return Field(type=type_str, name=name, default_value=default_value)


def cleanup_unprocessed_packages(
    src_dir: Path, all_packages: List[str], processed_packages: List[str]
) -> None:
    """Remove directories for packages that were not successfully processed."""
    import shutil
    
    processed_set = set(processed_packages)
    for package in all_packages:
        if package not in processed_set:
            package_dir = src_dir / package
            if package_dir.exists():
                shutil.rmtree(package_dir)
                print(f"Cleaned up unprocessed package directory: {package_dir}")


def process_package(
    package_path: Path, output_dir: Path, available_packages: Set[str]
) -> Tuple[List[str], bool]:
    """
    Process all .msg and .srv files in the given package directory.
    Generates Rust modules reflecting the package's directory structure.
    Returns a tuple: (list of dependencies, has_msgs_or_srvs)
    """
    package_name = package_path.name
    msg_dir = package_path / "msg"
    srv_dir = package_path / "srv"
    package_output_dir = output_dir / package_name
    package_output_dir.mkdir(exist_ok=True)

    if not msg_dir.is_dir() and not srv_dir.is_dir():
        print(
            f"No 'msg' or 'srv' directory found in package '{package_name}'. Skipping."
        )
        return [], False

    dependencies = set()
    has_msgs_or_srvs = False

    # Prepare module files
    mod_lines = []

    # Process msg files
    if msg_dir.is_dir():
        msg_files = glob(f"{msg_dir}/*.msg") + glob(f"{msg_dir}/**/*.msg")
        msg_files = [Path(f) for f in msg_files]
        if msg_files:
            has_msgs_or_srvs = True
            msg_output_dir = package_output_dir / "msg"
            msg_output_dir.mkdir(exist_ok=True)
            msg_mod_lines = []

            for filename in msg_files:
                msg_file_path = filename
                struct_name = filename.stem
                # print(f"Processing message '{struct_name}' in package '{package_name}'")
                message_spec = parse_msg_file(msg_file_path)

                # Generate code for the message
                rust_struct_code, msg_dependencies = generate_rust_struct_code(
                    message_spec, struct_name, package_name, dependencies
                )
                dependencies.update(msg_dependencies)

                # Write the message struct to its own file
                struct_file = msg_output_dir / f"{to_snake_case(struct_name)}.rs"
                with open(struct_file, "w") as f:
                    f.write(rust_struct_code)
                # print(f"Generated {struct_file}")

                # Update the mod.rs file in msg/
                msg_mod_lines.append(f"mod {to_snake_case(struct_name)};")
                msg_mod_lines.append(
                    f"pub use {to_snake_case(struct_name)}::{struct_name};"
                )

            # Write mod.rs in msg/
            with open(msg_output_dir / "mod.rs", "w") as f:
                f.write("\n".join(msg_mod_lines))

            # Update the package's mod.rs
            mod_lines.append('#[cfg(feature = "{0}")]'.format(package_name))
            mod_lines.append("pub mod msg;")

    # Process srv files
    if srv_dir.is_dir():
        srv_files = glob(f"{srv_dir}/*.srv") + glob(f"{srv_dir}/**/*.srv")
        srv_files = [Path(f) for f in srv_files]
        if srv_files:
            has_msgs_or_srvs = True
            srv_output_dir = package_output_dir / "srv"
            srv_output_dir.mkdir(exist_ok=True)
            srv_mod_lines = []

            for filename in srv_files:
                srv_file_path = filename
                service_name = filename.stem
                # print(
                #    f"Processing service '{service_name}' in package '{package_name}'"
                # )
                request_spec, response_spec = parse_srv_file(srv_file_path)

                # Generate code for the service
                rust_service_code, srv_dependencies = generate_rust_service_code(
                    service_name,
                    request_spec,
                    response_spec,
                    package_name,
                    dependencies,
                )
                dependencies.update(srv_dependencies)

                # Write the service struct to its own file
                service_file = srv_output_dir / f"{to_snake_case(service_name)}.rs"
                with open(service_file, "w") as f:
                    f.write(rust_service_code)
                # print(f"Generated {service_file}")

                # Update the mod.rs file in srv/
                srv_mod_lines.append(f"mod {to_snake_case(service_name)};")
                srv_mod_lines.append(
                    f"pub use {to_snake_case(service_name)}::{service_name};"
                )
                srv_mod_lines.append(
                    f"pub use {to_snake_case(service_name)}::{service_name}Request;"
                )
                srv_mod_lines.append(
                    f"pub use {to_snake_case(service_name)}::{service_name}Response;"
                )

            # Write mod.rs in srv/
            with open(srv_output_dir / "mod.rs", "w") as f:
                f.write("\n".join(srv_mod_lines))

            # Update the package's mod.rs
            mod_lines.append('#[cfg(feature = "{0}")]'.format(package_name))
            mod_lines.append("pub mod srv;")

    if not has_msgs_or_srvs:
        # No .msg or .srv files processed
        return [], False

    # Sanity check: ensure all dependencies are in available_packages
    missing_dependencies = dependencies - available_packages
    if missing_dependencies:
        print(
            f"Error: Package '{package_name}' depends on missing packages: {', '.join(missing_dependencies)}"
        )
        sys.exit(1)

    # Write the package's mod.rs
    with open(package_output_dir / "mod.rs", "w") as f:
        f.write("\n".join(mod_lines))

    return list(dependencies), True


def generate_rust_struct_code(
    message_spec: MessageSpec,
    struct_name: str,
    current_package: str,
    dependencies: Set[str],
    include_serde_import: bool = True,
) -> Tuple[str, Set[str]]:
    rust_fields = []
    msg_dependencies = set()

    use_serde_as = False  # Flag to check if serde_as is needed

    # Handle fields
    for field in message_spec.fields:
        rust_type, field_dependencies = get_rust_field_type(field.type, current_package)
        msg_dependencies.update(field_dependencies)

        # Collect dependencies
        if field_dependencies:
            dependencies.update(field_dependencies)
        elif is_custom_type(field.type):
            # Add current package as dependency if referencing its own types
            dependencies.add(current_package)

        # Handle bounded strings
        if field.is_bounded_string:
            rust_type = (
                "::std::string::String"  # Use fully qualified standard library String
            )

        # Handle arrays
        field_attrs = ""  # Initialize field attributes
        if field.is_array:
            if field.array_size is not None:
                # Fixed-size array
                rust_type = f"[{rust_type}; {field.array_size}]"
                if field.array_size > 32:
                    use_serde_as = True
                    field_attrs += f'    #[serde_as(as = "[_; {field.array_size}]")]\n'
            else:
                # Variable-size or bounded array
                rust_type = f"Vec<{rust_type}>"

        # Handle reserved keywords in field names
        field_name = field.name
        if field_name in RUST_KEYWORDS:
            field_attrs += f'    #[serde(rename = "{field_name}")]'
            field_name = f"{field_name}_"

        field_definition = ""
        if field_attrs:
            field_definition += f"{field_attrs}"

        if field.default_value is not None:
            field_definition += (
                f"    pub {field_name}: {rust_type}, // default: {field.default_value}"
            )
        else:
            field_definition += f"    pub {field_name}: {rust_type},"

        rust_fields.append(field_definition)

    # Generate the Rust struct
    rust_struct = ""
    if include_serde_import:
        rust_struct += "use serde::{Deserialize, Serialize};\n"
    if use_serde_as:
        rust_struct += "use serde_with::serde_as;\n"
    rust_struct += "\n"
    if use_serde_as:
        rust_struct += "#[serde_as]\n"
    rust_struct += (
        f"#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\n"
        f"pub struct {struct_name} {{\n" + "\n".join(rust_fields) + "\n}"
    )

    # Map constants to Rust code within an impl block
    rust_consts = []
    for const in message_spec.constants:
        rust_type = map_ros_type_to_rust(const.type, current_package)
        # Adjust the rust_type for string constants
        if rust_type == "::std::string::String":
            rust_type = "&'static str"
        rust_value = format_constant_value(rust_type, const.value)
        rust_consts.append(f"    pub const {const.name}: {rust_type} = {rust_value};")

    if rust_consts:
        consts_impl = f"impl {struct_name} {{\n" + "\n".join(rust_consts) + "\n}"
    else:
        consts_impl = ""

    # Generate the Default implementation
    default_impl = generate_default_impl(struct_name, message_spec, current_package)

    # Generate impl Message for {struct_name}
    message_impl = f"impl ros2_client::Message for {struct_name} {{}}\n"

    # Combine struct, constants impl, default impl, and message impl
    rust_code = rust_struct
    if consts_impl:
        rust_code += "\n\n" + consts_impl
    rust_code += "\n\n" + default_impl
    rust_code += "\n\n" + message_impl

    return rust_code, msg_dependencies


def generate_rust_service_code(
    service_name: str,
    request_spec: MessageSpec,
    response_spec: MessageSpec,
    current_package: str,
    dependencies: Set[str],
) -> Tuple[str, Set[str]]:
    service_dependencies = set()
    code_parts = []

    # Generate code for request struct
    request_struct_name = f"{service_name}Request"
    rust_request_code, req_dependencies = generate_rust_struct_code(
        request_spec,
        request_struct_name,
        current_package,
        dependencies,
        include_serde_import=False,
    )
    service_dependencies.update(req_dependencies)
    code_parts.append(rust_request_code)

    # Generate code for response struct
    response_struct_name = f"{service_name}Response"
    rust_response_code, res_dependencies = generate_rust_struct_code(
        response_spec,
        response_struct_name,
        current_package,
        dependencies,
        include_serde_import=False,
    )
    service_dependencies.update(res_dependencies)
    code_parts.append(rust_response_code)

    # Generate code for the service struct and its impl
    service_struct_code = f"pub struct {service_name};\n"
    service_struct_code += f"impl ros2_client::Service for {service_name} {{\n"
    service_struct_code += f"    type Request = {request_struct_name};\n"
    service_struct_code += f"    type Response = {response_struct_name};\n\n"
    service_struct_code += (
        f'    fn request_type_name(&self) -> &str {{ "{request_struct_name}" }}\n'
    )
    service_struct_code += (
        f'    fn response_type_name(&self) -> &str {{ "{response_struct_name}" }}\n'
    )
    service_struct_code += "}\n"
    code_parts.append(service_struct_code)

    # Combine all parts with a single serde import at the top
    rust_code = "use serde::{Deserialize, Serialize};\n\n" + "\n\n".join(code_parts)

    return rust_code, service_dependencies


def get_rust_field_type(ros_type: str, current_package: str) -> Tuple[str, Set[str]]:
    rust_type = map_ros_type_to_rust(ros_type, current_package)
    dependencies = set()

    if rust_type.startswith("::"):
        # It's a standard library type; no dependencies to add
        pass
    elif rust_type.startswith("crate::"):
        parts = rust_type.split("::")
        try:
            crate_index = parts.index("crate")
            dep_package = parts[crate_index + 1]
            if dep_package != current_package:
                dependencies.add(dep_package)
        except (ValueError, IndexError):
            # If 'crate' not found or index out of range, ignore
            pass
    else:
        # Built-in type; no dependencies
        pass

    return rust_type, dependencies


def is_custom_type(ros_type: str) -> bool:
    base_type = re.sub(r"<=\d+$", "", ros_type)  # Remove bounded string specifiers
    base_type = re.sub(r"\[.*\]", "", base_type)  # Remove array specifiers
    base_type = base_type.split("/")[-1]  # Handle namespaces
    return base_type not in [
        "bool",
        "byte",
        "char",
        "int8",
        "uint8",
        "int16",
        "uint16",
        "int32",
        "uint32",
        "int64",
        "uint64",
        "float32",
        "float64",
        "string",
        "wstring",
    ]


def type_implements_copy(rust_type: str) -> bool:
    rust_type = rust_type.strip()
    # Handle cases like [u8; 32]
    rust_type = re.sub(r"\[|\]|;.*", "", rust_type)
    return rust_type in COPY_TRAIT_TYPES


def rust_type_name(rust_type: str) -> str:
    return rust_type.split("::")[-1]


def format_constant_value(rust_type: str, value: str) -> str:
    value = value.strip()
    if rust_type == "&'static str":
        # Handle string constants
        value = value.strip("\"'")
        value = value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
        return f'"{value}"'
    elif rust_type == "bool":
        if value == "0":
            return "false"
        elif value == "1":
            return "true"
        elif value.lower() == "false":
            return "false"
        elif value.lower() == "true":
            return "true"
        else:
            raise ValueError(f"Invalid boolean constant value: {value}")
    else:
        return value


def generate_default_impl(
    struct_name: str, message_spec: MessageSpec, current_package: str
) -> str:
    default_fields = []
    for field in message_spec.fields:
        field_name = field.name
        if field_name in RUST_KEYWORDS:
            field_name = f"{field_name}_"
        default_value = field.default_value

        rust_type, _ = get_rust_field_type(field.type, current_package)

        # Determine the default value
        if default_value is not None:
            rust_default_value = map_default_value(
                field, default_value, current_package, struct_name
            )
        else:
            # Use the default value for the type
            rust_default_value = default_value_for_type(field, current_package)

        default_fields.append(f"            {field_name}: {rust_default_value},")

    default_impl = (
        f"impl Default for {struct_name} {{\n"
        f"    fn default() -> Self {{\n"
        f"        {struct_name} {{\n" + "\n".join(default_fields) + "\n"
        "        }\n"
        "    }\n"
        "}"
    )
    return default_impl


def map_default_value(
    field: Field, default_value: str, current_package: str, struct_name: str
) -> str:
    rust_type, _ = get_rust_field_type(field.type, current_package)

    # Handle arrays
    if field.is_array:
        # Parse array default values
        array_values = default_value.strip("[]").split(",")
        element_rust_type, _ = get_rust_field_type(field.type, current_package)
        rust_values = [
            map_scalar_value(element_rust_type, v.strip()) for v in array_values
        ]

        if field.array_size is not None:
            # Fixed-size array
            return f"[{', '.join(rust_values)}]"
        else:
            # Variable-size array
            return f"vec![{', '.join(rust_values)}]"

    # Handle strings
    if rust_type == "::std::string::String":
        value = default_value.strip("\"'")  # Remove surrounding quotes
        value = value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
        return f'::std::string::String::from("{value}")'

    # Check if default value is a constant
    if default_value.isupper():
        return f"Self::{default_value}"

    # Handle booleans
    if rust_type == "bool":
        return map_scalar_value(rust_type, default_value)

    # Handle floating point types
    if rust_type in ["f32", "f64"]:
        # Ensure the value is a float literal
        if "." not in default_value and "e" not in default_value.lower():
            default_value += ".0"
        return default_value

    # Handle scalar values
    return map_scalar_value(rust_type, default_value)


def map_scalar_value(rust_type: str, value: str) -> str:
    # Handle boolean literals
    if rust_type == "bool":
        value = value.strip()
        if value in ("0", "false", "False"):
            return "false"
        elif value in ("1", "true", "True"):
            return "true"
        else:
            raise ValueError(f"Invalid boolean value: {value}")

    # Handle string literals
    if rust_type == "::std::string::String":
        value = value.strip("\"'")  # Remove surrounding quotes
        value = value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
        return f'::std::string::String::from("{value}")'

    # Handle floating point types
    if rust_type in ["f32", "f64"]:
        # Ensure the value is a float literal
        if "." not in value and "e" not in value.lower():
            value += ".0"
        return value

    # Handle numeric literals and constants
    return value


def default_value_for_type(field: Field, current_package: str) -> str:
    rust_type, _ = get_rust_field_type(field.type, current_package)

    # Handle arrays
    if field.is_array:
        if field.array_size is not None:
            # Fixed-size array
            element_rust_type, _ = get_rust_field_type(field.type, current_package)
            if not type_implements_copy(element_rust_type):
                # Initialize using core::array::from_fn
                return f"core::array::from_fn(|_| {element_rust_type}::default())"
            else:
                default_element = default_value_for_type_simple(element_rust_type)
                return f"[{default_element}; {field.array_size}]"
        else:
            return "Vec::new()"
    # Handle default values for types
    if rust_type == "::std::string::String":
        return "::std::string::String::new()"
    elif rust_type in ["f32", "f64"]:
        return "0.0"
    elif rust_type in ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"]:
        return "0"
    elif rust_type == "bool":
        return "false"
    else:
        return f"{rust_type}::default()"


def default_value_for_type_simple(rust_type: str) -> str:
    if rust_type in ["f32", "f64"]:
        return "0.0"
    elif rust_type in ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"]:
        return "0"
    elif rust_type == "bool":
        return "false"
    else:
        return f"{rust_type}::default()"


def map_ros_type_to_rust(ros_type: str, current_package: str) -> str:
    ros_to_rust_map = {
        "bool": "bool",
        "byte": "u8",
        "char": "i8",
        "int8": "i8",
        "uint8": "u8",
        "int16": "i16",
        "uint16": "u16",
        "int32": "i32",
        "uint32": "u32",
        "int64": "i64",
        "uint64": "u64",
        "float32": "f32",
        "float64": "f64",
        "string": "::std::string::String",
        "wstring": "::std::string::String",
        # Add more mappings if needed
    }

    # Handle bounded strings: strip <=N
    bounded_string_match = re.match(r"^(wstring|string)<=\d+$", ros_type)
    if bounded_string_match:
        base_type = bounded_string_match.group(1)
        rust_type = ros_to_rust_map.get(base_type, base_type)
        return rust_type

    # Handle nested types (e.g., "std_msgs/Header")
    if "/" in ros_type:
        package_name, type_name = ros_type.split("/")
        rust_type = f"crate::{package_name}::msg::{type_name}"
        return rust_type

    # Handle built-in types
    if ros_type in ros_to_rust_map:
        return ros_to_rust_map[ros_type]

    # Handle custom types within the current package
    rust_type = f"crate::{current_package}::msg::{ros_type}"
    return rust_type


def generate_lib_rs(output_dir: Path, packages: List[str]):
    lib_rs_path = output_dir / "lib.rs"
    lines = []

    for package in packages:
        lines.append(f'#[cfg(feature = "{package}")]')
        lines.append(f"pub mod {package};\n")

    with open(lib_rs_path, "w") as f:
        f.write("\n".join(lines))

    # print(f"Generated {lib_rs_path}")


def update_cargo_toml(
    cargo_toml_path: Path,
    packages: List[str],
    package_dependencies: dict,
    version: str,
):
    # Read existing Cargo.toml content
    with open(cargo_toml_path, "r") as f:
        cargo_toml_lines = f.readlines()

    # Update the version
    new_cargo_toml_lines = []
    in_package_section = False
    for line in cargo_toml_lines:
        if line.strip().startswith("[package]"):
            in_package_section = True
            new_cargo_toml_lines.append(line)
            continue
        if in_package_section:
            if line.strip().startswith("version ="):
                new_cargo_toml_lines.append(f'version = "{version}"\n')
            elif line.strip().startswith("["):
                in_package_section = False
                new_cargo_toml_lines.append(line)
            else:
                new_cargo_toml_lines.append(line)
        else:
            new_cargo_toml_lines.append(line)

    # Remove existing [features] section
    new_cargo_toml_lines = []
    in_features_section = False
    for line in cargo_toml_lines:
        if line.strip().startswith("[features]"):
            in_features_section = True
            continue
        if in_features_section:
            if line.strip().startswith("["):
                # End of features section
                in_features_section = False
                new_cargo_toml_lines.append(line)
            else:
                continue  # Skip existing feature definitions
        else:
            new_cargo_toml_lines.append(line)

    # Add new [features] section
    new_cargo_toml_lines.append("[features]\n")
    new_cargo_toml_lines.append("default = []\n")
    all_str = ", ".join(f'"{package}"' for package in sorted(packages))
    new_cargo_toml_lines.append(f"all = [{all_str}]\n")

    for package in sorted(packages):
        deps = set(package_dependencies.get(package, []))
        deps.discard(package)  # Remove self-dependency
        if deps:
            # Include dependencies in the feature definition
            deps_str = ", ".join(f'"{dep}"' for dep in sorted(deps))
            new_cargo_toml_lines.append(f"{package} = [{deps_str}]\n")
        else:
            new_cargo_toml_lines.append(f"{package} = []\n")

    # Write updated Cargo.toml
    with open(cargo_toml_path, "w") as f:
        f.writelines(new_cargo_toml_lines)

    print(f"Updated {cargo_toml_path}")


def to_snake_case(name: str) -> str:
    """Convert CamelCase or PascalCase to snake_case."""
    s1 = re.sub("(.)([A-Z][a-z]+)", r"\1_\2", name)
    snake_cased = re.sub("([a-z0-9])([A-Z])", r"\1_\2", s1).lower()
    if snake_cased in RUST_KEYWORDS:
        snake_cased += "_"
    return snake_cased


# Example usage
if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage: python script.py <packages_dir> <crate_dir> <version>")
        sys.exit(1)

    main(
        packages_dir=Path(sys.argv[1]),
        crate_dir=Path(sys.argv[2]),
        version=sys.argv[3],
    )

    sys.exit(0)
