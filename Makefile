default:
	@echo "Building (Release)...";
	cargo rustc --release --color=always -p schemsearch-cli -- -C target-feature=+avx2

sql:
	@echo "Building (Release)...";
	cargo rustc --release --color=always -p schemsearch-cli --features sql -- -C target-feature=+avx2

debug:
	@echo "Building (Debug)...";
	cargo build -p schemsearch-cli

install: default
	@echo "Installing...";
	install -Dm755 target/release/schemsearch-cli /usr/bin/schemsearch

uninstall:
	@echo "Uninstalling...";
	rm -f /usr/bin/schemsearch

java:
	@echo "Building Java...";
	@echo "WARNING: This is WORK IN PROGRESS!";
	javac SchemSearch.java

clean:
	@echo "Cleaning...";
	cargo clean