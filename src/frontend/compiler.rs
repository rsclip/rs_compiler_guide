//! Frontend compiler
//! Called through args

use anyhow::Result;
use ariadne::Cache;
use std::path::Path;

use crate::ast::AST;
use crate::errors::ErrorReporter;
use crate::files::Files;
use crate::lexer;
use crate::parser::Parser;
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
        println!("Tokenizing: {}", &file_path);
        let tokens = self.lex_file(file_path.clone())?;

        println!("Parsing: {}", &file_path);
        let _program = self.parse_tokens(tokens, file_path)?;

        Ok(())
    }

    fn parse_tokens<'f>(&mut self, stream: Vec<Token>, file_id: String) -> Result<AST> {
        let mut parser = Parser::new(stream);
        let ast = match parser.parse(file_id.clone()) {
            Ok(program) => program,
            Err(err) => {
                let mut reporter = ErrorReporter::new(&mut self.files);
                reporter
                    .report(file_id, &err)
                    .expect("Failed to report error");
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
            let mut reporter = ErrorReporter::new(&mut self.files);

            for err in errors {
                reporter
                    .report(file_id.clone(), &err)
                    .expect("Failed to report error");
            }

            Err(anyhow::anyhow!("Failed to lex file"))
        } else {
            Ok(tokens)
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
