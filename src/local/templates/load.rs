use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use grp_core::Platform;
use git2::Repository;
use mynox::Context;
use mynox::Extend;
use mynox::Error;

use crate::local::git::structs::GitUtils;
use crate::local::structs::Local;
use super::structs::PrTemplate;

struct LoadOptions<'a> {
    extensions: &'a [&'a str],
    strip_frontmatter: bool,
}

impl Local {
    pub fn load_pr_templates(&self, repo: &Repository) -> Result<Vec<PrTemplate>, Error> {
        let workdir = GitUtils::get_working_dir(repo)?;

        let mut templates = Vec::new();
        let mut visited_paths = HashSet::new();

        match &self.0 {
            Platform::Gitlab => {
                let options = LoadOptions {
                    extensions: &["md"],
                    strip_frontmatter: false,
                };

                if let Some(dir) = Self::find_subdir_case_insensitive(workdir, ".gitlab/merge_request_templates") {
                    Self::scan_gitlab_directory(&dir, &mut templates, &mut visited_paths, &options)?;
                }
            }
            Platform::Github => {
                let options = LoadOptions {
                    extensions: &["md", "txt"],
                    strip_frontmatter: false,
                };

                let base_dirs = [".github", "", "docs"];

                // 1. Single default template (first match in order of precedence: .github/, root, docs/)
                for base in &base_dirs {
                    let parent = if base.is_empty() { workdir.to_path_buf() } else { workdir.join(base) };
                    if let Some(file) = Self::find_candidate_file(
                        &parent,
                        &["PULL_REQUEST_TEMPLATE", "pull_request_template"],
                        options.extensions,
                    ) {
                        if Self::load_template_file(
                            &file,
                            "Default",
                            true,
                            &mut templates,
                            &mut visited_paths,
                            &options,
                        )? {
                            break;
                        }
                    }
                }

                // 2. Multiple templates inside PULL_REQUEST_TEMPLATE subdirectories
                for base in &base_dirs {
                    let rel = if base.is_empty() {
                        "PULL_REQUEST_TEMPLATE".to_string()
                    } else {
                        format!("{base}/PULL_REQUEST_TEMPLATE")
                    };
                    if let Some(dir) = Self::find_subdir_case_insensitive(workdir, &rel) {
                        Self::scan_directory(&dir, &mut templates, &mut visited_paths, &options)?;
                    }
                }
            }
            Platform::Gitea => {
                let options = LoadOptions {
                    extensions: &["md"],
                    strip_frontmatter: true,
                };

                let base_dirs = ["", ".gitea", ".github"];
                Self::load_first_single_template(workdir, &base_dirs, &mut templates, &mut visited_paths, &options)?;
            }
            Platform::Forgejo | Platform::Codeberg => {
                let options = LoadOptions {
                    extensions: &["md"],
                    strip_frontmatter: true,
                };

                let base_dirs = ["", ".forgejo", ".gitea", ".github", "docs"];
                Self::load_first_single_template(workdir, &base_dirs, &mut templates, &mut visited_paths, &options)?;
            }
        }

        Ok(templates)
    }

