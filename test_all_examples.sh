#!/bin/bash

# Test script to run all E++ examples and report results

echo "=== E++ Examples Test Runner ==="
echo "Testing all examples in the examples/ directory..."
echo

# Counter for passed/failed tests
PASSED=0
FAILED=0
TOTAL=0

# Function to test a single file
test_file() {
    local file="$1"
    local basename=$(basename "$file" .eppx)
    
    echo -n "Testing $basename... "
    TOTAL=$((TOTAL + 1))
    
    # Run the E++ compiler
    if cargo run --quiet -- run "$file" > /tmp/eppx_test_output.txt 2>&1; then
        echo "✅ PASSED"
        PASSED=$((PASSED + 1))
    else
        echo "❌ FAILED"
        FAILED=$((FAILED + 1))
        echo "  Error output:"
        cat /tmp/eppx_test_output.txt | head -10 | sed 's/^/    /'
        echo
    fi
}

# Test all .eppx files in examples directory
echo "Found the following test files:"
find examples/ -name "*.eppx" | sort

echo
echo "Running tests..."
echo

# Test each file
for file in $(find examples/ -name "*.eppx" | sort); do
    test_file "$file"
done

echo
echo "=== Test Results ==="
echo "Total tests: $TOTAL"
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo "Success rate: $(echo "scale=1; $PASSED * 100 / $TOTAL" | bc -l)%"

if [ $FAILED -eq 0 ]; then
    echo "🎉 All tests passed!"
    exit 0
else
    echo "❌ Some tests failed"
    exit 1
fi
