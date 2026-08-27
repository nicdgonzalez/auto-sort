SORT_KEY := src/main/java/io/github/nicdgonzalez/autosort/SortKey.java

.PHONY: generate clean

generate: build/libs/auto-sort.jar

clean:
	rm -r ./.venv ./data "./$(SORT_KEY)"
	cargo clean

.venv:
	python3 -m venv .venv
	$@/bin/python3 -m pip install \
		requests==2.3.0 \
		lxml==5.3.2 \
		bs4==0.0.2

data/materials.txt: .venv
	mkdir "$(@D)"
	./scripts/materials.py > "$@"

target/release/auto-sort:
	cargo build --release

$(SORT_KEY): target/release/auto-sort ./data/materials.txt
	"$<" generate-code \
		--input $(word 2,$^) \
		--profile ./profiles/Default.toml \
		> "$@"


build/libs/auto-sort.jar: $(SORT_KEY)
	./gradlew build
