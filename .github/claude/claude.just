# claude-gh uv project; just runs module recipes in this directory.

fmt *args:
    uv run --locked ruff format . {{args}}

lint *args:
    uv run --locked ruff check . {{args}}

check *args:
    uv run --locked ty check . {{args}}

test *args:
    uv run --locked pytest {{args}}
