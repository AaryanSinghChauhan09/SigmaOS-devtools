//! commitpkg model — Arch devtools' package commit workflow.
//!
//! Upstream commitpkg: verify PKGBUILD, run makepkg --source, commit
//! to the repo (svn/git), optionally signoff packages. This port plans
//! the commit steps: validation → source build → VCS commit → signoff.

/// A commit step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitStep {
    /// Lint PKGBUILD (namcap/devtools checks).
    Lint,
    /// Build source package (`makepkg --source`).
    BuildSource,
    /// Stage files into repo.
    Stage,
    /// Commit to VCS.
    Commit,
    /// Push to remote.
    Push,
    /// Signoff packages on archlinux.org.
    Signoff,
}

/// A planned package commit.
#[derive(Debug)]
pub struct CommitPlan {
    /// Steps in order.
    pub steps: Vec<CommitStep>,
    /// PKGBUILD path.
    pub pkgbuild: String,
    /// Repo to commit to.
    pub repo: String,
    /// Whether signoff is included.
    pub signoff: bool,
    /// Commit message.
    pub message: String,
    /// Summary.
    pub summary: String,
}

/// Plan a commitpkg run.
pub fn plan_commit(
    pkgbuild: &str,
    repo: &str,
    message: &str,
    signoff: bool,
) -> Result<CommitPlan, crate::DevtoolsError> {
    if pkgbuild.is_empty() || !pkgbuild.contains("PKGBUILD") {
        return Err(crate::DevtoolsError::InvalidSpec(
            "pkgbuild path must point to a PKGBUILD".into(),
        ));
    }
    let mut steps = vec![
        CommitStep::Lint,
        CommitStep::BuildSource,
        CommitStep::Stage,
        CommitStep::Commit,
    ];
    if signoff {
        steps.push(CommitStep::Signoff);
    }
    steps.push(CommitStep::Push);
    let summary = format!(
        "commitpkg: {} → {} ({} steps, signoff={})",
        pkgbuild,
        repo,
        steps.len(),
        signoff
    );
    Ok(CommitPlan {
        steps,
        pkgbuild: pkgbuild.to_string(),
        repo: repo.to_string(),
        signoff,
        message: message.to_string(),
        summary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        let p = plan_commit("/repo/extra/foo/PKGBUILD", "extra", "foo: update to 1.0", true).unwrap();
        assert!(p.steps.contains(&CommitStep::Signoff));
        assert_eq!(p.steps.last(), Some(&CommitStep::Push));
        assert!(p.signoff);
    }

    #[test]
    fn test_no_signoff() {
        let p = plan_commit("PKGBUILD", "core", "msg", false).unwrap();
        assert!(!p.steps.contains(&CommitStep::Signoff));
    }

    #[test]
    fn test_invalid_path() {
        assert!(plan_commit("not/a/pkgbuild", "extra", "m", false).is_err());
    }
}
