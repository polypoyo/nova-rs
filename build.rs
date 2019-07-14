//! Set up build logic and repository tools

use std::{
    fs::{copy, read_dir, remove_file, write},
    path::Path,
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=tools");
    println!("cargo:rerun-if-changed=tools/git-hooks");

    let git_dir = Path::new(".git");

    if git_dir.exists() {
        install_git_hooks();
    }
}

fn install_git_hooks() {
    let installed_hooks_revision_path = Path::new(".git/hooks/hooks-revision");

    if !installed_hooks_revision_path.exists() {
        let shipped_hooks_path = Path::new("tools/git-hooks");
        let git_hooks_path = Path::new(".git/hooks");
        for file in read_dir(git_hooks_path).expect("Failed to read git hooks dir") {
            remove_file(file.expect("Failed to get file in git hooks dir").path())
                .expect("Failed to delete file in git hooks dir");
        }

        for file in read_dir(shipped_hooks_path).expect("Failed to read tools hooks dir") {
            let file = file.expect("Failed to get file in tools hooks dir");
            println!("cargo:rerun-if-changed:{}", file.path().to_str().unwrap());
            let target = git_hooks_path.join(file.file_name());
            copy(file.path(), &target).expect("Failed to copy file to git hooks dir");
        }
    }
}
