//! Handles the managing of source code files for `ariadne`

use std::collections::HashMap;
use std::fmt;

use ariadne::Source;

/// Represents a source file
pub struct SourceFile {
    pub id: String,
    pub source: Source,
}

/// Represents a collection of source files
pub struct Files {
    files: HashMap<String, SourceFile>,
}

impl SourceFile {
    /// Create a new `SourceFile`
    pub fn new(id: String, contents: String) -> Self {
        Self {
            id,
            source: Source::from(contents),
        }
    }

    /// Create a new `SourceFile` from a string
    /// Name is set to "src"
    pub fn from_code(contents: String) -> Self {
        Self {
            id: String::new(),
            source: Source::from(contents),
        }
    }
}

impl Files {
    /// Create a new `Files`
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Add a file to the collection
    pub fn add_file(&mut self, id: String, contents: String) {
        let file = SourceFile::new(id, contents);
        self.files.insert(file.id.clone(), file);
    }

    /// Get a file from the collection
    fn get_file(&self, path: &str) -> Option<&SourceFile> {
        self.files.get(path)
    }
}

impl ariadne::Cache<String> for Files {
    type Storage = String;

    fn fetch(
        &mut self,
        id: &String,
    ) -> Result<&Source<Self::Storage>, Box<dyn std::fmt::Debug + '_>> {
        let file = self.get_file(id).unwrap();
        Ok(&file.source)
    }

    fn display<'a>(&self, id: &'a String) -> Option<Box<dyn fmt::Display + 'a>> {
        let file = self.get_file(id)?;
        Some(Box::new(file.id.clone()))
    }
}
