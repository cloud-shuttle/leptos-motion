#!/bin/bash

# Trunk Server Startup Wrapper Script
# Handles environment variable conflicts and provides consistent server startup

set -e

# Override problematic environment variables
export NO_COLOR=
export TRUNK_COLOR=auto

# Default values
ADDRESS="127.0.0.1"
PORT="3000"
DIRECTORY="."
OPEN=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --address)
      ADDRESS="$2"
      shift 2
      ;;
    --port)
      PORT="$2"
      shift 2
      ;;
    --directory)
      DIRECTORY="$2"
      shift 2
      ;;
    --open)
      OPEN=true
      shift
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [--address ADDRESS] [--port PORT] [--directory DIR] [--open]"
      exit 1
      ;;
  esac
done

# Navigate to directory if specified
if [ "$DIRECTORY" != "." ]; then
  cd "$DIRECTORY" || {
    echo "❌ Failed to navigate to directory: $DIRECTORY"
    exit 1
  }
fi

# Build command
CMD="trunk serve --address $ADDRESS --port $PORT"
if [ "$OPEN" = true ]; then
  CMD="$CMD --open"
fi

echo "🚀 Starting Trunk Server..."
echo "📍 Directory: $(pwd)"
echo "🌐 Address: $ADDRESS:$PORT"
echo "🎨 Color mode: $TRUNK_COLOR"
echo "📝 Command: $CMD"
echo ""

# Execute trunk with clean environment
exec $CMD