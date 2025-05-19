from __future__ import annotations

from sql_impressao import fingerprint_many, fingerprint_one


def test_fingerprint_one():
    sql = "SELECT a, b FROM c WHERE d = e ORDER BY f"
    result = fingerprint_one(sql)
    assert result == "SELECT ... FROM c WHERE ... ORDER BY ..."


def test_fingerprint_one_dialect():
    sql = "SELECT a, b FROM c WHERE d = e ORDER BY f"
    result = fingerprint_one(sql, dialect="postgresql")
    assert result == "SELECT ... FROM c WHERE ... ORDER BY ..."


def test_fingerprint_many():
    sqls = ["SELECT a FROM b", "SELECT c FROM d"]
    result = fingerprint_many(sqls)
    assert result == ["SELECT ... FROM b", "SELECT ... FROM d"]


def test_fingerprint_many_dialect():
    sqls = ["SELECT a FROM b", "SELECT c FROM d"]
    result = fingerprint_many(sqls, dialect="postgresql")
    assert result == ["SELECT ... FROM b", "SELECT ... FROM d"]
