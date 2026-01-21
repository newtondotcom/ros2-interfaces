#!/usr/bin/env python3
import argparse
import json
import requests
import sys
import yaml


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--distro", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()

    distro = args.distro
    url = f"https://raw.githubusercontent.com/ros/rosdistro/master/{distro}/distribution.yaml"
    r = requests.get(url)
    if r.status_code != 200:
        print(f"Failed to load distribution.yaml for {distro}", file=sys.stderr)
        sys.exit(1)

    distro_data = yaml.safe_load(r.text)
    repos = distro_data.get("repositories", {})

    # Amend missing sources if needed
    amended_sources = {
        "radar_msgs": {
            "type": "git",
            "url": "https://github.com/ros-perception/radar_msgs.git",
            "version": "ros2",
        },
        "gazebo_ros_pkgs": {
            "type": "git",
            "url": "https://github.com/ros-simulation/gazebo_ros_pkgs.git",
            "version": "ros2",
        },
    }

    def amend_missing_sources(name, data):
        if name in amended_sources and "source" not in data:
            data["source"] = amended_sources[name]

    results = {"repositories": []}
    for name, data in repos.items():
        amend_missing_sources(name, data)
        if "release" in data and "source" in data and "url" in data["source"] and "version" in data["source"]:
            git_url = data["source"]["url"]
            branch = data["source"]["version"]
            # Identify packages
            # If "packages" in release, we'll record them
            # Otherwise, it's empty list
            release_data = data["release"]
            packages = release_data.get("packages", [])
            results["repositories"].append({"name": name, "git_url": git_url, "branch": branch, "packages": packages})

    with open(args.output, "w") as f:
        json.dump(results, f, indent=2)


if __name__ == "__main__":
    main()
