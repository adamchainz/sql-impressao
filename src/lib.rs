use ::sql_fingerprint as upstream_fingerprint;
use pyo3::prelude::*;

#[pyfunction]
fn fingerprint_one(sql: String) -> PyResult<String> {
    Ok(upstream_fingerprint::fingerprint_one(sql.as_str(), None))
}

#[pyfunction]
fn fingerprint_many(sql: Vec<String>) -> PyResult<Vec<String>> {
    let sql_slices: Vec<&str> = sql.iter().map(|s| s.as_str()).collect();
    Ok(upstream_fingerprint::fingerprint_many(sql_slices, None))
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
        let result: String = fingerprint_one(sql.to_string()).unwrap();
        assert_eq!(result, "SELECT * FROM users");
    }

    #[test]
    fn test_fingerprint_many() {
        let sqls = vec!["SELECT * FROM users", "SELECT * FROM orders"];
        let result: Vec<String> =
            fingerprint_many(sqls.iter().map(|s| s.to_string()).collect()).unwrap();
        assert_eq!(result, vec!["SELECT * FROM users", "SELECT * FROM orders"]);
    }
}
