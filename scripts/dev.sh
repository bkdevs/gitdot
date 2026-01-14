#!/bin/bash

# Script to start both Next.js frontend and Rust backend in tmux split panes

SESSION_NAME="gitdot"

tmux has-session -t $SESSION_NAME 2>/dev/null
if [ $? -eq 0 ]; then
    echo "Killing existing tmux session '$SESSION_NAME'..."
    tmux kill-session -t $SESSION_NAME
fi

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"

echo "Starting tmux session '$SESSION_NAME'..."

tmux new-session -d -s $SESSION_NAME -c "$PROJECT_ROOT/frontend" -n "frontend" "pnpm run dev"
tmux new-window -t $SESSION_NAME -c "$PROJECT_ROOT/backend" -n "backend" "cargo run"

tmux select-window -t $SESSION_NAME:0
tmux attach-session -t $SESSION_NAME
