use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct TextDocument {
    pub path: PathBuf,
    pub content: String,
    pub read_only: bool,
}

impl TextDocument {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let bytes = fs::read(&path)?;
        let content = String::from_utf8_lossy(&bytes).into_owned();

        Ok(Self {
            path,
            content,
            read_only: true,
        })
    }

    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.path.display().to_string())
    }

    pub fn language_name(&self) -> &'static str {
        language_for_path(&self.path)
    }

    pub fn line_count(&self) -> usize {
        if self.content.is_empty() {
            1
        } else {
            self.content.lines().count()
        }
    }
}

pub fn language_for_path(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "rs" => "Rust",
        "toml" => "TOML",
        "json" => "JSON",
        "js" | "jsx" | "mjs" | "cjs" => "JavaScript",
        "ts" | "tsx" | "mts" | "cts" => "TypeScript",
        "py" => "Python",
        "go" => "Go",
        "c" | "h" => "C",
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => "C++",
        "java" => "Java",
        "kt" | "kts" => "Kotlin",
        "swift" => "Swift",
        "rb" => "Ruby",
        "php" => "PHP",
        "sql" => "SQL",
        "html" | "htm" => "HTML",
        "css" | "scss" | "sass" | "less" => "CSS",
        "md" | "markdown" => "Markdown",
        "sh" | "bash" | "zsh" => "Shell",
        "xml" => "XML",
        "yaml" | "yml" => "YAML",
        _ => "Texto",
    }
}
