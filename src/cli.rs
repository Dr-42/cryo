/*
* Copyright (c) 2024, Dr. Spandan Roy
*
* This file is part of iceforge.
*
* iceforge is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* iceforge is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with iceforge.  If not, see <https://www.gnu.org/licenses/>.
*/

use clap::{ArgGroup, Parser, Subcommand};

/// Iceforge Build Tool
#[derive(Parser, Debug)]
#[command(author, about, version)]
struct IceforgeCLI {
    /// Build the project
    #[arg(short)]
    build: bool,
    /// Run the project
    #[arg(short)]
    run: bool,
    /// Clean the build directory
    #[arg(short)]
    clean: bool,

    /// Generate compile_commands.json for the project
    #[arg(long)]
    gen_cc: bool,

    /// Generate .vscode/c_cpp_properties.json for the project
    #[arg(long)]
    gen_vsc: bool,
    /// Commands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build the project or a subproject
    Build(BuildOptions),

    /// Run a binary from the project
    Run(RunOptions),

    /// Clean the build directory
    Clean(CleanOptions),

    /// Refresh and update dependencies
    Refresh,

    /// Install the current project or a remote iceforge repo for system-wide availability
    Install,

    /// Publish the project by tagging the current version in the config
    Publish(PublishOptions),

    /// Initialize a new iceforge project
    Init(InitOptions),
}

#[derive(Parser, Debug)]
#[command(group(ArgGroup::new("build_mode").args(&["release", "debug"])))]
struct BuildOptions {
    /// Build in release mode
    #[arg(long, group = "build_mode")]
    release: bool,

    /// Build in debug mode (default)
    #[arg(long, group = "build_mode")]
    debug: bool,

    /// Build only a specific subproject
    #[arg(long)]
    subproject: Option<String>,

    /// Specify the number of parallel jobs for the build
    #[arg(long)]
    parallel: Option<u32>,

    /// Generate compile_commands.json for the project
    #[arg(long)]
    generate_compile_commands: bool,

    /// Generate .vscode/c_cpp_properties.json for the project
    #[arg(long)]
    generate_vscode_config: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            release: false,
            debug: true,
            subproject: None,
            parallel: None,
            generate_compile_commands: false,
            generate_vscode_config: false,
        }
    }
}

#[derive(Parser, Debug, Default)]
struct RunOptions {
    /// Specify which binary to run if multiple exist
    #[arg(long)]
    binary: Option<String>,
}

#[derive(Parser, Debug, Default)]
struct CleanOptions {
    /// Clean only a specific subproject
    #[arg(long)]
    subproject: Option<String>,
}

#[derive(Parser, Debug)]
struct PublishOptions {
    /// Add the git tag to the specified remote repository
    #[arg(long)]
    remote: Option<String>,
}

#[derive(Parser, Debug)]
struct InitOptions {
    /// Specify the project name
    #[arg(long)]
    name: Option<String>,

    /// Create a new directory for the project and initialize it there
    #[arg(long)]
    dir: Option<String>,
}

fn handle_build(opts: BuildOptions) {
    // Handle the build process with the options provided
    println!("Building project...");
    if opts.release {
        println!("Building in release mode");
    }
    if opts.debug {
        println!("Building in debug mode");
    }
    if let Some(subproject) = opts.subproject {
        println!("Building subproject: {}", subproject);
    }
    if let Some(parallel) = opts.parallel {
        println!("Using {} parallel jobs", parallel);
    }
    if opts.generate_compile_commands {
        println!("Generating compile_commands.json");
    }
    if opts.generate_vscode_config {
        println!("Generating .vscode/c_cpp_properties.json");
    }
}

fn handle_run(opts: RunOptions) {
    // Handle running the binary
    if let Some(binary) = opts.binary {
        println!("Running binary: {}", binary);
    } else {
        println!("Running default binary");
    }
}

fn handle_clean(opts: CleanOptions) {
    // Handle the clean operation
    if let Some(subproject) = opts.subproject {
        println!("Cleaning subproject: {}", subproject);
    } else {
        println!("Cleaning the entire project");
    }
}

fn handle_refresh() {
    // Handle refreshing dependencies
    println!("Refreshing dependencies...");
}

fn handle_install() {
    // Handle the installation of the project
    println!("Installing project...");
}

fn handle_publish(opts: PublishOptions) {
    // Handle publishing the project by tagging the current version
    if let Some(remote) = opts.remote {
        println!("Tagging version and pushing to remote: {}", remote);
    } else {
        println!("Tagging version locally");
    }
}

fn handle_init(opts: InitOptions) {
    // Handle initializing a new project
    if let Some(name) = opts.name {
        println!("Initializing project: {}", name);
    } else {
        println!("Initializing project in the current directory");
    }

    if let Some(dir) = opts.dir {
        println!("Creating and initializing project in directory: {}", dir);
    }
}

pub fn parse() {
    let cli = IceforgeCLI::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Build(build_opts) => handle_build(build_opts),
            Commands::Run(run_opts) => handle_run(run_opts),
            Commands::Clean(clean_opts) => handle_clean(clean_opts),
            Commands::Refresh => handle_refresh(),
            Commands::Install => handle_install(),
            Commands::Publish(publish_opts) => handle_publish(publish_opts),
            Commands::Init(init_opts) => handle_init(init_opts),
        }
    }

    if cli.clean {
        handle_clean(CleanOptions::default());
    }
    if cli.build {
        handle_build(BuildOptions {
            generate_compile_commands: cli.gen_cc,
            generate_vscode_config: cli.gen_vsc,
            ..Default::default()
        });
    }
    if cli.run {
        handle_run(RunOptions::default());
    }
}
