use std::{
    fs, io,
    path::{Path, PathBuf},
};

const IGNORED_DIRECTORIES: &[&str] = &[
    ".git",
    ".idea",
    ".vscode",
    ".cache",
    ".next",
    "__pycache__",
    "build",
    "dist",
    "node_modules",
    "target",
];
const MAX_PROJECT_NODES: usize = 50_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub kind: EntryKind,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn is_dir(&self) -> bool {
        self.kind == EntryKind::Directory
    }
}

#[derive(Clone, Debug)]
pub struct ProjectTree {
    pub root: PathBuf,
    pub entries: Vec<FileNode>,
    pub truncated: bool,
}

#[derive(Clone, Debug)]
pub struct FileRow<'a> {
    pub node: &'a FileNode,
    pub depth: usize,
}

impl ProjectTree {
    pub fn scan(root: impl AsRef<Path>) -> io::Result<Self> {
        let root = root.as_ref().canonicalize()?;
        if !root.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} no es un directorio", root.display()),
            ));
        }

        let mut node_count = 0usize;
        let mut truncated = false;
        let entries = scan_directory(&root, 0, &mut node_count, &mut truncated)?;

        Ok(Self {
            root,
            entries,
            truncated,
        })
    }

    pub fn visible_rows(
        &self,
        expanded_dirs: &std::collections::HashSet<PathBuf>,
    ) -> Vec<FileRow<'_>> {
        let mut rows = Vec::new();
        collect_rows(&self.entries, 0, expanded_dirs, &mut rows);
        rows
    }
}

fn collect_rows<'a>(
    nodes: &'a [FileNode],
    depth: usize,
    expanded_dirs: &std::collections::HashSet<PathBuf>,
    rows: &mut Vec<FileRow<'a>>,
) {
    for node in nodes {
        rows.push(FileRow { node, depth });
        if node.is_dir() && expanded_dirs.contains(&node.path) {
            collect_rows(&node.children, depth + 1, expanded_dirs, rows);
        }
    }
}

fn scan_directory(
    directory: &Path,
    depth: usize,
    node_count: &mut usize,
    truncated: &mut bool,
) -> io::Result<Vec<FileNode>> {
    if depth > 128 || *node_count >= MAX_PROJECT_NODES {
        *truncated |= *node_count >= MAX_PROJECT_NODES;
        return Ok(Vec::new());
    }

    let mut nodes = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let name = entry.file_name().to_string_lossy().into_owned();
        if is_ignored_name(&name) {
            continue;
        }

        if *node_count >= MAX_PROJECT_NODES {
            *truncated = true;
            break;
        }

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };
        let kind = if file_type.is_dir() {
            EntryKind::Directory
        } else if file_type.is_symlink() {
            EntryKind::Symlink
        } else if file_type.is_file() {
            EntryKind::File
        } else {
            EntryKind::Other
        };

        let path = entry.path();
        let children = if kind == EntryKind::Directory {
            scan_directory(&path, depth + 1, node_count, truncated).unwrap_or_default()
        } else {
            Vec::new()
        };

        *node_count += 1;
        nodes.push(FileNode {
            name,
            path,
            kind,
            children,
        });
    }

    nodes.sort_by(|left, right| {
        let left_dir = left.is_dir();
        let right_dir = right.is_dir();
        right_dir
            .cmp(&left_dir)
            .then_with(|| left.name.to_lowercase().cmp(&right.name.to_lowercase()))
            .then_with(|| left.name.cmp(&right.name))
    });

    Ok(nodes)
}

fn is_ignored_name(name: &str) -> bool {
    IGNORED_DIRECTORIES.contains(&name)
}
