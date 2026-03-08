.PHONY: help fmt fmt-check test docs docs-venv check

help:
	@echo "LoopForge (meos) common targets:"
	@echo "  make fmt         - cargo fmt"
	@echo "  make fmt-check   - cargo fmt --check"
	@echo "  make test        - cargo test (workspace, locked)"
	@echo "  make docs        - mkdocs build --strict (uses .venv-docs if present)"
	@echo "  make docs-venv   - create .venv-docs and install docs deps"
	@echo "  make check       - fmt-check + test + docs"

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all --check

test:
	cargo test --workspace --locked

docs:
	@if [ -x .venv-docs/bin/python ]; then \
		.venv-docs/bin/python -m mkdocs build --strict; \
	else \
		python3 -m mkdocs build --strict; \
	fi

docs-venv:
	python3 -m venv .venv-docs
	.venv-docs/bin/pip install -r requirements-docs.txt

check: fmt-check test docs
