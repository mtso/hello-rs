.PHONY: package

compile:
	rustc src/main.rs

package:
	docker build -t mtso/hello-rs:0.1 .
