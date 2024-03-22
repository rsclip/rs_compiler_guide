//! Frontend compiler
//! Called through args

use anyhow::{Error, Result};
use ariadne::Cache;
use log::debug;
use std::path::Path;

use crate::ast::AST;
use crate::errors::{ErrorReporter, SemanticError};
use crate::files::Files;
use crate::lexer;
use crate::parser::Parser;
use crate::semantic_analysis::analyse;
use crate::token::Token;

pub struct Compiler {
    files: Files,
    main_file: String,
    options: CompilerOptions,
}

pub struct CompilerOptions {
    pub verbose: bool,
    pub print_tokens: bool,
    pub print_ast: bool,
    pub print_ir: bool,
    pub print_asm: bool,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -> Self {
        Self {
            files: Files::new(),
            main_file: String::new(),
            options,
        }
    }

    /// Add a file to the compiler
    pub fn add_file(&mut self, path: String) {
        // verify file exists
        if !Path::new(&path).exists() {
            panic!("File does not exist: {}", path);
        }

        let content = std::fs::read_to_string(&path)
            .expect(format!("Unable to read file: {}", path).as_str());

        self.add_source(path, content);
    }

    /// Add source code to the compiler
    pub fn add_source(&mut self, path: String, source: String) {
        self.files.add_file(path.clone(), source);

        if self.main_file.is_empty() {
            self.main_file = path;
        }
    }

    pub fn compile<P>(&mut self, _dst: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Track files to compile
        let mut files_to_compile = vec![self.main_file.clone()];

        // Loop until all files are compiled
        while let Some(file_path) = files_to_compile.pop() {
            println!("Compiling: {}", &file_path);
            self.compile_file(file_path)?;
            println!("Compiled");
        }

        Ok(())
    }

    fn compile_file(&mut self, file_path: String) -> Result<()> {
        debug!("Tokenizing: {}", &file_path);
        let tokens = self.lex_file(file_path.clone())?;

        debug!("Parsing: {}", &file_path);
        let program = self.parse_tokens(tokens, file_path.clone())?;

        debug!("Analysing: {}", &file_path);
        self.analyse_ast(&program, file_path.clone())?;

        Ok(())
    }

    fn parse_tokens<'f>(&mut self, stream: Vec<Token>, file_id: String) -> Result<AST> {
        let mut parser = Parser::new(stream);
        let ast = match parser.parse(file_id.clone()) {
            Ok(program) => program,
            Err(err) => {
                self.report_errors(&vec![err], &file_id);
                return Err(anyhow::anyhow!("Failed to parse file"));
            }
        };

        if self.options.print_ast {
            // println!("{:#?}", ast);
            println!("{}", ast);
        }

        Ok(ast)
    }

    fn lex_file<'f>(&mut self, file_id: String) -> Result<Vec<Token>> {
        let src = self.files.fetch(&file_id).unwrap();

        // load source into string
        let src: String = src.chars().collect();

        let lexer = lexer::Lexer::new(&src);
        let (tokens, errors) = lexer::consume_lexer(lexer);

        if self.options.print_tokens {
            for token in &tokens {
                println!("{:?}", token);
            }
        }

        if !errors.is_empty() {
            self.report_errors(&errors, &file_id);

            Err(anyhow::anyhow!("Failed to lex file"))
        } else {
            Ok(tokens)
        }
    }

    fn analyse_ast(&mut self, ast: &AST, file_id: String) -> Result<()> {
        // let errors = analyse(ast);
        // split into errors & warnings by checking downcastref
        let (errors, warnings): (Vec<Error>, Vec<Error>) =
            analyse(ast).into_iter().partition(|e| {
                if e.downcast_ref::<SemanticError>().is_some() {
                    true
                } else {
                    false
                }
            });

        self.report_errors(&errors, &file_id);
        self.report_errors(&warnings, &file_id);

        if !errors.is_empty() {
            return Err(anyhow::anyhow!("Failed to analyse file"));
        }

        Ok(())
    }

    fn report_errors(&mut self, errors: &Vec<Error>, file_id: &String) {
        let mut reporter = ErrorReporter::new(&mut self.files);

        for err in errors {
            reporter
                .report(file_id.clone(), &err)
                .expect("Failed to report error");
        }
    }
}

pub fn default_output_file(input_file: &str) -> String {
    // replace suffix
    let mut output_file = input_file.to_string();
    if let Some(index) = output_file.rfind('.') {
        output_file.replace_range(index.., ".exe");
    } else {
        output_file.push_str(".exe");
    }

    output_file
}
