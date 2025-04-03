use ::sql_fingerprint as upstream_fingerprint;
use pyo3::prelude::*;
use sqlparser::dialect::dialect_from_str;

#[pyfunction]
#[pyo3(signature = (sql, *, dialect = None))]
fn fingerprint_one(sql: String, dialect: Option<String>) -> PyResult<String> {
    let parsed_dialect = parse_dialect(dialect)?;
    Ok(upstream_fingerprint::fingerprint_one(
        sql.as_str(),
        parsed_dialect.as_deref(),
    ))
}

#[pyfunction]
#[pyo3(signature = (sql, *, dialect = None))]
fn fingerprint_many(sql: Vec<String>, dialect: Option<String>) -> PyResult<Vec<String>> {
    let parsed_dialect = parse_dialect(dialect)?;
    let sql_slices: Vec<&str> = sql.iter().map(|s| s.as_str()).collect();
    Ok(upstream_fingerprint::fingerprint_many(
        sql_slices,
        parsed_dialect.as_deref(),
    ))
}

fn parse_dialect(
    dialect_str: Option<String>,
) -> PyResult<Option<Box<dyn sqlparser::dialect::Dialect>>> {
    let dialect_name = dialect_str.unwrap_or_else(|| "generic".to_string());
    let dialect = dialect_from_str(&dialect_name);
    match dialect {
        Some(d) => Ok(Some(d)),
        None => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Invalid SQL dialect: {}",
            dialect_name
        ))),
    }
}

#[pymodule]
fn sql_fingerprint(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fingerprint_one, m)?)?;
    m.add_function(wrap_pyfunction!(fingerprint_many, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_one() {
        let sql = "SELECT * FROM users";
        let result: String = fingerprint_one(sql.to_string(), None).unwrap();
        assert_eq!(result, "SELECT * FROM users");
    }

    #[test]
    fn test_fingerprint_one_dialect() {
        let sql = "SELECT * FROM users";
        let result: String =
            fingerprint_one(sql.to_string(), Some("postgresql".to_string())).unwrap();
        assert_eq!(result, "SELECT * FROM users");
    }

    #[test]
    fn test_fingerprint_many() {
        let sqls = vec!["SELECT * FROM users", "SELECT * FROM orders"];
        let result: Vec<String> =
            fingerprint_many(sqls.iter().map(|s| s.to_string()).collect(), None).unwrap();
        assert_eq!(result, vec!["SELECT * FROM users", "SELECT * FROM orders"]);
    }

    #[test]
    fn test_fingerprint_many_dialect() {
        let sqls = vec!["SELECT * FROM users", "SELECT * FROM orders"];
        let result: Vec<String> = fingerprint_many(
            sqls.iter().map(|s| s.to_string()).collect(),
            Some("postgresql".to_string()),
        )
        .unwrap();
        assert_eq!(result, vec!["SELECT * FROM users", "SELECT * FROM orders"]);
    }
}
