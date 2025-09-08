=============
sql-impressao
=============

.. image:: https://img.shields.io/github/actions/workflow/status/adamchainz/sql-impressao/main.yml.svg?branch=main&style=for-the-badge
   :target: https://github.com/adamchainz/sql-impressao/actions?workflow=CI

.. image:: https://img.shields.io/pypi/v/sql-impressao.svg?style=for-the-badge
   :target: https://pypi.org/project/sql-impressao/

.. image:: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white&style=for-the-badge
   :target: https://github.com/pre-commit/pre-commit
   :alt: pre-commit

Python bindings for `sql-fingerprint <https://github.com/adamchainz/sql-fingerprint>`__, a SQL fingerprinter.
(*Impressão digital* is Portuguese for “fingerprint”.)

A quick example:

.. code-block:: python-console

    >>> from sql_impressao import fingerprint_one
    >>> fingerprint_one('SELECT a, b FROM cheeses WHERE origin = "France" ORDER BY age DESC')
    'SELECT ... FROM cheeses WHERE ... ORDER BY ...'

----

**Improve your Django and Git skills** with `my books <https://adamj.eu/books/>`__.

----

Installation
============

With **pip**:

.. code-block:: sh

    python -m pip install sql-impressao

Python 3.9 to 3.14 supported.

Usage
=====

``fingerprint_one(sql: str, *, dialect: str | None = None) -> str``
-------------------------------------------------------------------

Generate the fingerprint for a single SQL string.

``sql`` is the SQL string to fingerprint.
It may contain multiple statements separated by semicolons.

``dialect`` is an optional string that specifies the SQL dialect to use.
Supported names include:

* ``generic`` - the default, a generic SQL dialect.
* ``mysql`` - MySQL dialect.
* ``postgresql`` or ``postgres`` - PostgreSQL dialect.
* ``sqlite`` - SQLite dialect.

See the `source of the underlying function <https://github.com/apache/datafusion-sqlparser-rs/blob/776b10afe608a88811b807ab795831d55f186ee3/src/dialect/mod.rs#L1038-L1059>`__ for a full list.

Example:

.. code-block:: python

    from sql_impressao import fingerprint_one

    sql = 'SELECT a, b FROM cheeses WHERE origin = "France" ORDER BY age DESC'
    fingerprint = fingerprint_one(sql)
    assert fingerprint == "SELECT ... FROM cheeses WHERE ... ORDER BY ..."

``fingerprint_many(sql: list[str], *, dialect: str | None = None) -> list[str]``
--------------------------------------------------------------------------------

Generate the fingerprints for a list of SQL strings.
Doing so for a batch of strings allows sharing some state, such as savepoint ID aliases.

``sql`` is a list of SQL strings to fingerprint.
Each string may contain multiple statements separated by semicolons.

``dialect`` is an optional string that specifies the SQL dialect to use, as above.

Example:

.. code-block:: python

    from sql_impressao import fingerprint_many

    sqls = ["SELECT a, b FROM c", "SELECT b, c FROM d"]
    fingerprints = fingerprint_many(sqls)
    assert fingerprints == ["SELECT ... FROM c", "SELECT ... FROM d"]
