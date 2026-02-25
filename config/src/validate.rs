use std::collections::{HashMap, HashSet};

use super::error::CiConfigError;
use crate::ci::CiConfig;

pub fn validate_ci_config(config: &CiConfig) -> Result<(), CiConfigError> {
    let mut errors = Vec::new();

    errors.extend(check_non_empty(config));
    errors.extend(check_duplicate_task_names(config));
    errors.extend(check_duplicate_build_triggers(config));
    errors.extend(check_empty_build_task_lists(config));
    errors.extend(check_empty_commands(config));
    errors.extend(check_task_references(config));
    errors.extend(check_duplicate_task_refs_in_build(config));
    errors.extend(check_no_orphaned_tasks(config));
    errors.extend(check_waits_for_unknown_tasks(config));
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

fn check_duplicate_task_names(config: &CiConfig) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for task in &config.tasks {
        if !seen.insert(task.name.as_str()) {
            errors.push(format!("duplicate task name '{}'", task.name));
        }
    }
    errors
}

fn check_duplicate_build_triggers(config: &CiConfig) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for build in &config.builds {
        let trigger: String = build.trigger.clone().into();
        if !seen.insert(trigger.clone()) {
            errors.push(format!("duplicate build trigger '{trigger}'"));
        }
    }
    errors
}

fn check_empty_build_task_lists(config: &CiConfig) -> Vec<String> {
    config
        .builds
        .iter()
        .enumerate()
        .filter(|(_, b)| b.tasks.is_empty())
        .map(|(i, _)| format!("builds[{i}] has an empty task list"))
        .collect()
}

fn check_empty_commands(config: &CiConfig) -> Vec<String> {
    config
        .tasks
        .iter()
        .filter(|t| t.command.trim().is_empty())
        .map(|t| format!("task '{}' has an empty command", t.name))
        .collect()
}

fn check_duplicate_task_refs_in_build(config: &CiConfig) -> Vec<String> {
    let mut errors = Vec::new();
    for (i, build) in config.builds.iter().enumerate() {
        let mut seen = HashSet::new();
        for task_ref in &build.tasks {
            if !seen.insert(task_ref.as_str()) {
                errors.push(format!(
                    "builds[{i}] lists task '{task_ref}' more than once"
                ));
            }
        }
    }
    errors
}

fn check_waits_for_unknown_tasks(config: &CiConfig) -> Vec<String> {
    let defined: HashSet<&str> = config.tasks.iter().map(|t| t.name.as_str()).collect();
    let mut errors = Vec::new();
    for task in &config.tasks {
        if let Some(deps) = &task.waits_for {
            for dep in deps {
                if !defined.contains(dep.as_str()) {
                    errors.push(format!(
                        "task '{}' waits_for unknown task '{dep}'",
                        task.name
                    ));
                }
            }
        }
    }
    errors
}

