# ROS2 interfaces generated for rust lang

This repository contains Rust structs for all interfaces (i.e., messages and services) that are listed as releases on the ROS Index for various distributions listed [below](#links).

## How ? 

We fetch the [ROS Index](https://index.ros.org/?search_packages=true) to get per distro official packages. We then gather all of them and translate all of them to `rust` interfaces using [ros2_client](https://github.com/Atostek/ros2-client). We commit our changes to the distro folder and then update the crates you can find on [crates.io](https://crates.io)

## Links

| Distribution | Messages | Crates.io |
|---|---|---|
| Rolling | [list of msg](./ros2-interfaces-rolling/) | [ros2-interfaces-rolling](https://crates.io/crates/ros2-interfaces-rolling) |
| Jazzy | [list of msg](./ros2-interfaces-jazzy/) | [ros2-interfaces-jazzy](https://crates.io/crates/ros2-interfaces-jazzy) |
| Iron | [list of msg](./ros2-interfaces-iron/) | [ros2-interfaces-iron](https://crates.io/crates/ros2-interfaces-iron) |
| Humble | [list of msg](./ros2-interfaces-humble/) | [ros2-interfaces-humble](https://crates.io/crates/ros2-interfaces-humble) |

## CI Pipeline

```mermaid
stateDiagram-v2
    SetupRolling: Setup Rolling Only
    SetupAll: Setup All Distros<br/>(rolling, jazzy, iron, humble)
    SetupSpecific: Setup Selected Distro
    Install: Install Dependencies<br/>(uv + Python)
    Clone: Clone Repos<br/>(with cache)
    Query: Query ROS Index<br/>for Repos
    Extract: Extract Interfaces<br/>from .msg/.srv files
    Convert: Convert to Rust<br/>Structs
    RustSetup: Setup Rust<br/>Toolchain
    Compile: Check Compilation<br/>cargo check
    Fail1: Build Failed
    Test: Run Tests<br/>std_msgs publisher
    Fail2: Tests Failed
    PublishCrates: Publish to Crates.io
    Update: Update Distro Folder<br/>Commit & Push
    Cleanup: Clean Temp Dir

    [*] --> Trigger
    
    state TriggerType <<choice>>
    Trigger --> TriggerType
    
    TriggerType --> SetupRolling : Push to dev
    TriggerType --> SelectDistro : Workflow Dispatch
    
    state SelectDistro <<choice>>
    SelectDistro --> SetupAll : All
    SelectDistro --> SetupSpecific : Specific
    
    SetupRolling --> Install
    SetupAll --> Install
    SetupSpecific --> Install
    
    Install --> Query
    Query --> Clone
    Clone --> Extract
    Extract --> Convert
    
    Convert --> RustSetup
    RustSetup --> Compile
    
    state CompilePass <<choice>>
    Compile --> CompilePass
    CompilePass --> Fail1 : Failed
    
    CompilePass --> Test : Success
    
    state TestPass <<choice>>
    Test --> TestPass
    TestPass --> Fail2 : Failed
    
    TestPass --> Update : Success
    
    state UpdatePass <<choice>>
    Update --> UpdatePass
    UpdatePass --> PublishCheck : No Changes
    UpdatePass --> PublishCheck : Changes
    
    state PublishCheck <<choice>>
    PublishCheck --> PublishCrates : Dispatch
    PublishCheck --> Cleanup : Push
    PublishCrates --> Cleanup
    
    Cleanup --> [*]
    Fail1 --> Cleanup
    Fail2 --> Cleanup
```

## Things to do 
Add a cron schedule to the [pipeline](./.github/workflows/pipeline.yml)