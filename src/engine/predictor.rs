use std::collections::HashMap;

use crate::db::query::{DbQuery, Prediction};
use crate::engine::chains::{detect_chains, transition_probability, ChainType, CommandChain};
use crate::engine::context::ShellContext;
use crate::engine::learner::extract_binary;

/// Context-aware command predictor.
pub struct Predictor {
    db: DbQuery,
}

impl Predictor {
    /// Create a new predictor backed by the given database.
    pub fn new(db: DbQuery) -> Self {
        Predictor { db }
    }

    /// Get the next command predictions based on current context.
    pub fn predict(&self, context: &ShellContext, count: usize) -> Result<Vec<Prediction>, String> {
        let mut predictions = self.db.predict_commands(context, count * 3)?;

        // Apply re-ranking based on chain detection
        if !context.recent_commands.is_empty() {
            let chains = detect_chains(&context.recent_commands);
            for chain in &chains {
                self.apply_chain_boost(&mut predictions, chain);
            }
        }

        // Apply recency boost
        self.apply_recency_boost(&mut predictions, context);

        // Sort by score and deduplicate
        predictions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        let mut seen = std::collections::HashSet::new();
        predictions.retain(|p| {
            if seen.contains(&p.command) {
                false
            } else {
                seen.insert(p.command.clone());
                true
            }
        });

        predictions.truncate(count);
        Ok(predictions)
    }

    /// Get the single best next command prediction.
    pub fn predict_best(&self, context: &ShellContext) -> Result<Option<Prediction>, String> {
        let predictions = self.predict(context, 1)?;
        Ok(predictions.into_iter().next())
    }

    /// Get predictions for completing a partial command.
    pub fn complete(&self, prefix: &str, context: &ShellContext, count: usize) -> Result<Vec<Prediction>, String> {
        let predictions = self.predict(context, count * 2)?;

        // Filter by prefix
        let filtered: Vec<Prediction> = predictions
            .into_iter()
            .filter(|p| p.command.starts_with(prefix) || p.binary.starts_with(prefix))
            .collect();

        if filtered.len() >= count {
            return Ok(filtered.into_iter().take(count).collect());
        }

        // If we don't have enough prefix matches, fall back to DB prefix search
        let mut db_predictions = self.db_prefix_search(prefix, context, count)?;

        // Merge without duplicates
        let mut seen: std::collections::HashSet<String> = filtered.iter().map(|p| p.command.clone()).collect();
        let mut result = filtered;
        for p in db_predictions {
            if !seen.contains(&p.command) {
                seen.insert(p.command.clone());
                result.push(p);
            }
            if result.len() >= count {
                break;
            }
        }

        Ok(result)
    }

    /// Search the database for commands matching a prefix.
    fn db_prefix_search(&self, prefix: &str, _context: &ShellContext, limit: usize) -> Result<Vec<Prediction>, String> {
        let conn = self.db.connection();
        let pattern = format!("{}%", prefix);

        // First try commands starting with the prefix
        let mut stmt = conn
            .prepare(
                "SELECT command, binary_name, frequency as score
                 FROM commands
                 WHERE command LIKE ?1
                 ORDER BY frequency DESC
                 LIMIT ?2",
            )
            .map_err(|e| format!("Query error: {}", e))?;

        let results = stmt
            .query_map(rusqlite::params![pattern, limit as i64], |row| {
                Ok(Prediction {
                    command: row.get(0)?,
                    binary: row.get(1)?,
                    score: row.get::<_, f64>(2)?,
                    reason: "prefix match".to_string(),
                })
            })
            .map_err(|e| format!("Query map error: {}", e))?;

        results.collect::<Result<Vec<_>, _>>().map_err(|e| format!("Collect error: {}", e))
    }

    /// Boost predictions that continue observed chain patterns.
    fn apply_chain_boost(&self, predictions: &mut Vec<Prediction>, chain: &CommandChain) {
        if chain.commands.is_empty() {
            return;
        }

        let last_cmd = chain.commands.last().unwrap();
        let last_bin = extract_binary(last_cmd);

        for pred in predictions.iter_mut() {
            let pred_bin = extract_binary(&pred.command);

            match chain.chain_type {
                ChainType::Pipe => {
                    if is_pipe_relevant(last_bin.as_str(), pred_bin.as_str()) {
                        pred.score *= 2.0;
                        pred.reason = format!("pipe chain: {} -> {}", last_cmd, pred.command);
                    }
                }
                ChainType::Conditional => {
                    if last_bin == pred_bin || is_conditional_pair(last_bin.as_str(), pred_bin.as_str()) {
                        pred.score *= 1.8;
                        pred.reason = format!("conditional chain: {} -> {}", last_cmd, pred.command);
                    }
                }
                ChainType::Repeat => {
                    if pred.command == *last_cmd {
                        pred.score *= 1.5;
                        pred.reason = format!("repeat: {}", pred.command);
                    }
                }
                ChainType::Related => {
                    if last_bin == pred_bin {
                        pred.score *= 1.6;
                        pred.reason = format!("related: {} -> {}", last_bin, pred.command);
                    }
                }
                ChainType::Sequential => {
                    let prob = transition_probability(last_cmd, &pred.command);
                    pred.score *= 1.0 + prob;
                    pred.reason = format!("sequential: {} -> {}", last_cmd, pred.command);
                }
            }
        }
    }

