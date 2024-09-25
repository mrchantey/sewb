set dotenv-load

default:
	just --list --unsorted


run example:
	cargo run --example {{example}}
