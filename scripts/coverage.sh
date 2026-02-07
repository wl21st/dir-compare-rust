#!/bin/bash
# Run test coverage report using cargo-tarpaulin

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🧪 Running test coverage report..."

# Check if tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null && ! cargo tarpaulin --version &> /dev/null; then
    echo -e "${YELLOW}⚠️  cargo-tarpaulin not found. Installing...${NC}"
    cargo install cargo-tarpaulin
fi

# Create coverage directory
mkdir -p target/coverage

# Run coverage
echo "📊 Generating coverage report..."
cargo tarpaulin \
    --workspace \
    --out Html \
    --out Xml \
    --out Lcov \
    --output-dir target/coverage \
    --timeout 300 \
    --verbose

# Get the coverage percentage
if [ -f "target/coverage/cobertura.xml" ]; then
    COVERAGE=$(grep -o 'line-rate="[0-9.]*"' target/coverage/cobertura.xml | head -1 | sed 's/line-rate="\([0-9.]*\)"/\1/')
    if [ -n "$COVERAGE" ]; then
        COVERAGE_PERCENT=$(echo "$COVERAGE * 100" | bc -l 2>/dev/null || echo "N/A")
        echo -e "${GREEN}✅ Coverage: ${COVERAGE_PERCENT}%${NC}"
    fi
fi

echo ""
echo -e "${GREEN}✅ Coverage report generated!${NC}"
echo "📄 HTML Report: file://$(pwd)/target/coverage/tarpaulin-report.html"
echo "📄 XML Report: $(pwd)/target/coverage/cobertura.xml"
echo "📄 LCOV Report: $(pwd)/target/coverage/lcov.info"
echo ""
echo "Open the HTML report in your browser to view detailed coverage."
