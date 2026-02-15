SHELL := /usr/bin/env bash
PREFIX ?= /usr/local
BINDIR := $(PREFIX)/bin

.PHONY: help release clean install uninstall remove

help:
	@echo "Available targets:"
	@echo "  install                Build and install lsf to $(BINDIR)"
	@echo "  uninstall/remove       Remove lsf from $(BINDIR)"
	@echo "  release VERSION=x.y.z  Build release artifacts into dist/"
	@echo "  clean                  Remove dist/ and target/"

install:
	cargo build --release
	install -Dm755 target/release/lsf "$(DESTDIR)$(BINDIR)/lsf"
	@echo "lsf installed to $(BINDIR)/lsf"

uninstall:
	rm -f "$(DESTDIR)$(BINDIR)/lsf"
	@echo "lsf removed from $(BINDIR)/lsf"

remove: uninstall

release:
	@if [[ -z "$$VERSION" ]]; then \
		echo "ERROR: VERSION is required. Usage: make release VERSION=x.y.z"; \
		exit 1; \
	fi
	@chmod +x tools/release.sh
	@./tools/release.sh "$$VERSION"

clean:
	rm -rf dist
	cargo clean
