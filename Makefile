# Command to start the server
.PHONY: server
server:
	cargo run --bin server

# Command to build the web assets
.PHONY: web
web:
	cd web && npm run dev
    
build: 
	cargo build --release && cd web && npm run build