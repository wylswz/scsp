generate:
	protoc --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative ./scsp.proto

.PHONY: build
build:
	mkdir -p target
	go build -o target/main main/main.go
	chmod +x target/main

clean:
	rm -rf target