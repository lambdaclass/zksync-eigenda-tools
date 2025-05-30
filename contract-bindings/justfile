# default recipe to display help information
default:
  @just --list

# install dependencies
deps:
    #!/usr/bin/env sh
    if ! command -v mise > /dev/null 2>&1; then
        echo "Error: mise is not installed or not in your PATH"
        echo "Please install mise first:"
        echo "  https://mise.jdx.dev/getting-started.html"
        echo ""
        echo "Quick install:"
        echo "  curl https://mise.run | sh"
        exit 1
    fi
    mise install

build *args='':
  cargo build --workspace $@
