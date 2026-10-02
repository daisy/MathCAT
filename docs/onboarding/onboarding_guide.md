# MathCAT Contributor Onboarding Guide

## 1. Welcome to MathCAT

### Welcome Message

- What is MathCAT?
- Why accessible mathematics matters
- The impact of MathCAT in screen readers and assistive technologies
- A brief history of the project
- Project goals and guiding principles

### Who This Guide Is For

- Developers
- Translators and localization contributors
- Braille experts
- Accessibility specialists
- Testers and QA contributors
- Technical writers and documentation contributors
- Organizations integrating MathCAT

### What You Can Expect

- How to get set up
- How to find work
- How to communicate with the community
- Paths for different contributor types

---

## 2. Getting Connected

### Community and Communication

#### GitHub

- Repository overview
- Issues
- Discussions
- Pull Requests
- Labels and project organization

#### Key People and Roles

- Maintainers
- Reviewers
- Community contributors
- Release coordinators

#### Communication Channels

- GitHub Discussions
- DAISY Consortium channels
- Mailing lists
- Meetings and working groups

#### Community Expectations

- Code of Conduct
- Inclusive communication
- Accessibility-first mindset
- Respectful review practices

#### First Week Checklist

- Star/watch repository
- Read contributor documents
- Introduce yourself
- Review open issues
- Set up development environment

---

## 3. Understanding the Project

### High-Level Architecture

#### Major Components

- MathML parsing and cleanup
- Intent inference
- Speech generation
- Braille generation
- Navigation subsystem
- Language rule system
- Platform/API bindings

#### Repository Tour

Suggested overview of major directories:

- /docs
- /src
- /Rules
- /tests
- /BrailleDocs
- /PythonScripts

#### How MathCAT Works

Simple workflow diagram:

```
MathML
  ↓
Cleanup
  ↓
Intent Analysis
  ↓
Speech / Braille Generation
  ↓
Navigation Support
```

---

## 4. Development Environment Setup

### Prerequisites

#### Required Tools

- Git
- Rust toolchain
- Cargo
- Editor recommendations

#### Building the Project

- Clone repository
- Build instructions
- Running tests
- Verifying installation

#### Development Workflow

- Branch strategy
- Making changes
- Submitting Pull Requests
- CI expectations

---

## 5. Choose Your Contribution Path

### [Path A: Core Rust Development](paths/core-rust-development.md)

#### Best For

- Rust developers
- Assistive technology developers

#### Learn

- Core architecture
- Processing pipeline
- Rule engine
- Navigation engine

#### First Tasks

- Good first issues
- Bug fixes
- Unit tests

#### Key Files and Folders

- src/
- tests/

---

### [Path B: Speech Rule Development](paths/speech-rule-development.md)

#### Best For

- Linguists
- Accessibility experts
- Math educators

#### What You'll Work On

- ClearSpeak improvements
- SimpleSpeak improvements
- Pronunciation enhancements

#### Learn

- Speech rule format
- Rule organization
- Validation process

#### Recommended Starter Tasks

- Speech fixes
- Terminology improvements
- Edge-case handling

#### Key Resources

- Rules documentation
- Existing language examples

---

### [Path C: Translation & Localization](paths/translation-localization.md)

#### Best For

- Native speakers
- Translators

#### What You'll Work On

- New languages
- Improvements to existing languages
- Terminology review

#### Translation Workflow

- Understanding rule files
- Testing speech output
- Reviewing terminology

#### Checklist for New Language Contributors

---

### [Path D: Braille Development](paths/braille-development.md)

#### Best For

- Braille specialists
- Accessibility professionals

#### What You'll Work On

- Nemeth enhancements
- UEB Technical support
- Additional braille codes

#### Key Concepts

- Math braille standards
- Testing methodology
- Verification practices

#### Recommended First Contributions

---

### [Path E: API and Integration Development](paths/api-integration.md)

#### Best For

- Python developers
- C/C++ developers
- Java developers
- Web developers

#### Existing Bindings

- Python
- C/C++
- Java
- WebAssembly

#### Areas for Contribution

- API improvements
- Documentation
- Sample applications
- Integration testing

---

### [Path F: Testing and Quality Assurance](paths/testing-qa.md)

#### Best For

- New contributors
- Accessibility testers
- Power users

#### Types of Testing

- Speech output validation
- Braille validation
- Navigation testing
- Regression testing

#### Testing Workflow

- Reproducing bugs
- Creating test cases
- Reporting issues

#### Good First Contributions

- Expanding test coverage
- Verifying fixes

---

### [Path G: Documentation](paths/documentation.md)

#### Best For

- Technical writers
- Educators
- New contributors

#### Areas Needing Documentation

- User documentation
- Developer documentation
- Tutorials
- Examples

#### Contribution Process

---

## 6. Working with Issues

### Finding Good First Issues

#### Label Guide

- good first issue
- bug
- enhancement
- documentation
- help wanted

#### Claiming Work

#### When to Ask Questions

#### Issue Discussion Etiquette

---

## 7. Pull Request Process

### Before Submitting

- Run tests
- Check formatting
- Verify accessibility implications
- Update documentation

### During Review

- Responding to feedback
- Updating commits
- Review expectations

### After Merge

- Release process overview
- Contributor recognition

---

## 8. Accessibility and Quality Principles

### Design Philosophy

#### Accessibility First

#### Language Quality

#### Mathematical Accuracy

#### Backward Compatibility

#### User Experience

---

## 9. Resources

### Essential Reading

- Project README
- Developer documentation
- User documentation
- Braille documentation

### Reference Materials

- MathML resources
- ClearSpeak guidance
- Braille standards

### Helpful Tools

- Screen readers
- Braille displays
- MathML editors
- Testing utilities

---

## 10. Contributor Journey Map

```
Interested in MathCAT
  ↓
Join Community
  ↓
Set Up Environment
  ↓
Choose Contribution Area
  ↓
Complete First Issue
  ↓
Submit Pull Request
  ↓
Become Regular Contributor
  ↓
Mentor Others
```

### Appendix A: Glossary

- MathML
- ClearSpeak
- SimpleSpeak
- Nemeth
- UEB Technical
- Intent Inference
- Navigation Node

### Appendix B: New Contributor Checklist

A one-page checklist covering:

- Read README
- Join communication channels
- Set up environment
- Run tests
- Identify a first issue
- Make first contribution
- Submit first PR

---

This structure separates onboarding by contributor type while keeping a common welcome and community section up front. It scales well as MathCAT grows and makes it easy for newcomers to jump directly to the area most relevant to them.
