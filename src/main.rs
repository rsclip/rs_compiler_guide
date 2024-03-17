extern crate anyhow;
extern crate codespan_reporting;
extern crate gumdrop;
extern crate thiserror;

pub mod errors;
pub mod lexer;
pub mod token;
#[macro_use]
pub mod parser;
pub mod ast;

pub mod frontend;

use std::path::{Path, PathBuf};

use gumdrop::Options;

/// The main entry point for the program
#[derive(Debug, Options)]
pub struct Args {
    /// The file to parse
    #[options(free, help = "The file to compile")]
    file: String,

    /// optional output
    #[options(help = "The file to output to")]
    out: Option<String>,

    /// Print tokens?
    #[options(help = "Print tokens")]
    tokens: bool,

    /// Print AST?
    #[options(help = "Print AST")]
    ast: bool,

    /// Print IR?
    #[options(help = "Print IR")]
    ir: bool,

    /// Print ASM?
    #[options(help = "Print ASM")]
    asm: bool,
}

fn compile(opts: Args) -> PathBuf {
    let options = frontend::CompilerOptions {
        verbose: true,
        print_tokens: opts.tokens,
        print_ast: opts.ast,
        print_ir: opts.ir,
        print_asm: opts.asm,
    };

    let mut compiler = frontend::Compiler::new(options);

    compiler.add_file(opts.file.clone());
    let out_str = opts.out.unwrap_or(frontend::compiler::default_output_file(&opts.file));
    let out = Path::new(&out_str).to_path_buf();

    match compiler.compile(out.clone()) {
        Ok(_) => println!("Compiled {} to {}", opts.file, out_str),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    out.clone()
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    compile(args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tokens_compile() {
        let args = vec!["tests/simple_tokens.pyl", "--tokens"];
        let opts = Args::parse_args_default(&args).expect("Failed to parse args");

        compile(opts);
    }

    #[test]
    fn add_compile() {
        let args = vec!["tests/add.pyl", "--ast", "--ir", "--asm", "--tokens"];
        let opts = Args::parse_args_default(&args).expect("Failed to parse args");

        compile(opts);
    }
}
