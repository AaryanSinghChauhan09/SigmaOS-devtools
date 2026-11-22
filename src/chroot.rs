//! Chroot build model — makechrootpkg semantics.
//!
//! Upstream makechrootpkg creates a clean chroot, bind-mounts
//! /etc/pacman.d/gnupg + makepkg.conf, copies the PKGBUILD dir,
//! runs makepkg inside, then extracts packages. This port models the
//! build plan: steps + bind mounts + verification — callers execute
//! the actual chroot commands.

/// A bind mount spec for the chroot.
#[derive(Debug, Clone)]
pub struct BindMount {
    /// Source path on host.
    pub source: String,
    /// Target path inside chroot.
    pub target: String,
    /// Read-only?
    pub read_only: bool,
}

/// A chroot build step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChrootStep {
    /// Create chroot from packages.
    CreateChroot,
    /// Bind-mount host resources.
    BindMounts,
    /// Copy PKGBUILD source into chroot.
    CopySources,
    /// Sync chroot packages.
    SyncPackages,
    /// Run makepkg inside chroot.
    RunMakepkg,
    /// Copy built packages out.
    ExtractPackages,
    /// Cleanup chroot + mounts.
    Cleanup,
}

/// Chroot build specification.
#[derive(Debug, Clone)]
pub struct ChrootSpec {
    /// Chroot name (e.g. `devtools-builder`).
    pub name: String,
    /// Base packages for chroot.
    pub base_packages: Vec<String>,
    /// Source directory (containing PKGBUILD).
    pub srcdir: String,
    /// Output directory for built packages.
    pub outdir: String,
    /// GPG keyring to bind-mount.
    pub gnupg_dir: String,
    /// makepkg.conf to bind-mount.
    pub makepkg_conf: String,
    /// Extra bind mounts.
    pub extra_mounts: Vec<BindMount>,
    /// Architecture.
    pub arch: String,
}

impl Default for ChrootSpec {
    fn default() -> Self {
        Self {
            name: "builder".into(),
            base_packages: vec!["base-devel".into(), "pacman-contrib".into()],
            srcdir: "src".into(),
            outdir: "pkg".into(),
            gnupg_dir: "/etc/pacman.d/gnupg".into(),
            makepkg_conf: "/etc/makepkg.conf".into(),
            extra_mounts: Vec::new(),
            arch: "x86_64".into(),
        }
    }
}

/// A planned chroot build.
#[derive(Debug)]
pub struct BuildPlan {
    /// Ordered steps.
    pub steps: Vec<ChrootStep>,
    /// Bind mounts to set up.
    pub mounts: Vec<BindMount>,
    /// Command to run inside chroot.
    pub makepkg_cmd: String,
    /// Summary.
    pub summary: String,
}

/// Plan a chroot build (makechrootpkg).
pub fn plan_chroot_build(spec: &ChrootSpec) -> Result<BuildPlan, crate::DevtoolsError> {
    if spec.srcdir.is_empty() {
        return Err(crate::DevtoolsError::InvalidSpec("empty srcdir".into()));
    }
    let mut mounts = vec![
        BindMount {
            source: spec.gnupg_dir.clone(),
            target: "/etc/pacman.d/gnupg".into(),
            read_only: true,
        },
        BindMount {
            source: spec.makepkg_conf.clone(),
            target: "/etc/makepkg.conf".into(),
            read_only: true,
        },
        BindMount {
            source: spec.srcdir.clone(),
            target: "/build/src".into(),
            read_only: false,
        },
        BindMount {
            source: spec.outdir.clone(),
            target: "/build/pkg".into(),
            read_only: false,
        },
    ];
    mounts.extend(spec.extra_mounts.iter().cloned());
    let steps = vec![
        ChrootStep::CreateChroot,
        ChrootStep::BindMounts,
        ChrootStep::CopySources,
        ChrootStep::SyncPackages,
        ChrootStep::RunMakepkg,
        ChrootStep::ExtractPackages,
        ChrootStep::Cleanup,
    ];
    let makepkg_cmd = format!(
        "makechrootpkg -c -r {} -n {} -- {}",
        spec.name,
        spec.arch,
        spec.base_packages.join(" ")
    );
    let summary = format!(
        "chroot `{}` build: {} → {} ({} mounts, arch {})",
        spec.name,
        spec.srcdir,
        spec.outdir,
        mounts.len(),
        spec.arch
    );
    Ok(BuildPlan { steps, mounts, makepkg_cmd, summary })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan() {
        let plan = plan_chroot_build(&ChrootSpec::default()).unwrap();
        assert_eq!(plan.steps.len(), 7);
        assert!(plan.mounts.iter().any(|m| m.target == "/etc/pacman.d/gnupg"));
        assert!(plan.makepkg_cmd.contains("makechrootpkg"));
    }

    #[test]
    fn test_empty_srcdir_fails() {
        let spec = ChrootSpec { srcdir: String::new(), ..Default::default() };
        assert!(plan_chroot_build(&spec).is_err());
    }
}
