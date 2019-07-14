//! Set up build logic and repository tools

use std::{
    fs::{copy, read_dir, read_to_string, remove_file, write},
    path::Path,
    process::Command,
};

fn main() {
    let git_dir = Path::new(".git");
    // make sure we are in a git repo
    let git_rev = if git_dir.exists() {
        let git_rev = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .expect("Failed to execute git command");
        String::from_utf8(git_rev.stdout).expect("Failed to read git output as UTF-8")
    } else {
        String::from("unknown")
    };

    if git_dir.exists() {
        install_git_hooks(&git_rev);
    }
}

fn install_git_hooks(git_rev: &String) {
    let hooks_revision_path = Path::new("tools/git-hooks/hooks-revision");
    let installed_hooks_revision_path = Path::new(".git/hooks/hooks-revision");

    if !hooks_revision_path.exists() {
        write(hooks_revision_path, git_rev).expect("Failed to write git rev to hooks revision");
    }

    let needs_update = if installed_hooks_revision_path.exists() {
        read_to_string(hooks_revision_path).expect("Failed to read hooks revision")
            != read_to_string(installed_hooks_revision_path).expect("Failed to read installed hooks revision")
    } else {
        true
    };

    if needs_update {
        let shipped_hooks_path = Path::new("tools/git-hooks");
        let git_hooks_path = Path::new(".git/hooks");
        for file in read_dir(git_hooks_path).expect("Failed to read git hooks dir") {
            remove_file(file.expect("Failed to get file in git hooks dir").path())
                .expect("Failed to delete file in git hooks dir");
        }

        for file in read_dir(shipped_hooks_path).expect("Failed to read tools hooks dir") {
            let file = file.expect("Failed to get file in tools hooks dir");
            let target = git_hooks_path.join(file.file_name());
            copy(file.path(), &target).expect("Failed to copy file to git hooks dir");
        }

        write(hooks_revision_path, git_rev).expect("Failed to write git rev to hooks revision");
    }
}
