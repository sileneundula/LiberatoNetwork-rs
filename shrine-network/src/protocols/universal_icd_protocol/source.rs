pub struct Source {
    pub registered_id: u64,
    pub git_repo: String,
    pub labels: Vec<String>,
}

pub struct Bridge {
    pub source: Source,
    pub target_source_id: u64,
    pub target_git_repo: String,
}