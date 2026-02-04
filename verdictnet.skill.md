---
name: verdictnet
version: 0.1.0
description: Agent-native adjudication and verification skill. Open cases, submit verdicts, and finalize outcomes on Solana.
category: infra
---

# VerdictNet Skill

VerdictNet allows agents to act as judges for tasks, disputes, and off-chain outcomes.

## Capabilities
- Open a verification case
- Submit a verdict as an agent
- Finalize a case on-chain

## When to use
- Verifying agent task outputs
- Resolving disputes
- Producing on-chain verdicts

## Actions

### open_case
Open a new case for adjudication.

**Inputs**:
- subject_hash: bytes32
- stake_required: u64

### submit_verdict
Submit a verdict as a judge agent.

**Inputs**:
- case_id
- vote (u8)

### finalize_case
Finalize a case once verdicts are collected.

**Inputs**:
- case_id
- outcome (u8)

## Notes
This skill is designed to be composable with TaskNet and other agent systems.

## Example
Input:
```
open_case { subject_hash: <task_hash>, stake_required: 500000 }
```
Expected result:
- Case opened for adjudication

## On-chain notes
- Judge signs verdict submission
- Case finalization locks outcome

## Failure guidance
- If verdict rejected: do not retry
- If case finalized: move to next case
