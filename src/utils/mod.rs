use crate::models::RepositoryInfo;
use crate::models::CombinedRepositoryInfo;
use std::collections::HashMap;

pub fn extract_project_name(name: &str) -> String {
    name
        .split('_')
        .next()
        .unwrap_or_default()
        .split('-')
        .map(|word| {
            word.char_indices()
                .map(|(i, c)| if i == 0 { c.to_uppercase().to_string() } else { c.to_lowercase().to_string() })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn identify_role(name: &str) -> String {
    name.split('_').nth(1).unwrap_or("General").to_string()
}

pub fn combine_repos(repos: Vec<RepositoryInfo>) -> Vec<CombinedRepositoryInfo> {
    let mut projects: HashMap<String, CombinedRepositoryInfo> = HashMap::new();

    for mut repo in repos {
        let project_name = extract_project_name(&repo.name);
        let role = identify_role(&repo.name);

        repo._type = Some(role.clone());

        let project_entry = projects.entry(project_name.clone()).or_insert_with(|| CombinedRepositoryInfo {
            name: project_name,
            components: HashMap::new()
        });

        project_entry.components.insert(role, repo);
    }

    projects.into_values().collect()
}
