NAME := mikudos_rust_service
SERVICE_VERSION := 0.0.1
PORT := 50051
GREETER_PATH := greeter/

.PHONY: proto
proto:
	protoc --rust_out=./src/${GREETER_PATH} --grpc_out=./src/${GREETER_PATH} --plugin=protoc-gen-grpc=`which grpc_rust_plugin` ./proto/${GREETER_PATH}*.proto

.PHONY: build
build: proto
	cargo build

.PHONY: dev
dev:
	DEBUG=* cargo run