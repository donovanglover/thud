all: dev

.PHONY: install
install:
	@makepkg -sci
	@rm -fv *.pkg.tar.zst

.PHONY: dev
dev:
	@updpkgsums
	@makepkg -sci
	@rm -fv *.pkg.tar.zst
