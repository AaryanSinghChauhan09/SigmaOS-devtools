//! devtools — Arch developer tools CLI.

use sigmaos_devtools::{plan_chroot_build, ChrootSpec, plan_commit, plan_db_update, RepoName, RepoPackage};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("chroot") => {
            let plan = plan_chroot_build(&ChrootSpec::default()).unwrap();
            println!("{}\nsteps: {:?}", plan.summary, plan.steps);
        }
        Some("commit") => {
            let plan = plan_commit("PKGBUILD", "extra", "update", true).unwrap();
            println!("{}\nsteps: {:?}", plan.summary, plan.steps);
        }
        Some("dbupdate") => {
            let staged = vec![RepoPackage { filename: "foo-1.0-1-x86_64.pkg.tar.zst".into(), name: "foo".into(), version: "1.0-1".into() }];
            let plan = plan_db_update(RepoName::Extra, &staged, &[]);
            println!("{}\noutputs: {:?}", plan.summary, plan.outputs);
        }
        _ => eprintln!("usage: devtools <chroot|commit|dbupdate>"),
    }
}
