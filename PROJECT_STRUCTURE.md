# Leptos Motion Project Structure

This document outlines the organized structure of the Leptos Motion project.

## 📁 Root Directory

```
leptos-motion/
├── README.md                 # Main project README
├── CHANGELOG.md             # Version history and changes
├── Cargo.toml               # Rust workspace configuration
├── Cargo.lock               # Dependency lock file
├── LICENSE                  # Project license
├── Makefile                 # Build and development commands
├── rust-toolchain.toml      # Rust toolchain specification
├── deny.toml                # Cargo deny configuration
├── flake.nix                # Nix development environment
├── package.json             # Node.js dependencies
├── playwright.config.ts     # E2E testing configuration
├── .envrc                   # Environment configuration
├── .gitignore               # Git ignore rules
├── .pnpmrc                  # pnpm configuration
```

## 🗂️ Core Directories

### **`crates/`** - Rust Library Crates
```
crates/
├── leptos-motion/           # Main library crate
├── leptos-motion-core/      # Core animation engine
├── leptos-motion-dom/       # DOM integration
├── leptos-motion-gestures/  # Gesture detection
├── leptos-motion-layout/    # Layout animations
├── leptos-motion-scroll/    # Scroll animations
└── leptos-motion-macros/    # Procedural macros
```

### **`examples/`** - Example Applications
```
examples/
├── basic-animations/        # Basic animation examples
├── showcase/                # Feature showcase
├── e-commerce-gallery/      # E-commerce example
├── dashboard-app/           # Dashboard example
├── mobile-app/              # Mobile app example
├── advanced-gestures/       # Gesture examples
├── layout-animations/       # Layout animation examples
└── scroll-effects/          # Scroll animation examples
```

### **`tests/`** - Testing Infrastructure
```
tests/
├── component-mounting.spec.ts
├── debug.spec.ts
├── event-handler-alternatives.spec.ts
├── event-handler-logic-debug.spec.ts
├── interactive-elements-debug.spec.ts
├── leptos-compatibility.spec.ts
├── leptos-dom-reconciliation.spec.ts
├── manual-event-handler-workaround.spec.ts
├── reality-check.spec.ts
├── test-app.html            # Test application
├── e2e/                     # End-to-end tests
├── integration/             # Integration tests
├── performance/             # Performance tests
├── unit/                    # Unit tests
└── visual/                  # Visual regression tests
```

## 📚 Documentation (`docs/`)

### **`docs/releases/`** - Release Documentation
```
docs/releases/
├── RELEASE_NOTES.md         # Detailed release notes
└── RELEASE_SUMMARY.md       # Release summary and metrics
```

### **`docs/guides/`** - User Guides
```
docs/guides/
├── getting_started.md       # Getting started guide
├── advanced_features.md     # Advanced features guide
├── performance.md           # Performance optimization
└── testing_guide.md         # Testing guide
```

### **`docs/api/`** - API Documentation
```
docs/api/
├── api_reference.md         # API reference
├── design.md                # Design decisions
└── implementation_plan.md   # Implementation details
```

### **`docs/examples/`** - Example Documentation
```
docs/examples/
├── INDEX.md                 # Examples index
├── testing_strategy.md      # Testing strategy
└── release_checklist.md     # Release process
```

### **`docs/book/`** - Book Documentation
```
docs/book/                   # Comprehensive documentation book
```

### **Development Documentation**
```
docs/
├── CONTRIBUTING.md          # Contribution guidelines
├── DEVELOPMENT.md           # Development setup
├── SECURITY.md              # Security policy
├── GITHUB_RELEASE.md        # GitHub release process
├── ROADMAP.md               # Project roadmap
└── PROJECT_INDEX.md         # Project overview
```

## 🎨 Assets (`assets/`)

### **`assets/screenshots/`** - Test Screenshots
```
assets/screenshots/
├── test-step1-*.png         # Test step screenshots
├── test-step2-*.png
├── test-step3-*.png
├── test-step4-*.png
└── test-step5-*.png
```

### **`assets/debug-screenshots/`** - Debug Screenshots
```
assets/debug-screenshots/
├── debug-step1-*.png        # Debug step screenshots
├── debug-step2-*.png
├── debug-step3-*.png
├── debug-step4-*.png
└── debug-step5-*.png
```

### **`assets/test-results/`** - Test Results
```
assets/test-results/          # Playwright test results
```

### **`assets/icons/`** - Project Icons
```
assets/icons/                 # Project icon files
```

### **`assets/logos/`** - Project Logos
```
assets/logos/                 # Project logo files
```

## 🛠️ Development Tools

### **`scripts/`** - Development Scripts
```
scripts/
├── test-all.sh              # Run all tests
└── test-quality.sh          # Run quality checks
```

### **`website/`** - Project Website
```
website/                     # Project website source
```

### **`.github/`** - GitHub Configuration
```
.github/                     # GitHub Actions and templates
```

## 📦 Build and Dependencies

### **`target/`** - Build Output
```
target/                      # Rust build artifacts
```

### **`node_modules/`** - Node.js Dependencies
```
node_modules/                # Node.js packages
```

### **`playwright-report/`** - Test Reports
```
playwright-report/           # Playwright test reports
```

## 🔧 Configuration Files

- **`.direnv/`** - direnv configuration
- **`.claude/`** - Claude AI configuration
- **`pnpm-lock.yaml`** - pnpm lock file
- **`flake.lock`** - Nix flake lock file

## 📋 File Organization Principles

### **1. Clear Separation of Concerns**
- **Code**: All Rust code in `crates/`
- **Examples**: All examples in `examples/`
- **Tests**: All tests in `tests/`
- **Documentation**: All docs in `docs/`
- **Assets**: All media in `assets/`

### **2. Logical Grouping**
- **Releases**: All release-related docs in `docs/releases/`
- **Guides**: User-facing documentation in `docs/guides/`
- **API**: Technical documentation in `docs/api/`
- **Screenshots**: Organized by test step and debug status

### **3. Easy Navigation**
- **README.md** at root for quick start
- **CHANGELOG.md** at root for version history
- **PROJECT_STRUCTURE.md** for navigation help
- Clear folder names and organization

### **4. Development Workflow**
- **Scripts** in `scripts/` for common tasks
- **Configuration** files at root level
- **Build artifacts** in `target/`
- **Dependencies** properly managed

## 🚀 Benefits of This Organization

1. **Cleaner Root Directory**: Only essential files visible
2. **Logical Grouping**: Related files are together
3. **Easy Navigation**: Clear folder structure
4. **Better Maintenance**: Organized for long-term development
5. **Professional Appearance**: Clean, organized project structure
6. **Easier Onboarding**: New contributors can find things quickly

## 📝 Maintenance Notes

- **Screenshots**: Automatically organized by test step
- **Debug Files**: Separated from regular test files
- **Documentation**: Categorized by purpose and audience
- **Assets**: Ready for future expansion (icons, logos, etc.)

This organization makes the project much more professional and easier to navigate while maintaining all the functionality and information.
