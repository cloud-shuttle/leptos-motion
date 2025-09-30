# Trunk Server Configuration Design

## Problem Statement

The Leptos Motion demo testing is blocked by trunk server startup failures caused by environment variable conflicts. The `NO_COLOR=1` environment variable causes trunk to receive an invalid value for the `--no-color` flag, preventing demo servers from starting.

## Root Cause Analysis

### Environment Variable Conflict

**Issue**: `NO_COLOR=1` environment variable is interpreted incorrectly by trunk.

**Evidence**:
```bash
$ env | grep NO_COLOR
NO_COLOR=1

$ trunk serve --address 127.0.0.1 --port 3000
error: invalid value '1' for '--no-color'
  [possible values: true, false]
```

**Root Cause**: Trunk expects `--no-color` to be a boolean flag, but receives "1" as a positional argument.

### Trunk Color Handling

Trunk supports color output control through:
1. `--color` flag with values: `auto`, `always`, `never`
2. `NO_COLOR` environment variable support
3. `TRUNK_COLOR` environment variable

**Current Behavior**: Environment `NO_COLOR=1` conflicts with trunk's flag parsing.

## Solution Design

### Environment Variable Override Strategy

**Primary Solution**: Use environment variable overrides to neutralize the conflict.

```bash
# Override problematic environment variables
NO_COLOR= TRUNK_COLOR=auto trunk serve --address 127.0.0.1 --port 3000
```

**Why This Works**:
- `NO_COLOR=` (empty) disables the NO_COLOR environment variable
- `TRUNK_COLOR=auto` explicitly sets trunk's color mode
- Trunk ignores the problematic NO_COLOR value

### Configuration Architecture

#### 1. Local Development
```bash
# scripts/dev-server.sh
#!/bin/bash
export NO_COLOR=
export TRUNK_COLOR=auto
trunk serve --address 127.0.0.1 --port 3000
```

#### 2. CI/CD Environment
```yaml
# .github/workflows/test.yml
- name: Run demo tests
  env:
    NO_COLOR: ""
    TRUNK_COLOR: auto
  run: |
    cd examples/comprehensive-showcase
    trunk serve --address 127.0.0.1 --port 3000 &
    sleep 10
    npm test
```

#### 3. Playwright Configuration
```typescript
// playwright.config.ts
webServer: {
  command: 'NO_COLOR= TRUNK_COLOR=auto trunk serve --address 127.0.0.1 --port 3000',
  url: 'http://localhost:3000',
  reuseExistingServer: !process.env.CI,
  timeout: 120 * 1000,
}
```

### Implementation Strategy

#### Phase 1: Immediate Fix

1. **Update all trunk commands** to use environment overrides
2. **Test manual server startup** with the fix
3. **Verify Playwright integration**

#### Phase 2: Permanent Solution

1. **Create wrapper scripts** for consistent server startup
2. **Update documentation** with environment requirements
3. **Add environment validation** to prevent future issues

### Wrapper Script Implementation

```bash
#!/bin/bash
# scripts/start-trunk-server.sh

# Override problematic environment variables
export NO_COLOR=
export TRUNK_COLOR=auto

# Parse arguments
ADDRESS="127.0.0.1"
PORT="3000"
DIRECTORY="."

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
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Navigate to directory
cd "$DIRECTORY" || exit 1

# Start trunk server with clean environment
exec trunk serve --address "$ADDRESS" --port "$PORT"
```

### Testing Strategy

#### Unit Tests for Configuration
```rust
#[cfg(test)]
mod trunk_config_tests {
    use std::process::Command;

    #[test]
    fn test_trunk_server_startup() {
        let output = Command::new("bash")
            .args(&["-c", "NO_COLOR= TRUNK_COLOR=auto trunk --version"])
            .output()
            .expect("Failed to execute trunk");

        assert!(output.status.success());
    }

    #[test]
    fn test_environment_override() {
        // Test that NO_COLOR override works
        let output = Command::new("bash")
            .args(&["-c", "NO_COLOR=1 TRUNK_COLOR=auto trunk --help"])
            .output()
            .expect("Failed to execute trunk");

        // Should not contain the "invalid value" error
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("invalid value"));
    }
}
```

