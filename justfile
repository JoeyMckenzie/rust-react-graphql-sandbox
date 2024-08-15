set shell := ["pwsh.exe", "-c"]

API_DIR := "api"
UI_DIR := "ui"

default: dev

dev:
    @echo "Starting development servers..."
    pwsh -Command "Start-Process pwsh -ArgumentList '-NoExit', '-Command', 'Set-Location {{API_DIR}}; cargo run'"
    pwsh -Command "Start-Process pwsh -ArgumentList '-NoExit', '-Command', 'Set-Location {{UI_DIR}}; pnpm run dev'"
    @echo "Both servers are running. Close the windows to stop the servers."
    @pwsh -Command "pause"

dev-ui:
    cd hop-query-ui/ && pnpm run dev

dev-api:
    cd hop-query-api/ && cargo watch -x run