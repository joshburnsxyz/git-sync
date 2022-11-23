SRC=./git-sync.sh
OUT=./git-sync.sh
PREFIX=usr

install:
	cp $(SRC) $(OUT)
	chmod a+x $(OUT)
	cp $(OUT) $(PREFIX)/usr/$(OUT)
	
.PHONY: install
