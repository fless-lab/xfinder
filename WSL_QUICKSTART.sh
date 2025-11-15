#!/bin/bash
# WSL Quickstart - xfinder Assist Me setup
# À exécuter dans WSL Ubuntu après avoir cloné le repo

set -e

echo "🚀 xfinder WSL Setup"
echo "===================="

# 1. Installer Rust
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "✅ Rust already installed"
fi

# 2. Installer Python
echo "📦 Installing Python..."
sudo apt update
sudo apt install -y python3 python3-pip python3-venv build-essential

# 3. Créer venv
echo "📦 Creating Python venv..."
python3 -m venv .venv
source .venv/bin/activate

# 4. Installer dépendances Python
echo "📦 Installing Python packages..."
pip install --upgrade pip
pip install torch --index-url https://download.pytorch.org/whl/cpu
pip install sentence-transformers leann

# 5. Tester
echo "🧪 Testing Python packages..."
python -c "from sentence_transformers import SentenceTransformer; print('✅ sentence-transformers OK')"
python -c "import leann; print('✅ LEANN OK')"

# 6. Configurer PyO3
echo "⚙️  Configuring PyO3..."
export PYO3_PYTHON=$(pwd)/.venv/bin/python3
echo "export PYO3_PYTHON=$(pwd)/.venv/bin/python3" >> ~/.bashrc

# 7. Compiler
echo "🔨 Building xfinder..."
cargo build --release

echo ""
echo "✅ Setup complete!"
echo ""
echo "To run xfinder:"
echo "  1. source .venv/bin/activate"
echo "  2. ./target/release/xfinder"
echo ""
echo "Next steps:"
echo "  - Click '🤖 Assist Me' tab"
echo "  - Click '🚀 Initialiser Assist Me' in sidebar"
echo "  - Click '📚 Indexer maintenant'"
echo "  - Type a question and search!"
