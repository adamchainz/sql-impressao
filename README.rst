========================
sql-fingerprint (Python)
========================

.. image:: https://img.shields.io/github/actions/workflow/status/adamchainz/sql-fingerprint-python/main.yml.svg?branch=main&style=for-the-badge
   :target: https://github.com/adamchainz/sql-fingerprint-python/actions?workflow=CI

.. image:: https://img.shields.io/pypi/v/sql-fingerprint-python.svg?style=for-the-badge
   :target: https://pypi.org/project/sql-fingerprint-python/

.. image:: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white&style=for-the-badge
   :target: https://github.com/pre-commit/pre-commit
   :alt: pre-commit

Python bindings for `sql-fingerprint <https://github.com/adamchainz/sql-fingerprint>`__, a SQL fingerprinter.
A quick example:

.. code-block:: python-console

    >>> from sql_fingerprint import fingerprint_one
    >>> fingerprint_one('SELECT a, b FROM cheeses WHERE origin = "France" ORDER BY age DESC')
    'SELECT ... FROM cheeses WHERE ... ORDER BY ...'

----

**Improve your Django and Git skills** with `my books <https://adamj.eu/books/>`__.

----

Installation
============

With **pip**:

.. code-block:: sh

    python -m pip install sql-fingerprint

Python 3.9 to 3.13 supported.

Usage
=====

``fingerprint_one(sql: str) -> str``
------------------------------------

Generate the fingerprint for a single SQL statement.

Example:

.. code-block:: python

    from sql_fingerprint import fingerprint_one

    sql = 'SELECT a, b FROM cheeses WHERE origin = "France" ORDER BY age DESC'
    fingerprint = fingerprint_one(sql)
    assert fingerprint == "SELECT ... FROM cheeses WHERE ... ORDER BY ..."

``fingerprint_many(sql: list[str]) -> list[str]``
-------------------------------------------------

Generate the fingerprints for a list of SQL statements.
Doing so for a batch of strings allows sharing some state, such as savepoint ID aliases.

Example:

.. code-block:: python

    from sql_fingerprint import fingerprint_many

    sqls = ["SELECT a, b FROM c", "SELECT b, c FROM d"]
    fingerprints = fingerprint_many(sqls)
    assert fingerprints == ["SELECT ... FROM c", "SELECT ... FROM d"]