    /// Boost recently used commands.
    fn apply_recency_boost(&self, predictions: &mut Vec<Prediction>, context: &ShellContext) {
        let recent: Vec<String> = context
            .recent_commands
            .iter()
            .map(|c| extract_binary(c))
            .collect();

        for pred in predictions.iter_mut() {
            let pred_bin = extract_binary(&pred.command);
            if recent.contains(&pred_bin) {
                pred.score *= 1.3;
            }
        }
    }

    /// Generate a ranked list of completion candidates for the install script.
    pub fn generate_completions(&self, prefix: &str, count: usize) -> Result<Vec<String>, String> {
        let conn = self.db.connection();
        let pattern = format!("{}%", prefix);

        let mut stmt = conn
            .prepare(
                "SELECT command FROM commands WHERE command LIKE ?1 ORDER BY frequency DESC LIMIT ?2",
            )
            .map_err(|e| format!("Query error: {}", e))?;

        let results = stmt
            .query_map(rusqlite::params![pattern, count as i64], |row| {
                row.get::<_, String>(0)
            })
            .map_err(|e| format!("Query map error: {}", e))?;

        results.collect::<Result<Vec<_>, _>>().map_err(|e| format!("Collect error: {}", e))
    }
}

/// Check if two binaries are pipe-relevant.
fn is_pipe_relevant(prev: &str, next: &str) -> bool {
    matches!(
        (prev, next),
        ("grep" | "find" | "ps" | "ls" | "cat" | "sort" | "uniq" | "head" | "tail", _)
            | (_, "grep" | "awk" | "sed" | "sort" | "uniq" | "head" | "tail" | "less" | "more" | "xargs")
    )
}

/// Check if two binaries form a common conditional pair.
fn is_conditional_pair(prev: &str, next: &str) -> bool {
    matches!(
        (prev, next),
        ("make", "make") | ("cargo", "cargo") | ("npm", "npm") | ("yarn", "yarn") | ("cd", "ls")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::query::DbQuery;
    use crate::engine::learner::{HistoryEntry, ShellType};
    use std::path::PathBuf;

    fn setup_test_db() -> DbQuery {
        let db = DbQuery::new_in_memory();

        let entries = vec![
            HistoryEntry {
                command: "git status".to_string(),
                binary: "git".to_string(),
                timestamp: 1620000000,
                cwd: Some("/project".to_string()),
                exit_status: Some(0),
                duration: None,
                shell_type: ShellType::Bash,
            },
            HistoryEntry {
                command: "git diff".to_string(),
                binary: "git".to_string(),
                timestamp: 1620000001,
                cwd: Some("/project".to_string()),
                exit_status: Some(0),
                duration: None,
                shell_type: ShellType::Bash,
            },
            HistoryEntry {
                command: "ls".to_string(),
                binary: "ls".to_string(),
                timestamp: 1620000002,
                cwd: Some("/project".to_string()),
                exit_status: Some(0),
                duration: None,
                shell_type: ShellType::Bash,
            },
            HistoryEntry {
                command: "cd /tmp".to_string(),
                binary: "cd".to_string(),
                timestamp: 1620000003,
                cwd: Some("/project".to_string()),
                exit_status: Some(0),
                duration: None,
                shell_type: ShellType::Bash,
            },
        ];

        db.insert_history_entries(&entries).unwrap();
        db.update_features(&entries).unwrap();
        db.update_chains(&entries).unwrap();

        db
    }

    #[test]
    fn test_predict_returns_results() {
        let db = setup_test_db();
        let predictor = Predictor::new(db);

        let context = ShellContext::new(
            PathBuf::from("/project"),
            14,
            2,
            vec!["git status".to_string()],
        );

        let predictions = predictor.predict(&context, 5).unwrap();
        assert!(!predictions.is_empty());
    }

    #[test]
    fn test_predict_best() {
        let db = setup_test_db();
        let predictor = Predictor::new(db);

        let context = ShellContext::new(
            PathBuf::from("/project"),
            14,
            2,
            vec!["git status".to_string()],
        );

        let best = predictor.predict_best(&context).unwrap();
        assert!(best.is_some());
    }

    #[test]
    fn test_complete_with_prefix() {
        let db = setup_test_db();
        let predictor = Predictor::new(db);

        let context = ShellContext::new(
            PathBuf::from("/project"),
            14,
            2,
            vec![],
        );

        let completions = predictor.complete("git", &context, 5).unwrap();
        assert!(!completions.is_empty());
        assert!(completions.iter().all(|c| c.command.starts_with("git") || c.binary.starts_with("git")));
    }

    #[test]
    fn test_generate_completions() {
        let db = setup_test_db();
        let predictor = Predictor::new(db);

        let completions = predictor.generate_completions("git", 5).unwrap();
        assert!(!completions.is_empty());
    }
}
