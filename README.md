# gitte.rs
[![Crates.io](https://img.shields.io/crates/v/gitters)](https://crates.io/crates/gitters)
[![License](https://img.shields.io/crates/l/gitters)](https://github.com/GennaroBiondi/gitte-rs/blob/main/LICENSE)
[![Downloads](https://img.shields.io/crates/d/gitters)](https://crates.io/crates/gitters)

Rust tool to quickly check if git commits and branches follow common conventions
(successor of [glitter](https://github.com/GennaroBiondi/glitter))

# Usage

when run without arguments, the program will automatically check every git commit
and git branch in the repository in the current directory.

```bash
gitte
```

the program also supports a path that points to the directory of the repo as an argument
```bash
gitte ~/my_repo
```

if any commits or branches don't follow the specification of [Conventional Commits](https://www.conventionalcommits.org/)
or [Conventional Branch](https://conventional-branch.github.io/) then it will print a warning, providing the reason it was flagged.

# All possible warnings
## Commits
### Mismatched parenthesis in scope
```bash
Mismatched parenthesis in scope!
```

means the parenthesis in your commit's scope are mismatched.

.e.g: "type)scope(description", "type(scope description".

### No scope or colon found!
```bash
No scope or colon found!
```

means the program found no ':' colon symbol or scope.

.e.g: "type description", "description"

### No description found!
```bash
No description found!
```

means the program didn't find the following patterns:
- "): "
- ": "
in your commit message

.e.g: "type : description", "type(scope) description"

### Scope is not valid ascii!
```bash
Scope is not valid ascii!
```

means your commit scope contains non-ASCII characters.

.e.g: "fix(ümlaut): description"

### Scope contains either a space, tab, or new line character!
```bash
Scope contains either a space, tab, or new line character!
```

means your commit scope contains whitespace.

.e.g: "fix(my scope): description"

### Scope contains invalid character
```bash
Scope contains invalid character
```

means your commit scope contains one of the following: `(`, `)`, `:`, `@`

.e.g: "fix(my(scope): description", "fix(my@scope): description"

### Commit Type not recognized: [string]
```bash
Commit Type not recognized: '*'
```

means the program couldn't match your commit type with the following:
- "fix" (Fix)
- "feat" (Feat)
- "build" (Build)
- "chore" (Chore)
- "ci" (Ci)
- "docs" (Docs)
- "style" (Style)
- "refactor" (Refactor)
- "perf" (Perf)
- "test" (Test)

**Note**:
This isn't really an error, as the Conventional Commits specification allows custom types,
and not a specific list

## Branches
### No '/' separator found in branch name!
```bash
No '/' separator found in branch name!
```

means your branch name doesn't contain a '/' separator between the type and description. Trunk branches ("main", "master", "develop") are exempt from this rule.

.e.g: "feat", "chore"

### No type found in branch name!
```bash
No type found in branch name!
```

means the part before the '/' separator is empty.

.e.g: "/description"

### Branch Type not recognized: [string]
```bash
Branch Type not recognized: '*'
```

means the program couldn't match your branch type with the following:
- "fix" (Fix)
- "bugfix" (Bugfix)
- "hotfix" (Hotfix)
- "feat" / "feature" (Feat)
- "release" (Release)
- "chore" (Chore)
- "ai" (Ai)
- "claude" (Claude)
- "codex" (Codex)
- "copilot" (Copilot)
- "cursor" (Cursor)

**Note**:
This isn't really an error, as the Conventional Branches specification allows custom types,
and not a specific list

### No description found in branch name!
```bash
No description found in branch name!
```

means there's nothing after the '/' separator.

.e.g: "feat/", "chore/"

### Found uppercase character in branch name description!
```bash
Found uppercase character in branch name description!
```

means your branch description contains an uppercase character. Descriptions must be lowercase.

.e.g: "feat/My-Feature", "fix/Bug-Fix"

### Found underscore in branch name description!
```bash
Found underscore in branch name description!
```

means your branch description contains an underscore. Use hyphens instead.

.e.g: "feat/my_feature", "fix/bug_fix"

### Found leading or trailing hyphen in branch name description!
```bash
Found leading or trailing hyphen in branch name description!
```

means your branch description starts or ends with a hyphen.

.e.g: "feat/-my-feature", "feat/my-feature-"

### Found consecutive hyphens in branch name description!
```bash
Found consecutive hyphens in branch name description!
```

means your branch description contains two or more consecutive hyphens.

.e.g: "feat/my--feature", "fix/bug--fix"