#### Integration Tests
```typescript
// playwright.config.ts integration test
test.describe('Server Configuration', () => {
  test('trunk server starts with environment overrides', async ({ request }) => {
    // Test that server starts and responds
    const response = await request.get('http://localhost:3000/health');
    expect(response.status()).toBe(200);
  });

  test('server serves WASM files correctly', async ({ request }) => {
    const response = await request.get('http://localhost:3000/comprehensive_showcase_bg.wasm');
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toBe('application/wasm');
  });
});
```

### Documentation Updates

#### Developer Setup Guide
```markdown
# Development Environment Setup

## Environment Variables

The following environment variables may conflict with trunk:

```bash
# Problematic variables (avoid or override)
NO_COLOR=1  # Conflicts with trunk's color handling

# Recommended overrides
export NO_COLOR=
export TRUNK_COLOR=auto
```

## Starting Demo Servers

Use the provided wrapper script for consistent server startup:

```bash
# Preferred method
./scripts/start-trunk-server.sh --directory examples/comprehensive-showcase

# Manual method (with overrides)
NO_COLOR= TRUNK_COLOR=auto trunk serve --address 127.0.0.1 --port 3000
```
```

#### Troubleshooting Guide
```markdown
# Troubleshooting Trunk Server Issues

## Error: "invalid value '1' for '--no-color'"

**Cause**: NO_COLOR environment variable conflict

**Solution**:
```bash
# Override the environment variable
NO_COLOR= TRUNK_COLOR=auto trunk serve --address 127.0.0.1 --port 3000
```

## Error: "Server failed to start"

**Cause**: Port already in use or network issues

**Solutions**:
1. Kill existing trunk processes: `pkill -f trunk`
2. Use different port: `--port 3001`
3. Check network: `curl http://localhost:3000`

## Error: "WASM files not found"

**Cause**: Build incomplete or path issues

**Solutions**:
1. Ensure build completed: `trunk build`
2. Check dist directory exists
3. Verify Trunk.toml configuration
```

### Risk Assessment

#### Low Risk
- Environment variable overrides are safe
- No code changes required
- Backward compatible

#### Medium Risk
- Environment-dependent behavior
- CI/CD pipeline modifications needed
- Documentation updates required

#### Mitigation Strategies
1. **Comprehensive Testing**: Test across different environments
2. **Documentation**: Clear setup instructions
3. **Automation**: Wrapper scripts prevent manual errors
4. **Monitoring**: CI/CD validation of server startup

### Future Considerations

#### Long-term Solutions
1. **Trunk Enhancement**: Request better environment variable handling
2. **Containerization**: Docker-based testing environment
3. **Alternative Tools**: Evaluate wasm-server-runner or other options

#### Monitoring & Maintenance
1. **Environment Validation**: Check for conflicting variables
2. **Automated Health Checks**: Server startup validation
3. **Performance Monitoring**: Track server startup times

### Implementation Checklist

- [ ] Create wrapper script for trunk server startup
- [ ] Update all CI/CD configurations with environment overrides
- [ ] Update Playwright configuration
- [ ] Test server startup in multiple environments
- [ ] Update developer documentation
- [ ] Add troubleshooting guides
- [ ] Create automated tests for server configuration
- [ ] Establish monitoring for server health

### Success Criteria

1. **Server Startup**: All demo servers start without manual intervention
2. **Environment Compatibility**: Works across different development environments
3. **CI/CD Integration**: Automated testing pipeline works reliably
4. **Documentation**: Clear setup and troubleshooting procedures
5. **Monitoring**: Server health and performance monitoring in place

### Timeline

- **Week 1**: Implement environment overrides and wrapper scripts
- **Week 2**: Update CI/CD and Playwright configurations
- **Week 3**: Testing and validation across environments
- **Week 4**: Documentation and monitoring implementation

This design provides a comprehensive solution to the trunk server configuration issues, ensuring reliable demo testing and development workflow. 🚀
