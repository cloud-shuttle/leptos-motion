# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records for the leptos-motion project. ADRs document important architectural decisions, their context, and consequences.

## ADR Index

- [ADR-001: Package Manager - pnpm](./001-package-manager-pnpm.md)
- [ADR-002: Leptos Version Support Strategy](./002-leptos-version-support.md)
- [ADR-003: Rust Coding Practices and Version Strategy](./003-rust-coding-practices.md)
- [ADR-004: Testing Pyramid and Coverage Standards](./004-testing-strategy.md)
- [ADR-005: Playwright Testing Strategy](./005-playwright-testing.md)
- [ADR-006: Competitor Analysis and Capability Matching](./006-competitor-analysis.md)

## ADR Template

When creating a new ADR, use the following template:

```markdown
# ADR-XXX: [Title]

## Status
[Proposed | Accepted | Rejected | Deprecated | Superseded]

## Context
[Describe the context and problem statement, e.g., in free form using two to three sentences. You may want to articulate the problem in form of a question.]

## Decision
[Describe the change that we're proposing or have agreed to implement.]

## Consequences
[Describe the resulting context, after applying the decision. All consequences should be listed here, not just the "positive" ones. A particular decision may have positive, negative, and neutral consequences, but all of them affect the team and project in the future.]

## Implementation Notes
[Optional: Any specific implementation details, code examples, or migration steps.]

## References
[Optional: Links to related issues, PRs, or external resources.]
```

## Contributing

When making architectural decisions:

1. Create a new ADR using the template above
2. Update this README with the new ADR entry
3. Submit a PR for review
4. Once accepted, the ADR becomes part of our architectural documentation

## Review Process

- All ADRs must be reviewed by at least two team members
- ADRs should be discussed in team meetings when they affect multiple areas
- Deprecated ADRs should be marked as such and linked to their superseding ADRs