    fn load_first_single_template(
        workdir: &Path,
        base_dirs: &[&str],
        templates: &mut Vec<PrTemplate>,
        visited: &mut HashSet<PathBuf>,
        options: &LoadOptions<'_>,
    ) -> Result<(), Error> {
        let filenames = ["PULL_REQUEST_TEMPLATE", "pull_request_template"];

        for base in base_dirs {
            let parent = if base.is_empty() { workdir.to_path_buf() } else { workdir.join(base) };
            if !parent.is_dir() {
                continue;
            }

            if let Some(candidate) = Self::find_candidate_file(&parent, &filenames, options.extensions) {
                if Self::load_template_file(&candidate, "Default", true, templates, visited, options)? {
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn scan_gitlab_directory(
        dir: &Path,
        templates: &mut Vec<PrTemplate>,
        visited: &mut HashSet<PathBuf>,
        options: &LoadOptions<'_>,
    ) -> Result<(), Error> {
        if !dir.is_dir() {
            return Ok(());
        }

        let entries = fs::read_dir(dir)
            .context("local::template::read_dir", "Failed to read template directory")
            .note(format!("» **path**: _<raw>{}</raw>_", dir.display()))?;

        let mut paths: Vec<_> = entries
            .filter_map(|res| res.ok().map(|e| e.path()))
            .filter(|p| p.is_file())
            .collect();

        paths.sort();

        // Load Default.md (case-insensitive) first so it sits at index 0 with is_default = true
        for path in &paths {
            let is_default_stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.eq_ignore_ascii_case("default"));

            if is_default_stem {
                Self::load_template_file(path, "Default", true, templates, visited, options)?;
                break;
            }
        }

        for path in &paths {
            Self::load_template_file(path, "", false, templates, visited, options)?;
        }

        Ok(())
    }

    fn scan_directory(
        dir: &Path,
        templates: &mut Vec<PrTemplate>,
        visited: &mut HashSet<PathBuf>,
        options: &LoadOptions<'_>,
    ) -> Result<(), Error> {
        if !dir.is_dir() {
            return Ok(());
        }

        let entries = fs::read_dir(dir)
            .context("local::template::read_dir", "Failed to read template directory")
            .note(format!("» **path**: _<raw>{}</raw>_", dir.display()))?;

        let mut paths: Vec<_> = entries
            .filter_map(|res| res.ok().map(|e| e.path()))
            .filter(|p| p.is_file())
            .collect();

        paths.sort();

        for path in paths {
            Self::load_template_file(&path, "", false, templates, visited, options)?;
        }

        Ok(())
    }

    fn load_template_file(
        path: &Path,
        fallback_name: &str,
        is_default: bool,
        templates: &mut Vec<PrTemplate>,
        visited: &mut HashSet<PathBuf>,
        options: &LoadOptions<'_>,
    ) -> Result<bool, Error> {
        if !path.is_file() {
            return Ok(false);
        }

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        if !options.extensions.iter().any(|ext| ext.eq_ignore_ascii_case(&extension)) {
            return Ok(false);
        }

        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        if visited.contains(&canonical) {
            return Ok(false);
        }

        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Template");
        let default_name = if !fallback_name.is_empty() {
            fallback_name.to_string()
        } else {
            file_stem.replace('_', " ").replace('-', " ")
        };

        let raw_content = fs::read_to_string(path)
            .context("local::template::read_file", "Failed to read template file")
            .note(format!("» **path**: _<raw>{}</raw>_", path.display()))?;

        let (name, content) = if options.strip_frontmatter && extension == "md" {
            match Self::strip_markdown_frontmatter(&raw_content) {
                Some((extracted_name, body)) => {
                    let resolved_name = extracted_name.unwrap_or(default_name);
                    (resolved_name, body)
                }
                None => (default_name, raw_content),
            }
        } else {
            (default_name, raw_content)
        };

        visited.insert(canonical);
        templates.push(PrTemplate::new(name, is_default, content));
        Ok(true)
    }

    fn find_candidate_file(
        parent: &Path,
        stems: &[&str],
        extensions: &[&str],
    ) -> Option<PathBuf> {
        if !parent.is_dir() {
            return None;
        }

        for stem in stems {
            for ext in extensions {
                let candidate = parent.join(format!("{stem}.{ext}"));
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }

        let entries = fs::read_dir(parent).ok()?;
        let mut paths: Vec<_> = entries
            .filter_map(|res| res.ok().map(|e| e.path()))
            .filter(|p| p.is_file())
            .collect();
        paths.sort();

        for ext in extensions {
            for path in &paths {
                let matches_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| stems.iter().any(|target| s.eq_ignore_ascii_case(target)));
                let matches_ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| e.eq_ignore_ascii_case(ext));

                if matches_stem && matches_ext {
                    return Some(path.clone());
                }
            }
        }

        None
    }

    fn find_subdir_case_insensitive(base: &Path, rel_path: &str) -> Option<PathBuf> {
        let mut current = base.to_path_buf();

        for component in rel_path.split('/').filter(|c| !c.is_empty()) {
            let exact = current.join(component);
            if exact.is_dir() {
                current = exact;
                continue;
            }

            let entries = fs::read_dir(&current).ok()?;
            let mut found = None;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir()
                    && path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.eq_ignore_ascii_case(component))
                {
                    found = Some(path);
                    break;
                }
            }
            current = found?;
        }

        Some(current)
    }

    fn strip_markdown_frontmatter(content: &str) -> Option<(Option<String>, String)> {
        let bytes = content.as_bytes();
        let first_nl = bytes.iter().position(|&b| b == b'\n')?;
        let first_line = &content[..first_nl];

        if !Self::is_separator(first_line) {
            return None;
        }

        let front_start = first_nl + 1;
        let mut line_start = front_start;

        while line_start < bytes.len() {
            let line_end = bytes[line_start..]
                .iter()
                .position(|&b| b == b'\n')
                .map(|pos| line_start + pos)
                .unwrap_or(bytes.len());

            let line = &content[line_start..line_end];
            if Self::is_separator(line) {
                let front = &content[front_start..line_start];
                if !Self::looks_like_frontmatter(front) {
                    return None;
                }

                let name = Self::extract_name_field(front);
                let body = if line_end + 1 <= bytes.len() {
                    content[line_end + 1..].to_string()
                } else {
                    String::new()
                };

                return Some((name, body));
            }

            line_start = line_end + 1;
        }

        None
    }

    fn is_separator(line: &str) -> bool {
        let trimmed = line.trim_end_matches('\r').trim();
        trimmed.len() >= 3 && trimmed.bytes().all(|b| b == b'-')
    }

    fn looks_like_frontmatter(front: &str) -> bool {
        let mut has_key_value = false;
        for line in front.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('-') {
                continue;
            }
            if let Some((key, _)) = trimmed.split_once(':') {
                if !key.trim().is_empty() && key.trim().chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    has_key_value = true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        has_key_value
    }

    fn extract_name_field(front: &str) -> Option<String> {
        for line in front.lines() {
            let trimmed = line.trim();
            if let Some((key, val)) = trimmed.split_once(':') {
                if key.trim().eq_ignore_ascii_case("name") {
                    let unquoted = val
                        .trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .trim();
                    if !unquoted.is_empty() {
                        return Some(unquoted.to_string());
                    }
                }
            }
        }
        None
    }
}
