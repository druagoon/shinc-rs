use std::path::Path;

use git2::{IndexAddOption, Repository};

use crate::git_ref_tag;

const HEAD: &str = "HEAD";

pub struct Repo {
    inner: Repository,
}

#[allow(dead_code)]
impl Repo {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let repo = Repository::open(path)?;
        Ok(Self { inner: repo })
    }

    pub fn with_local() -> anyhow::Result<Self> {
        let repo = Repository::discover(".")?;
        Ok(Self { inner: repo })
    }

    pub fn tag_exists(&self, tag_name: &str) -> anyhow::Result<bool, git2::Error> {
        let r = self.inner.revparse_single(&git_ref_tag!(tag_name));
        match r {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.code() == git2::ErrorCode::NotFound {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn current_branch(&self) -> anyhow::Result<String> {
        let head = self.inner.head()?;
        let branch = head.shorthand().unwrap();
        Ok(branch.to_string())
    }

    pub fn commit<P: AsRef<Path>>(&self, files: &[P], message: &str) -> anyhow::Result<()> {
        let mut index = self.inner.index()?;
        for file in files {
            index.add_path(file.as_ref())?;
        }
        index.write()?;
        let tree_id = index.write_tree()?;
        let tree = self.inner.find_tree(tree_id)?;
        let signature = self.inner.signature()?;
        let parent_commit = self.inner.head()?.peel_to_commit()?;
        self.inner.commit(Some(HEAD), &signature, &signature, message, &tree, &[&parent_commit])?;
        Ok(())
    }

    pub fn commit_all(&self, message: &str) -> anyhow::Result<()> {
        let mut index = self.inner.index()?;
        index.add_all(["."].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;
        let tree_id = index.write_tree()?;
        let tree = self.inner.find_tree(tree_id)?;
        let signature = self.inner.signature()?;
        let parent_commit = self.inner.head()?.peel_to_commit()?;
        self.inner.commit(Some(HEAD), &signature, &signature, message, &tree, &[&parent_commit])?;
        Ok(())
    }

    pub fn create_tag(&self, tag_name: &str, message: &str) -> anyhow::Result<()> {
        let head = self.inner.head()?;
        let commit = head.peel_to_commit()?;
        self.inner.tag(tag_name, commit.as_object(), &self.inner.signature()?, message, false)?;
        Ok(())
    }
}
