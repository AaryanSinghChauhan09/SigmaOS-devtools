//! Repo database update model — db-update / dbscripts semantics.
//!
//! Upstream `db-update` regenerates repo .db files from staged packages
//! and moves packages into the repo tree. This port plans the update:
//! which packages enter/leave each repo and the generated artifacts.

/// A repository name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RepoName {
    /// Core.
    Core,
    /// Extra.
    Extra,
    /// Multilib.
    Multilib,
    /// Testing.
    Testing,
    /// Custom repo.
    Custom(&'static str),
}

impl RepoName {
    /// Repo db filename (e.g. `core.db`).
    pub fn db_file(&self) -> String {
        match self {
            Self::Core => "core.db".into(),
            Self::Extra => "extra.db".into(),
            Self::Multilib => "multilib.db".into(),
            Self::Testing => "testing.db".into(),
            Self::Custom(n) => format!("{n}.db"),
        }
    }
}

/// A package entering/leaving a repo.
#[derive(Debug, Clone)]
pub struct RepoPackage {
    /// Package filename.
    pub filename: String,
    /// Package name.
    pub name: String,
    /// Version.
    pub version: String,
}

/// A db-update plan for one repo.
#[derive(Debug)]
pub struct DbUpdatePlan {
    /// Repo.
    pub repo: RepoName,
    /// Packages to add/update.
    pub added: Vec<RepoPackage>,
    /// Packages to remove.
    pub removed: Vec<RepoPackage>,
    /// Output db files to regenerate.
    pub outputs: Vec<String>,
    /// Summary.
    pub summary: String,
}

/// Plan a db-update for repos given staged vs current package sets.
pub fn plan_db_update(
    repo: RepoName,
    staged: &[RepoPackage],
    current: &[RepoPackage],
) -> DbUpdatePlan {
    let staged_names: std::collections::BTreeSet<_> =
        staged.iter().map(|p| p.filename.clone()).collect();
    let current_names: std::collections::BTreeSet<_> =
        current.iter().map(|p| p.filename.clone()).collect();
    let added: Vec<_> = staged
        .iter()
        .filter(|p| !current_names.contains(&p.filename))
        .cloned()
        .collect();
    let removed: Vec<_> = current
        .iter()
        .filter(|p| !staged_names.contains(&p.filename))
        .cloned()
        .collect();
    let outputs = vec![
        repo.db_file(),
        format!("{}.files", repo.db_file()),
    ];
    let summary = format!(
        "{}: +{} added, -{} removed → {}",
        match repo {
            RepoName::Core => "core",
            RepoName::Extra => "extra",
            RepoName::Multilib => "multilib",
            RepoName::Testing => "testing",
            RepoName::Custom(n) => n,
        },
        added.len(),
        removed.len(),
        outputs.join(", ")
    );
    DbUpdatePlan { repo, added, removed, outputs, summary }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkg(name: &str) -> RepoPackage {
        RepoPackage {
            filename: format!("{name}-1.0-1-x86_64.pkg.tar.zst"),
            name: name.into(),
            version: "1.0-1".into(),
        }
    }

    #[test]
    fn test_diff() {
        let staged = vec![pkg("a"), pkg("b")];
        let current = vec![pkg("a"), pkg("c")];
        let plan = plan_db_update(RepoName::Extra, &staged, &current);
        assert_eq!(plan.added.len(), 1); // b
        assert_eq!(plan.removed.len(), 1); // c
        assert_eq!(plan.outputs[0], "extra.db");
    }

    #[test]
    fn test_repo_db_names() {
        assert_eq!(RepoName::Core.db_file(), "core.db");
        assert_eq!(RepoName::Multilib.db_file(), "multilib.db");
    }
}
