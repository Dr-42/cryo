# Cryo

A build tool for C projects.

# Build Tool - CLI and Configuration Specification

## Table of Contents

1. [CLI Interface](#cli-interface)
    1. [General Commands](#general-commands)
    2. [Build Commands](#build-commands)
    3. [Run Commands](#run-commands)
    4. [Package and Versioning](#package-and-versioning)
    5. [Project Initialization](#project-initialization)
2. [TOML Configuration](#toml-configuration)
    1. [Build Section](#build-section)
    2. [Dependencies Section](#dependencies-section)
    3. [Subprojects Section](#subprojects-section)
    4. [Custom Build Rules](#custom-build-rules)
    5. [Overrides](#overrides)

---

## CLI Interface

### General Commands

- `cryo build [OPTIONS]`
    - **Description**: Builds the entire project or a specified subproject.
    - **Options**:
      - `--release` : Build in release mode.
      - `--debug` : Build in debug mode (default).
      - `--subproject <name>` : Build only a specific subproject.
      - `--parallel <N>` : Specify the number of parallel jobs for the build.
      - `--generate-compile-commands` : Generate a `compile_commands.json` file.
      - `--generate-vscode-config` : Generate `.vscode/c_cpp_properties.json`.
      - `--inject-metadata` : Inject project metadata (name, version, asset path) into the build.
  
- `cryo run [OPTIONS]`
    - **Description**: Runs the built binary or a specified binary if there are multiple binaries in the project.
    - **Options**:
      - `--binary <name>` : Specify which binary to run if multiple exist.

- `cryo refresh`
    - **Description**: Refresh and update dependencies (like `cargo update`).
    
### Build Commands

- `cryo build --release`
    - **Description**: Builds the project in release mode.
  
- `cryo build --debug`
    - **Description**: Builds the project in debug mode (default).
  
- `cryo build --subproject <name>`
    - **Description**: Build a specific subproject by its name.

### Run Commands

- `cryo run`
    - **Description**: Runs the default or only binary.
  
- `cryo run --binary <name>`
    - **Description**: Runs a specified binary if there are multiple binaries in the project.

### Package and Versioning

- `cryo install`
    - **Description**: Installs the current project or a specified remote Cryo repository for system-wide availability.

- `cryo bump-version [OPTIONS]`
    - **Description**: Bumps the version of the project and adds a Git tag for versioning.
    - **Options**:
      - `--major` : Bump the major version.
      - `--minor` : Bump the minor version.
      - `--patch` : Bump the patch version (default).

### Project Initialization

- `cryo init [OPTIONS]`
    - **Description**: Initializes a new Cryo project in the current directory or a new directory.
    - **Options**:
      - `--name <name>` : Specify the project name.
      - `--dir <path>` : Create a new directory for the project and initialize it there.

---

## TOML Configuration

### Specification
# Cryo Build Tool - TOML Configuration Table

The following table summarizes the fields available in the Cryo build tool's TOML configuration file, specifying the possible values, whether the field is optional or required, and a brief description.

| **Section**       | **Field**              | **Required** | **Type**           | **Possible Values**                                | **Description**                                                                                         |
|-------------------|------------------------|--------------|--------------------|----------------------------------------------------|---------------------------------------------------------------------------------------------------------|
| **[build]**       | `c_standard`           | No           | String             | `"c99"`, `"c11"`, `"gnu11"`, etc.                  | Specifies the C standard to use in the build.                                                            |
|                   | `compiler`             | Yes          | String             | Any valid compiler name (e.g., `"gcc"`, `"clang"`) | Specifies the compiler to use for building the project.                                                  |
|                   | `global_cflags`        | No           | String             | Any valid compiler flags                           | Specifies global compilation flags (e.g., `"-Wall -Wextra"`).                                             |
|                   | `debug_flags`          | No           | String             | Any valid debug flags                              | Specifies flags to use in debug mode builds (e.g., `"-g"`).                                               |
|                   | `release_flags`        | No           | String             | Any valid release flags                            | Specifies flags to use in release mode builds (e.g., `"-O3"`).                                            |
|                   | `parallel_jobs`        | No           | Integer            | Any positive integer                               | Specifies the number of parallel jobs for building (e.g., `4`).                                           |
| **[dependencies]**|                        |              |                    |                                                    | Section for external dependencies.                                                                       |
| **[dependencies.remote]** | `name`         | Yes          | String             | Any valid string                                   | Specifies the name of the remote dependency.                                                             |
|                   | `version`              | No           | String             | Any valid version tag (e.g., `"v1.0.1"`)           | Specifies the version of the dependency (optional).                                                      |
|                   | `source`               | Yes          | URL String         | A valid Git URL                                    | The URL of the remote Git repository for the dependency.                                                  |
|                   | `include_name`         | Yes          | String             | Any valid string                                   | Specifies the folder prefix for source includes from the dependency.                                      |
|                   | `include_dirs`         | Yes          | Array of Strings    | A list of valid directory paths                    | Specifies the directories that need to be included in the build from the dependency.                      |
|                   | `build_method`         | No           | String             | `"cmake"`, `"header-only"`, `"custom"`             | Specifies the build method for the remote dependency.                                                     |
|                   | `build_command`        | No           | String             | Any valid shell command                            | Custom command to build the dependency if `build_method` is `"custom"`.                                   |
|                   | `build_output`         | No           | String             | Any valid output path                              | Specifies the output binary or library if `build_method` is `"custom"`.                                   |
| **[dependencies.pkg_config]** | `name`     | Yes          | String             | Any valid package name                             | Specifies the name of the dependency to be queried via `pkg-config`.                                       |
|                   | `pkg_config_query`     | Yes          | String             | Any valid `pkg-config` query                       | Specifies the query to `pkg-config` (e.g., `"freetype2"`).                                                |
| **[dependencies.manual]** | `name`         | Yes          | String             | Any valid string                                   | Specifies the name of the manually handled dependency.                                                    |
|                   | `ldflags`              | Yes (manual) | String             | Any valid linker flags                             | Specifies manual linker flags for the dependency (e.g., `"-lglfw"`).                                      |
| **[subprojects]**  | `name`                | Yes          | String             | Any valid string                                   | Specifies the name of the subproject.                                                                    |
|                   | `type`                 | Yes          | String             | `"binary"`, `"library"`, `"header-only"`           | Specifies the type of subproject (binary, library, or header-only).                                       |
|                   | `src_dir`              | Yes (except header-only) | String      | A valid directory path                             | Specifies the directory where the subproject source files are located.                                    |
|                   | `include_dirs`         | Yes          | Array of Strings    | A list of valid directory paths                    | Specifies the directories that need to be included in the build for this subproject.                      |
|                   | `dependencies`         | No           | Array of Strings    | List of subproject and remote dependency names     | Specifies the dependencies of the subproject (e.g., `["core", "mylib_v2"]`).                              |
|                   | `output_name`          | Yes          | String             | Any valid file name                                | Specifies the output binary or library name for the subproject (e.g., `"game_executable"`).                |
| **[custom_build_rules]** | `name`          | Yes          | String             | Any valid string                                   | Specifies the name of the custom build rule.                                                              |
|                   | `description`          | No           | String             | Any valid string                                   | A brief description of the custom build rule.                                                             |
|                   | `src_dir`              | Yes          | String             | A valid directory path                             | Specifies the directory where the source files for the custom build are located.                          |
|                   | `output_dir`           | Yes          | String             | A valid directory path                             | Specifies the directory where the output files will be placed.                                            |
|                   | `trigger_extensions`   | Yes          | Array of Strings    | List of valid file extensions                      | Specifies the file extensions that will trigger the custom build rule (e.g., `[".vert", ".frag"]`).        |
|                   | `output_extension`     | Yes          | String             | A valid file extension                             | Specifies the extension for the output files (e.g., `".spv"`).                                            |
|                   | `command`              | Yes          | String             | Any valid shell command                            | Specifies the shell command to run for the custom build (e.g., `glslc -o $out -fshader-stage=vert $in`).   |
|                   | `rebuild_rule`         | Yes          | String             | `"if-changed"`, `"always"`, `"on-trigger"`         | Specifies the condition for rebuilding (only rebuild if changed, always rebuild, or trigger-based).        |
| **[overrides]**    | `name`                | Yes          | String             | Any valid subproject name                          | Specifies the subproject name to which the override applies.                                              |
|                   | `cflags`               | No           | String             | Any valid compilation flags                        | Specifies custom compiler flags for the overridden subproject.                                            |
|                   | `parallel_jobs`        | No           | Integer            | Any positive integer                               | Specifies the number of parallel jobs for this overridden subproject.                                      |

---

### Notes:
- Required fields must be provided for the build to work, while optional fields provide flexibility for advanced customization.
- Fields like `build_method`, `dependencies`, and `cflags` allow the configuration to be as simple or complex as needed for a given project.
- Multiple subprojects and remote dependencies can be defined, each with their own settings.


### Build Section

This section defines the general build configuration, including compiler settings, optimization flags, and parallel job configurations.

```toml
[build]
c_standard = "c11"               # Specify the C standard (e.g., c99, c11, gnu11, etc.)
compiler = "gcc"                 # Compiler
global_cflags = "-Wall -Wextra"   # Global optimization flags
debug_flags = "-g"               # Debug flags for debug builds
release_flags = "-O3"            # Release flags for release builds
parallel_jobs = 4                # Number of parallel jobs for building
```

### Dependencies Section

This section allows specifying external dependencies, both remote and local, fetched from Git or using `pkg-config`.

#### Example:

```toml
[dependencies]
[[dependencies.remote]]
name = "mylib_v1"
version = "v1.0.1"
source = "https://github.com/example/mylib.git"
include_name = "mylib"           # The folder prefix for the source includes
include_dirs = ["src/include"]    # Directories to include in the build

[[dependencies.remote]]
name = "glm"
source = "https://github.com/example/glm.git"
include_name = "glm"
include_dirs = ["src/include"]
build_method = "cmake"            # Specifies custom build method

[[dependencies.pkg_config]]
name = "freetype"
pkg_config_query = "freetype2"    # Queries pkg-config for `freetype2` library

[[dependencies.manual]]
name = "glfw"
ldflags = "-lglfw"                # Manually specifies linking flags for GLFW
```

### Subprojects Section

Defines subprojects within the build configuration, allowing each subproject to have its own configuration, dependencies, and output.

#### Example:

```toml
[[subprojects]]
name = "core"
type = "library"                                 # Can be "binary", "library", or "header-only"
src_dir = "src/core"                             # Source directory
include_dirs = ["src/core/include"]              # Include directories
dependencies = ["mylib_v2.mylibA", "freetype"]   # Dependencies specific to this subproject
output_name = "libcore.a"                        # Output file (static library)

[[subprojects]]
name = "game"
type = "binary"
src_dir = "src/game"
include_dirs = ["src/game/include", "src/core/include"]
dependencies = ["core", "mylib_v2"]              # This binary depends on core and mylib_v2
output_name = "game_executable"                  # Final binary name
```

### Custom Build Rules

Custom build rules allow the definition of additional build processes, such as asset compilation or custom source transformation, outside of regular compilation.

#### Example:

```toml
[[custom_build_rules]]
name = "vulkan_vertex_shaders"
description = "Compile vertex shaders to SPIR-V"
src_dir = "assets/shaders"
output_dir = "assets/compiled_shaders"
trigger_extensions = [".vert"]                  # Only files with this extension will trigger the rule
output_extension = ".spv"                       # Output file extension
command = "glslc -o $out -fshader-stage=vert $in" # Command to run for compilation
rebuild_rule = "if-changed"                     # Only rebuild if the source files change

[[custom_build_rules]]
name = "vulkan_fragment_shaders"
description = "Compile fragment shaders to SPIR-V"
src_dir = "assets/shaders"
output_dir = "assets/compiled_shaders"
trigger_extensions = [".frag"]
output_extension = ".spv"
command = "glslc -o ${out} -fshader-stage=frag ${in}"
rebuild_rule = "if-changed"
```

### Overrides

The `overrides` section allows overriding specific build configurations for subprojects or specific builds (e.g., debugging, more strict compilation flags).

#### Example:

```toml
[[overrides]]
name = "core"
cflags = "-Werror"               # Treat warnings as errors for this subproject
parallel_jobs = 8                # Override parallel job count for this subproject
```

---

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](./LICENSE) file for details.
