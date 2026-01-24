---
description: Review code changes and create commit following ChaseAI standards
---

# Review & Commit Workflow

## Overview

This workflow reviews changed code for quality, security, and compliance, then creates a commit if all checks pass.

**Detailed review criteria are defined in:**

- `.agent/rules/rust-coding-standards.md` - Rust rules, Cargo, safety, and performance
- `.agent/AGENTS.md` - Agent instructions and workflow rules

1. Execute [.agent/workflows/init-workflow.md](file:///Users/dmytro.zvieriev/sandbox/ChaseAI/.agent/workflows/init-workflow.md) to reset context.

## Phase 0: Automatic File Detection & Analysis

// turbo
The agent automatically detects all changed files and begins review immediately:

```bash
git status
git diff --name-only
git diff --stat
```

**Automatic Agent Actions (No User Input Required):**

1. **Detect all changed files** - Identify .rs, .ts, .md, and .toml modified files
2. **Read full content** - Load complete content of each changed file
3. **Determine file types** - Categorize by extension and purpose
4. **Apply matching rules** - Automatically select applicable rules:
   - `.rs` files → Rust standards + complexity thresholds
   - `.ts` files → TypeScript rules + type safety
   - `.md` files → Markdownlint standards
   - `.toml` files → Dependency management (exact versions)

5. **Scan all files** - Check each file against all applicable rules
6. **Collect findings** - Document all critical, high, and medium priority issues
7. **Begin Phase 1** - Automatically proceed to comprehensive code review

## Phase 1: Code Review of Changes

// turbo
Agent performs comprehensive code review by applying rules to each file:

### Automatic Rule Application

For each changed file, the agent:

1. **Reads the full file content**
2. **Identifies file type and purpose**
3. **Applies matching rule sets** from `.agent/rules/`
4. **Checks for violations** against:
   - **Cognitive Complexity** - Max 30 for Rust
   - **Dependency versions** - No `^` or `~` in package.json or Cargo.toml
   - **Safety** - No `unwrap()` in Rust library code without justification
   - **Formatting** - Rustfmt and Prettier/Lint compliance
   - **Commit size** - Maximum 4000 LOC per commit

5. **Categorizes findings**:
   - 🚨 **Critical Issues** - Must fix before commit
   - ⚠️ **High Priority Issues** - Should fix before commit
   - 💡 **Medium Priority Issues** - Nice to fix

### Step 2: Validate Code Quality

// turbo
Run automated checks:

```bash
bun run lint
bun test
```

**Required Checks:**

- [ ] Markdownlint passes
- [ ] Cargo clippy passes (no warnings)
- [ ] All Rust tests pass

## Phase 2: Prepare Commit Message

Create a proper commit message following standards:

**Format:**

```text
[TYPE] Brief description (50 chars max)

Detailed explanation (optional)
- Bullet point 1
- Bullet point 2
```

**Commit Types:** feat, fix, refactor, test, docs, style, chore, perf

## Phase 3: Create Commit

### Decision Point: All Checks Passed?

**If YES (All checks passed):**

- ✅ Code review complete with no critical/high priority issues
- ✅ All tests pass
- ✅ Linting and clippy pass
- ✅ Commit message prepared

**Then:** Proceed to automatic commit creation

### Create Commit

// turbo
Create the commit with pre-commit hooks enabled:

```bash
git add .
git commit -m "[TYPE] Description"
```

**Critical Rules:**

- ⛔ **NEVER use `--no-verify`** - Pre-commit hooks MUST run
- ✅ Always let pre-commit hooks run

## Review Summary Template

```markdown
## Commit Review Summary

### Validation Results
- [ ] Pre-commit hooks passed
- [ ] All tests pass
- [ ] No linting/clippy errors
- [ ] Commit message valid

### Critical Issues: [count]
[List or "None found ✅"]

### Recommendation
- [ ] **Approve** - Ready to merge
- [ ] **Request changes** - Must address issues before merge
```