fn check_dag(config: &CiConfig) -> Vec<String> {
    let defined: HashSet<&str> = config.tasks.iter().map(|t| t.name.as_str()).collect();

    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for task in &config.tasks {
        adj.entry(task.name.as_str()).or_default();
        if let Some(deps) = &task.waits_for {
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

#[cfg(test)]
mod tests {
    use crate::ci::CiConfig;

    fn valid_toml() -> &'static str {
        r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            [[tasks]]
            name = "test"
            command = "cargo test"
        "#
    }

    fn assert_validation_errors(toml: &str, expected: &[&str]) {
        let err = CiConfig::new(toml).unwrap_err();
        let msg = err.to_string();
        for expected_substr in expected {
            assert!(
                msg.contains(expected_substr),
                "expected error to contain {expected_substr:?}, got:\n{msg}"
            );
        }
    }

    // --- Valid configs ---

    #[test]
    fn valid_minimal_config() {
        let config = CiConfig::new(valid_toml()).unwrap();
        assert_eq!(config.tasks.len(), 1);
        assert_eq!(config.builds.len(), 1);
    }

    #[test]
    fn valid_multi_task_with_deps() {
        let toml = r#"
            [[builds]]
            trigger = "push_to_main"
            tasks = ["build", "test", "lint"]

            [[tasks]]
            name = "lint"
            command = "cargo clippy"

            [[tasks]]
            name = "build"
            command = "cargo build"
            waits_for = ["lint"]

            [[tasks]]
            name = "test"
            command = "cargo test"
            waits_for = ["build"]
        "#;
        let config = CiConfig::new(toml).unwrap();
        assert_eq!(config.tasks.len(), 3);
    }

    #[test]
    fn valid_multiple_builds_sharing_tasks() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["lint", "test"]

            [[builds]]
            trigger = "push_to_main"
            tasks = ["lint", "test", "deploy"]

            [[tasks]]
            name = "lint"
            command = "cargo clippy"

            [[tasks]]
            name = "test"
            command = "cargo test"
            waits_for = ["lint"]

            [[tasks]]
            name = "deploy"
            command = "./deploy.sh"
            waits_for = ["test"]
        "#;
        CiConfig::new(toml).unwrap();
    }

    #[test]
    fn valid_diamond_dependency() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["a", "b", "c", "d"]

            [[tasks]]
            name = "a"
            command = "echo a"

            [[tasks]]
            name = "b"
            command = "echo b"
            waits_for = ["a"]

            [[tasks]]
            name = "c"
            command = "echo c"
            waits_for = ["a"]

            [[tasks]]
            name = "d"
            command = "echo d"
            waits_for = ["b", "c"]
        "#;
        CiConfig::new(toml).unwrap();
    }

    // --- Parse errors ---

    #[test]
    fn parse_error_invalid_toml() {
        let err = CiConfig::new("not valid toml {{{").unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("failed to parse config"),
            "expected parse error, got:\n{msg}"
        );
    }

    #[test]
    fn parse_error_missing_required_field() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            [[tasks]]
            name = "test"
        "#;
        let err = CiConfig::new(toml).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("failed to parse config"),
            "expected parse error for missing `command`, got:\n{msg}"
        );
    }

    #[test]
    fn parse_error_invalid_trigger() {
        let toml = r#"
            [[builds]]
            trigger = "on_commit"
            tasks = ["test"]

            [[tasks]]
            name = "test"
            command = "cargo test"
        "#;
        let err = CiConfig::new(toml).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("failed to parse config"),
            "expected parse error for bad trigger, got:\n{msg}"
        );
    }

    // --- Empty builds / tasks ---

    #[test]
    fn empty_builds() {
        let toml = r#"
            builds = []

            [[tasks]]
            name = "test"
            command = "cargo test"
        "#;
        assert_validation_errors(toml, &["builds cannot be empty"]);
    }

    #[test]
    fn empty_tasks() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            tasks = []
        "#;
        let err = CiConfig::new(toml).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("tasks cannot be empty") || msg.contains("failed to parse config"),
            "expected empty tasks or parse error, got:\n{msg}"
        );
    }

    #[test]
    fn both_empty() {
        let toml = r#"
            builds = []
            tasks = []
        "#;
        assert_validation_errors(toml, &["builds cannot be empty", "tasks cannot be empty"]);
    }

    // --- Unknown task references ---

    #[test]
    fn build_references_unknown_task() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test", "missing_task"]

            [[tasks]]
            name = "test"
            command = "cargo test"
        "#;
        assert_validation_errors(toml, &["builds[0] references unknown task 'missing_task'"]);
    }

    #[test]
    fn multiple_builds_reference_unknown_tasks() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["ghost1"]

            [[builds]]
            trigger = "push_to_main"
            tasks = ["ghost2"]

            [[tasks]]
            name = "real"
            command = "echo hi"
        "#;
        assert_validation_errors(
            toml,
            &[
                "builds[0] references unknown task 'ghost1'",
                "builds[1] references unknown task 'ghost2'",
            ],
        );
    }

    // --- Orphaned tasks ---

    #[test]
    fn orphaned_task() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            [[tasks]]
            name = "test"
            command = "cargo test"

            [[tasks]]
            name = "orphan"
            command = "echo lonely"
        "#;
        assert_validation_errors(toml, &["task 'orphan' is not referenced by any build"]);
    }

    #[test]
    fn multiple_orphaned_tasks() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["used"]

            [[tasks]]
            name = "used"
            command = "echo used"

            [[tasks]]
            name = "orphan1"
            command = "echo a"

            [[tasks]]
            name = "orphan2"
            command = "echo b"
        "#;
        assert_validation_errors(
            toml,
            &[
                "task 'orphan1' is not referenced by any build",
                "task 'orphan2' is not referenced by any build",
            ],
        );
    }

    // --- Cycle detection ---

    #[test]
    fn self_cycle() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["a"]

            [[tasks]]
            name = "a"
            command = "echo a"
            waits_for = ["a"]
        "#;
        assert_validation_errors(toml, &["cyclical dependency detected: a -> a"]);
    }

    #[test]
    fn two_node_cycle() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["a", "b"]

            [[tasks]]
            name = "a"
            command = "echo a"
            waits_for = ["b"]

            [[tasks]]
            name = "b"
            command = "echo b"
            waits_for = ["a"]
        "#;
        assert_validation_errors(toml, &["cyclical dependency detected"]);
    }

    #[test]
    fn three_node_cycle() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["a", "b", "c"]

            [[tasks]]
            name = "a"
            command = "echo a"
            waits_for = ["c"]

            [[tasks]]
            name = "b"
            command = "echo b"
            waits_for = ["a"]

            [[tasks]]
            name = "c"
            command = "echo c"
            waits_for = ["b"]
        "#;
        assert_validation_errors(toml, &["cyclical dependency detected"]);
    }

    #[test]
    fn cycle_in_subset_of_tasks() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["ok", "a", "b"]

            [[tasks]]
            name = "ok"
            command = "echo ok"

            [[tasks]]
            name = "a"
            command = "echo a"
            waits_for = ["b"]

            [[tasks]]
            name = "b"
            command = "echo b"
            waits_for = ["a"]
        "#;
        assert_validation_errors(toml, &["cyclical dependency detected"]);
    }

    // --- Multiple errors at once ---

    #[test]
    fn unknown_ref_and_orphan_together() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["nonexistent"]

            [[tasks]]
            name = "orphan"
            command = "echo orphan"
        "#;
        assert_validation_errors(
            toml,
            &[
                "builds[0] references unknown task 'nonexistent'",
                "task 'orphan' is not referenced by any build",
            ],
        );
    }

    #[test]
    fn error_message_formatting() {
        let toml = r#"
            builds = []
            tasks = []
        "#;
        let err = CiConfig::new(toml).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.starts_with("config validation failed:\n"),
            "expected 'config validation failed:' prefix, got:\n{msg}"
        );
        assert!(
            msg.contains("  - builds cannot be empty"),
            "expected bullet-formatted error, got:\n{msg}"
        );
        assert!(
            msg.contains("  - tasks cannot be empty"),
            "expected bullet-formatted error, got:\n{msg}"
        );
    }

    // --- waits_for references unknown task ---

    #[test]
    fn waits_for_references_undefined_task() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["a"]

            [[tasks]]
            name = "a"
            command = "echo a"
            waits_for = ["nonexistent"]
        "#;
        assert_validation_errors(toml, &["task 'a' waits_for unknown task 'nonexistent'"]);
    }

    #[test]
    fn waits_for_multiple_undefined_tasks() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["a"]

            [[tasks]]
            name = "a"
            command = "echo a"
            waits_for = ["ghost1", "ghost2"]
        "#;
        assert_validation_errors(
            toml,
            &[
                "task 'a' waits_for unknown task 'ghost1'",
                "task 'a' waits_for unknown task 'ghost2'",
            ],
        );
    }

    // --- Duplicate task names ---

    #[test]
    fn duplicate_task_names() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            [[tasks]]
            name = "test"
            command = "cargo test"

            [[tasks]]
            name = "test"
            command = "cargo test --release"
        "#;
        assert_validation_errors(toml, &["duplicate task name 'test'"]);
    }

    #[test]
    fn multiple_duplicate_task_names() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["a", "b"]

            [[tasks]]
            name = "a"
            command = "echo a"

            [[tasks]]
            name = "b"
            command = "echo b"

            [[tasks]]
            name = "a"
            command = "echo a2"

            [[tasks]]
            name = "b"
            command = "echo b2"
        "#;
        assert_validation_errors(
            toml,
            &["duplicate task name 'a'", "duplicate task name 'b'"],
        );
    }

    // --- Duplicate build triggers ---

    #[test]
    fn duplicate_build_triggers() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            [[tasks]]
            name = "test"
            command = "cargo test"
        "#;
        assert_validation_errors(toml, &["duplicate build trigger 'pull_request'"]);
    }

    // --- Empty build task lists ---

    #[test]
    fn empty_build_task_list() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = []

            [[tasks]]
            name = "test"
            command = "cargo test"
        "#;
        assert_validation_errors(toml, &["builds[0] has an empty task list"]);
    }

    #[test]
    fn multiple_builds_with_empty_task_lists() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = []

            [[builds]]
            trigger = "push_to_main"
            tasks = []

            [[tasks]]
            name = "test"
            command = "cargo test"
        "#;
        assert_validation_errors(
            toml,
            &[
                "builds[0] has an empty task list",
                "builds[1] has an empty task list",
            ],
        );
    }

    // --- Empty commands ---

    #[test]
    fn empty_command() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            [[tasks]]
            name = "test"
            command = ""
        "#;
        assert_validation_errors(toml, &["task 'test' has an empty command"]);
    }

    #[test]
    fn whitespace_only_command() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test"]

            [[tasks]]
            name = "test"
            command = "   "
        "#;
        assert_validation_errors(toml, &["task 'test' has an empty command"]);
    }

    // --- Duplicate task refs in a build ---

    #[test]
    fn duplicate_task_ref_in_build() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["test", "test"]

            [[tasks]]
            name = "test"
            command = "cargo test"
        "#;
        assert_validation_errors(toml, &["builds[0] lists task 'test' more than once"]);
    }

    #[test]
    fn duplicate_task_refs_across_builds() {
        let toml = r#"
            [[builds]]
            trigger = "pull_request"
            tasks = ["a", "a"]

            [[builds]]
            trigger = "push_to_main"
            tasks = ["b", "b"]

            [[tasks]]
            name = "a"
            command = "echo a"

            [[tasks]]
            name = "b"
            command = "echo b"
        "#;
        assert_validation_errors(
            toml,
            &[
                "builds[0] lists task 'a' more than once",
                "builds[1] lists task 'b' more than once",
            ],
        );
    }
}
