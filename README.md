# DuckDB primes extension

Lets you select primes in duckdb:


    D select * from primes(20);
    ┌───────┐
    │  no   │
    │ int64 │
    ├───────┤
    │     2 │
    │     3 │
    │     5 │
    │     7 │
    │    11 │
    │    13 │
    │    17 │
    │    19 │
    └───────┘

Fun, huh? Mostly testing stuff though...
