SRC=./git-sync.sh
PREFIX=usr

install:
	cp $(SRC) $(PREFIX)/usr/git-sync
	
.PHONY: install
