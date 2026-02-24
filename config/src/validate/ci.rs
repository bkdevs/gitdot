use std::collections::{HashMap, HashSet};

use crate::ci::CiConfig;

#[derive(Debug, thiserror::Error)]
pub enum CiConfigError {
    #[error("failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("config validation failed:\n{}", .0.iter().map(|e| format!("  - {e}")).collect::<Vec<_>>().join("\n"))]
    Validation(Vec<String>),
}

pub fn validate(config: &CiConfig) -> Result<(), CiConfigError> {
    let mut errors = Vec::new();

    errors.extend(check_non_empty(config));
    errors.extend(check_task_references(config));
    errors.extend(check_no_orphaned_tasks(config));
    errors.extend(check_dag(config));

    if errors.is_empty() {
        Ok(())
    } else {
        Err(CiConfigError::Validation(errors))
    }
}

fn check_non_empty(config: &CiConfig) -> Vec<String> {
    let mut errors = Vec::new();
    if config.builds.is_empty() {
        errors.push("builds cannot be empty".to_string());
    }
    if config.tasks.is_empty() {
        errors.push("tasks cannot be empty".to_string());
    }
    errors
}

fn check_task_references(config: &CiConfig) -> Vec<String> {
    let defined: HashSet<&str> = config.tasks.iter().map(|t| t.name.as_str()).collect();
    let mut errors = Vec::new();

    for (i, build) in config.builds.iter().enumerate() {
        for task_ref in &build.tasks {
            if !defined.contains(task_ref.as_str()) {
                errors.push(format!("builds[{i}] references unknown task '{task_ref}'"));
            }
        }
    }

    errors
}

fn check_no_orphaned_tasks(config: &CiConfig) -> Vec<String> {
    let referenced: HashSet<&str> = config
        .builds
        .iter()
        .flat_map(|b| b.tasks.iter().map(String::as_str))
        .collect();

    config
        .tasks
        .iter()
        .filter(|t| !referenced.contains(t.name.as_str()))
        .map(|t| format!("task '{}' is not referenced by any build", t.name))
        .collect()
}

fn check_dag(config: &CiConfig) -> Vec<String> {
    let defined: HashSet<&str> = config.tasks.iter().map(|t| t.name.as_str()).collect();

    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for task in &config.tasks {
        adj.entry(task.name.as_str()).or_default();
        if let Some(deps) = &task.runs_after {
            for dep in deps {
                if defined.contains(dep.as_str()) {
                    adj.entry(dep.as_str())
                        .or_default()
                        .push(task.name.as_str());
                }
            }
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Gray,
        Black,
    }

    let mut color: HashMap<&str, Color> = adj.keys().map(|&k| (k, Color::White)).collect();
    let mut path: Vec<&str> = Vec::new();
    let mut errors = Vec::new();

    fn dfs<'a>(
        node: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        color: &mut HashMap<&'a str, Color>,
        path: &mut Vec<&'a str>,
        errors: &mut Vec<String>,
    ) {
        color.insert(node, Color::Gray);
        path.push(node);

        if let Some(neighbors) = adj.get(node) {
            for &next in neighbors {
                match color.get(next) {
                    Some(Color::Gray) => {
                        let cycle_start = path.iter().position(|&n| n == next).unwrap();
                        let cycle: Vec<&str> = path[cycle_start..].to_vec();
                        errors.push(format!(
                            "cyclical dependency detected: {} -> {}",
                            cycle.join(" -> "),
                            next
                        ));
                    }
                    Some(Color::White) => {
                        dfs(next, adj, color, path, errors);
                    }
                    _ => {}
                }
            }
        }

        path.pop();
        color.insert(node, Color::Black);
    }

    let nodes: Vec<&str> = adj.keys().copied().collect();
    for node in nodes {
        if color[node] == Color::White {
            dfs(node, &adj, &mut color, &mut path, &mut errors);
        }
    }

    errors
}
