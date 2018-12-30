use git2::{BranchType, Error, Repository};

pub fn find_new_files_at_path(file_path: &str) -> Result<Vec<String>, Error> {
    let repo = Repository::discover(".")?;
    let head = repo.head()?;
    if !head.is_branch() {
        return Err(Error::from_str("HEAD is not a branch"));
    }
    let branch_name = head
        .shorthand()
        .ok_or_else(|| Error::from_str("Unable to get branch name"))?;
    if branch_name == "master" {
        return Err(Error::from_str("Cannot diff master with master"));
    }
    let head_commit_tree = head.peel_to_commit()?.tree()?;
    let master_branch_tree = repo
        .find_branch("master", BranchType::Local)?
        .into_reference()
        .peel_to_tree()?;

    let diff = repo.diff_tree_to_tree(Some(&master_branch_tree), Some(&head_commit_tree), None)?;

    let mut new_migration_files: Vec<String> = Vec::new();
    for delta in diff.deltas() {
        let path = delta.new_file().path().unwrap();
        if delta.old_file().id().is_zero()
            && !delta.new_file().id().is_zero()
            && path.starts_with(file_path)
        {
            new_migration_files.push(String::from(path.to_str().unwrap()));
        }
    }
    return Ok(new_migration_files);
}
