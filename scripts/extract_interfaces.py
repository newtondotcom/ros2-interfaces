#!/usr/bin/env python3
import argparse
import json
import shutil
from pathlib import Path
from glob import glob


def find_interface_files(package_path: Path, interface_type: str):
    files = glob(f"{package_path}/{interface_type}/*.{interface_type}")
    files_sub = glob(
        f"{package_path}/{interface_type}/**/*.{interface_type}", recursive=True
    )
    # Deduplicate
    return list(set(files + files_sub))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--distro", required=True)
    parser.add_argument("--json-file", required=True)
    parser.add_argument("--cloned-dir", required=True)
    parser.add_argument("--output-dir", required=True)
    args = parser.parse_args()

    distro = args.distro
    cloned_dir = Path(args.cloned_dir)
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    with open(args.json_file) as f:
        data = json.load(f)

    for repo in data["repositories"]:
        repo_name = repo["name"]
        packages = repo["packages"]
        repo_path = cloned_dir / repo_name

        if not repo_path.exists():
            print(f"Repository path {repo_path} does not exist, skipping {repo_name}")
            continue

        # If packages is empty, treat the repo root as a single package
        if not packages:
            packages = ["."]

        for package_name in packages:
            package_path = repo_path / package_name
            if not package_path.exists():
                continue

            msg_files = find_interface_files(package_path, "msg")
            srv_files = find_interface_files(package_path, "srv")

            if not msg_files and not srv_files:
                continue

            # If package_name == ".", use repo_name as the package directory name
            target_package_name = repo_name if package_name == "." else package_name
            target_package_dir = output_dir / distro / target_package_name
            target_package_dir.mkdir(parents=True, exist_ok=True)

            # Use package_path as the base for relative paths
            for f in msg_files + srv_files:
                rel = Path(f).relative_to(package_path)
                dest = target_package_dir / rel
                dest.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(f, dest)


if __name__ == "__main__":
    main()
