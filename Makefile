SHELL := /usr/bin/env bash

.PHONY: help release clean

help:
	@echo "Available targets:\n  release VERSION=x.y.z  Build release artifacts into dist/\n  clean                  Remove dist/"

release:
	@if [[ -z "$$VERSION" ]]; then \
		echo "ERROR: VERSION is required. Usage: make release VERSION=x.y.z"; \
		exit 1; \
	fi
	@chmod +x tools/release.sh
	@./tools/release.sh "$$VERSION"

clean:
	rm -rf dist
