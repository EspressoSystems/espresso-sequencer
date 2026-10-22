# claude-gh uv project; just runs module recipes in this directory.

fmt *args:
    ruff format . {{args}}

lint *args:
    ruff check . {{args}}

check *args:
    uv sync --locked
    ty check --python .venv . {{args}}

test *args:
    uv run --locked pytest {{args}}
