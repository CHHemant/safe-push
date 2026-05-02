# Examples

This directory contains example files for testing SafePush.

## test-secrets.txt

A sample file containing placeholder secrets to demonstrate SafePush scanning.
Replace the placeholder values with patterns matching real secrets to test detection.

```bash
# Create a test file with placeholder patterns
cat > test-secrets.txt << 'EOF'
aws_access_key_id = REPLACE_WITH_EXAMPLE_KEY
aws_secret_access_key = REPLACE_WITH_EXAMPLE_SECRET
github_token = ghp_REPLACE36CHARPLACEHOLDERVALUE000000
stripe_key = STRIPE_LIVE_KEY_PLACEHOLDER
contact = test@example.com
EOF

# Run SafePush to detect secrets
safe-push scan test-secrets.txt

# Sanitize the file
safe-push sanitize test-secrets.txt test-secrets-safe.txt
```
