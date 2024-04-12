use anyhow::{self, bail};
use std::path::{Path, PathBuf};

pub trait ResolvePath {
    fn resolve_path(&self, current_path: &PathBuf, import_src: &str) -> anyhow::Result<PathBuf>;
}

pub struct SimplePathResolver {
    base_url: String,
}

impl SimplePathResolver {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }
}

impl ResolvePath for SimplePathResolver {
    fn resolve_path(&self, current_path: &PathBuf, import_src: &str) -> anyhow::Result<PathBuf> {
        let p = match import_src.starts_with(".") {
            true => Path::new(current_path).with_file_name(import_src),
            false => Path::new(&self.base_url).join(import_src),
        };

        if let Ok(resolved_path) = p.join("index.js").canonicalize() {
            return Ok(resolved_path);
        }

        if let Ok(resolved_path) = p.join("index.ts").canonicalize() {
            return Ok(resolved_path);
        }

        for extension in ["ts", "tsx", "js", "jsx"] {
            let mut p = p.clone();
            p.set_extension(extension);
            if let Ok(resolved_path) = p.canonicalize() {
                return Ok(resolved_path);
            }
        }

        bail!("Fail to resolve the import src {:?}", import_src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_relative_path() {
        let resolver = SimplePathResolver::new("");

        assert_eq!(
            resolver
                .resolve_path(
                    &PathBuf::from("test-project/everybodyyyy/src/components/buttons/index.ts"),
                    "./counter"
                )
                .unwrap(),
            PathBuf::from("test-project/everybodyyyy/src/components/buttons/counter.tsx")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            resolver
                .resolve_path(
                    &PathBuf::from("test-project/everybodyyyy/src/components/buttons/index.ts"),
                    "../links"
                )
                .unwrap(),
            PathBuf::from("test-project/everybodyyyy/src/components/links/index.ts")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            resolver
                .resolve_path(
                    &PathBuf::from("test-project/everybodyyyy/src/components/buttons/index.ts"),
                    "../../App.tsx"
                )
                .unwrap(),
            PathBuf::from("test-project/everybodyyyy/src/App.tsx")
                .canonicalize()
                .unwrap()
        );
    }

    #[test]
    fn test_resolve_alias_path() {
        let resolver = SimplePathResolver::new("test-project/everybodyyyy/src");

        assert_eq!(
            resolver
                .resolve_path(
                    &PathBuf::from("test-project/everybodyyyy/src/components/buttons/index.ts"),
                    "components"
                )
                .unwrap(),
            PathBuf::from("test-project/everybodyyyy/src/components/index.ts")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            resolver
                .resolve_path(
                    &PathBuf::from("test-project/everybodyyyy/src/components/buttons/index.ts"),
                    "components/links"
                )
                .unwrap(),
            PathBuf::from("test-project/everybodyyyy/src/components/links/index.ts")
                .canonicalize()
                .unwrap()
        );

        assert_eq!(
            resolver
                .resolve_path(
                    &PathBuf::from("test-project/everybodyyyy/src/components/buttons/index.ts"),
                    "App"
                )
                .unwrap(),
            PathBuf::from("test-project/everybodyyyy/src/App.tsx")
                .canonicalize()
                .unwrap()
        );
    }
}
