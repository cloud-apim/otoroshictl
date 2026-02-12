.PHONY: test test-unit test-integration test-all otoroshi-start otoroshi-stop otoroshi-logs check build fmt clean

bdev:
	cargo build

# Build
build:
	cargo build --release

# Unit tests (fast, no Docker required)
test-unit:
	cargo test --test config_tests --test smoke_tests

# Start Otoroshi for integration tests (timeout: 2 minutes)
otoroshi-start:
	docker compose -f docker/docker-compose.test.yaml up -d
	@echo "Waiting for Otoroshi to start (timeout: 2 minutes)..."
	@count=0; \
	while ! curl -sf "http://localhost:8080/.well-known/otoroshi/monitoring/health?access_key=test-health-key" > /dev/null 2>&1; do \
		count=$$((count + 1)); \
		if [ $$count -ge 40 ]; then \
			echo "Error: Otoroshi failed to start within 2 minutes"; \
			$(MAKE) otoroshi-stop; \
			exit 1; \
		fi; \
		echo "  Waiting... ($$count/40)"; \
		sleep 3; \
	done
	@echo "Otoroshi is ready!"

# Stop Otoroshi
otoroshi-stop:
	docker compose -f docker/docker-compose.test.yaml down -v

# View Otoroshi logs
otoroshi-logs:
	docker compose -f docker/docker-compose.test.yaml logs -f

# Integration tests (requires Otoroshi to be running)
test-integration:
	cargo test --test '*' -- --ignored --test-threads=1

# Full test workflow (starts Otoroshi, runs tests, stops Otoroshi)
test-all: build otoroshi-start
	cargo test --test '*' -- --ignored --test-threads=1 || ($(MAKE) otoroshi-stop && exit 1)
	$(MAKE) otoroshi-stop

# Alias for unit tests
test: test-unit

# Lint and format checks
check:
	cargo fmt -- --check || echo "Warning: formatting issues found (run 'make fmt' to fix)"
	cargo clippy || echo "Warning: clippy issues found"
	cargo test --test config_tests --test smoke_tests

# Format code
fmt:
	cargo fmt

# Clean build artifacts and stop Otoroshi
clean:
	cargo clean
	$(MAKE) otoroshi-stop 2>/dev/null || true
