//! Frontend compiler
//! Called through args

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use codespan_reporting::files::SimpleFiles;

use crate::errors::ErrorReporter;
use crate::lexer;
use crate::token::Token;

pub struct Compiler {
    files: SimpleFiles<String, String>,
    main_file: String,
    imports: HashMap<String, usize>,
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
            files: SimpleFiles::new(),
            main_file: String::new(),
            imports: HashMap::new(),
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
        let file_id = self.files.add(path.clone(), source);
        self.imports.insert(path.clone(), file_id);

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
            self.compile_file(file_path)?;
        }

        Ok(())
    }

    fn compile_file(&mut self, file_path: String) -> Result<()> {
        let file_id = self
            .imports
            .get(&file_path)
            .ok_or_else(|| anyhow::anyhow!("File not found"))?;

        let tokens = self.lex_file(*file_id)?;
        if self.options.print_tokens {
            for token in &tokens {
                println!("{:?}", token);
            }
        }

        Ok(())
    }

    fn lex_file(&self, file_id: usize) -> Result<Vec<Token>> {
        let simple_file = self.files.get(file_id)?;

        let lexer = lexer::Lexer::new(simple_file.source());
        let (tokens, errors) = lexer::consume_lexer(lexer);

        if !errors.is_empty() {
            let reporter = ErrorReporter::new(&self.files);

            for err in errors {
                reporter.report(file_id, &err)?;
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
