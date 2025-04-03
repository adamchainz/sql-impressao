use ::sql_fingerprint;
use pyo3::prelude::*;
use sqlparser::dialect::dialect_from_str;

#[pyfunction]
#[pyo3(signature = (sql, *, dialect = None))]
fn fingerprint_one(sql: String, dialect: Option<String>) -> PyResult<String> {
    let parsed_dialect = parse_dialect(dialect)?;
    Ok(sql_fingerprint::fingerprint_one(
        sql.as_str(),
        parsed_dialect.as_deref(),
    ))
}

#[pyfunction]
#[pyo3(signature = (sql, *, dialect = None))]
fn fingerprint_many(sql: Vec<String>, dialect: Option<String>) -> PyResult<Vec<String>> {
    let parsed_dialect = parse_dialect(dialect)?;
    let sql_slices: Vec<&str> = sql.iter().map(|s| s.as_str()).collect();
    Ok(sql_fingerprint::fingerprint_many(
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
fn sql_impressao(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fingerprint_one, m)?)?;
    m.add_function(wrap_pyfunction!(fingerprint_many, m)?)?;
    Ok(())
}
