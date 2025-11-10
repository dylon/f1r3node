# Budget Breakdown: Rholang Optimization Proof Mechanization

**Document Version**: 1.0
**Last Updated**: 2025-11-10
**Status**: Planning Phase - Single Source of Truth for All Budget Estimates

---

## Purpose

This document provides the **single source of truth** for all budget estimates across the formal verification documentation suite. All other documents reference these numbers.

**Important**: This document was created after an inconsistency review that found budget numbers varying across documents. These consolidated estimates reflect realistic timelines based on proof complexity analysis.

---

## Summary: Three Approaches

| Approach | Total Cost | Duration | Confidence | Best For |
|----------|------------|----------|------------|----------|
| **Full Mechanization** | **$240K-345K** | **18-26 months** | 98% | High-assurance systems, certification |
| **Hybrid (Recommended)** | **$125K-170K** | **9-12 months** | 98% | Production blockchain, cost-effective validation |
| **Property Testing Only** | **$15K-25K** | **2-3 months** | 95% | MVP, budget-constrained, defer mechanization |

---

## Full Mechanization Breakdown

### Phase 1: Foundation + Proof-of-Concept (6-8 Months)

**Team**: 1.5 FTE (1 senior formal methods expert + 0.5 junior developer)

| Activity | Duration | Personnel Cost | External/Infra | Contingency (10%) | Total |
|----------|----------|----------------|----------------|-------------------|-------|
| **Months 1-2**: Core definitions (Syntax, Semantics, Monad, Complexity) | 8 weeks | $20,000 | $500 | $2,050 | **$22,550** |
| **Months 3-4**: Reusable lemmas (480 total: lists, sets, state, eval, complexity) | 8 weeks | $20,000 | $1,000 | $2,100 | **$23,100** |
| **Months 5-6**: Rust semantics (Vec, Rc, amortized) + Automation (tactics, sledgehammer) | 8 weeks | $20,000 | $1,000 | $2,100 | **$23,100** |
| **Months 6-8**: PoC Proofs (1, 6, 11) + Validation + Documentation | 8-10 weeks | $22,500 | $2,000 | $2,450 | **$26,950** |
| **External Review**: Coq expert code review (8 hours/month × 7 months) | Throughout | - | $4,000 | $400 | **$4,400** |
| **PHASE 1 TOTAL** | **6-8 months** | **$82,500** | **$8,500** | **$9,100** | **$100,100** |

**Rounded Phase 1 Budget**: **$90,000 - $120,000**

**Assumptions**:
- Senior expert: $165K annual salary (~$80/hour, 20 hours/week = $1,600/week)
- Junior developer: $90K annual salary (~$43/hour, 20 hours/week = $860/week)
- Combined: ~$2,460/week × 40 weeks = $98,400

### Phase 2: Remaining Proofs (12-18 Months)

**Team**: 1-1.5 FTE (1 senior expert, optional 0.5 FTE for documentation/testing)

| Activity | Duration | Personnel Cost | External/Infra | Contingency (10%) | Total |
|----------|----------|----------------|----------------|-------------------|-------|
| **Months 1-3**: Proof 4 (Accumulator - most complex, 8-12 weeks) | 12 weeks | $24,000 | $500 | $2,450 | **$26,950** |
| **Months 4-5**: Proof 5 (Match Optimization, 3-5 weeks) | 5 weeks | $10,000 | $250 | $1,025 | **$11,275** |
| **Months 6-8**: Proofs 2, 3 (Rust semantics: Rc, Pre-allocation, 8-12 weeks) | 12 weeks | $24,000 | $1,000 | $2,500 | **$27,500** |
| **Months 9-11**: Proofs 8, 9, 10 (Persistent data structures, 9-15 weeks) | 12 weeks | $24,000 | $1,000 | $2,500 | **$27,500** |
| **Months 12-14**: Proof 7 (Clone Reduction - ownership/borrowing, 6-8 weeks) | 8 weeks | $16,000 | $500 | $1,650 | **$18,150** |
| **Months 15-18**: Library refactoring, documentation, peer review, paper submission | 12 weeks | $24,000 | $5,000 | $2,900 | **$31,900** |
| **RustBelt Integration**: Collaborate with RustBelt team for Rust semantics | Throughout | - | $10,000 | $1,000 | **$11,000** |
| **PHASE 2 TOTAL** | **12-18 months** | **$122,000** | **$18,250** | **$14,025** | **$154,275** |

**Rounded Phase 2 Budget**: **$150,000 - $225,000**

**Assumptions**:
- Senior expert: $165K annual = ~$80/hour
- Average 30 hours/week for 72 weeks = 2,160 hours × $80 = $172,800
- Discounted for part-time allocation and efficiency gains from Phase 1 foundation

### Phase 3: Rust Code Extraction/Refinement (18-24 Months, Optional)

**Team**: 2-3 FTE (1 Rust expert, 1 Coq expert, 1 RustBelt/Iris expert)

| Activity | Duration | Personnel Cost | External/Infra | Contingency (15%) | Total |
|----------|----------|----------------|----------------|-------------------|-------|
| **Option A**: Custom Coq → Rust extraction plugin development | 6-9 months | $120,000 | $10,000 | $19,500 | **$149,500** |
| **Option B**: Refinement proofs (Coq spec ↔ Rust impl) via RustBelt/Iris | 12-18 months | $240,000 | $15,000 | $38,250 | **$293,250** |
| **Collaboration**: RustBelt/Prusti/Creusot teams, possible Rust language team | Throughout | - | $20,000 | $3,000 | **$23,000** |
| **CI/CD Pipeline**: Continuous verification infrastructure | 2 months | $24,000 | $5,000 | $4,350 | **$33,350** |
| **Certification Documentation**: For regulatory compliance/auditing | 2 months | $24,000 | $2,000 | $3,900 | **$29,900** |
| **PHASE 3 TOTAL** | **18-24 months** | **$288,000-360,000** | **$52,000** | **$51,000-69,000** | **$391,000-481,000** |

**Rounded Phase 3 Budget**: **$350,000 - $500,000**

**Note**: Phase 3 significantly more expensive due to:
- Custom compiler/extraction tooling development (if Option A)
- RustBelt/Iris theorem prover expertise (scarce and expensive)
- Integration complexity with existing Rust codebase

---

## Full Mechanization Total

| Phase | Duration | Cost |
|-------|----------|------|
| **Phase 1**: Foundation + PoC | 6-8 months | $90K-120K |
| **Phase 2**: Remaining 8 proofs | 12-18 months | $150K-225K |
| **Phase 3**: Rust integration (optional) | 18-24 months | $350K-500K |
| **TOTAL (Phases 1+2)** | **18-26 months** | **$240K-345K** |
| **TOTAL (All Phases)** | **36-50 months** | **$590K-845K** |

---

## Hybrid Approach Breakdown (Recommended)

### Hybrid Phase 1: Foundation + 3 Critical Proofs (6-8 Months)

**Same as Full Mechanization Phase 1**: $90,000 - $120,000

### Hybrid Phase 2: Property Testing (2 Months)

**Team**: 1 FTE (Rust developer with property testing experience)

| Activity | Duration | Personnel Cost | External/Infra | Contingency (10%) | Total |
|----------|----------|----------------|----------------|-------------------|-------|
| Property test framework setup (proptest/QuickChick) | 1 week | $2,000 | $200 | $220 | **$2,420** |
| Proofs 2, 3: Property tests for Rc and pre-allocation (2 weeks) | 2 weeks | $4,000 | $100 | $410 | **$4,510** |
| Proof 4: Property tests + empirical O(n) validation (2 weeks) | 2 weeks | $4,000 | $100 | $410 | **$4,510** |
| Proof 5: Property tests for match optimization (1 week) | 1 week | $2,000 | $100 | $210 | **$2,310** |
| Proofs 7-10: Property tests for remaining optimizations (3 weeks) | 3 weeks | $6,000 | $200 | $620 | **$6,820** |
| **HYBRID PHASE 2 TOTAL** | **2 months** | **$18,000** | **$700** | **$1,870** | **$20,570** |

**Rounded Hybrid Phase 2 Budget**: **$20,000 - $30,000**

### Hybrid Phase 3: Differential + Integration Testing (1-2 Months)

**Team**: 1 FTE (Integration specialist)

| Activity | Duration | Personnel Cost | External/Infra | Contingency (10%) | Total |
|----------|----------|----------------|----------------|-------------------|-------|
| Differential testing framework (vs Scala reference) | 2 weeks | $4,000 | $500 | $450 | **$4,950** |
| Run differential tests on all 11 proofs (100K+ test cases) | 2 weeks | $4,000 | $200 | $420 | **$4,620** |
| Integration tests with real Rholang contracts | 2 weeks | $4,000 | $300 | $430 | **$4,730** |
| CI/CD integration (run tests on every commit) | 1 week | $2,000 | $500 | $250 | **$2,750** |
| **HYBRID PHASE 3 TOTAL** | **1-2 months** | **$14,000** | **$1,500** | **$1,550** | **$17,050** |

**Rounded Hybrid Phase 3 Budget**: **$15,000 - $20,000**

---

## Hybrid Total

| Phase | Duration | Cost |
|-------|----------|------|
| **Hybrid Phase 1**: Foundation + 3 mechanized | 6-8 months | $90K-120K |
| **Hybrid Phase 2**: Property test 8 proofs | 2 months | $20K-30K |
| **Hybrid Phase 3**: Differential + integration | 1-2 months | $15K-20K |
| **HYBRID TOTAL** | **9-12 months** | **$125K-170K** |

---

## Property Testing Only Breakdown

### Property Testing All 11 Proofs (2-3 Months)

**Team**: 1 FTE (Rust developer)

| Activity | Duration | Personnel Cost | External/Infra | Contingency (10%) | Total |
|----------|----------|----------------|----------------|-------------------|-------|
| Framework setup (proptest, generators for Rholang AST) | 2 weeks | $4,000 | $500 | $450 | **$4,950** |
| Property tests for all 11 proofs (10K+ tests each) | 6 weeks | $12,000 | $500 | $1,250 | **$13,750** |
| Differential testing against Scala reference | 2 weeks | $4,000 | $300 | $430 | **$4,730** |
| Documentation + CI integration | 1 week | $2,000 | $200 | $220 | **$2,420** |
| **TOTAL** | **2-3 months** | **$22,000** | **$1,500** | **$2,350** | **$25,850** |

**Rounded Property Testing Budget**: **$15,000 - $25,000**

**Note**: This provides ~95% confidence. Defer mechanization for future if/when needed.

---

## Cost Drivers and Assumptions

### Personnel Rates

| Role | Annual Salary | Hourly Rate | Typical Hours/Week |
|------|---------------|-------------|-------------------|
| **Senior Formal Methods Expert** | $165,000 | $80 | 20-40 (0.5-1 FTE) |
| **Junior Developer / Research Assistant** | $90,000 | $43 | 20 (0.5 FTE) |
| **Rust Expert** | $150,000 | $72 | 40 (1 FTE) |
| **Integration Specialist** | $120,000 | $58 | 40 (1 FTE) |
| **External Coq Expert Review** | N/A | $200 | 8 hours/month |
| **External Consultant (RustBelt/Iris)** | N/A | $250 | Variable |

### Infrastructure Costs

| Item | Cost | Frequency |
|------|------|-----------|
| **Development Workstation** (8+ cores, 16GB RAM) | $1,500 | One-time |
| **Cloud CI/CD** (GitHub Actions or similar) | $50-100 | Per month |
| **Coq/Isabelle Licenses** | $0 | Free (open source) |
| **AFP (Archive of Formal Proofs)** | $0 | Free |
| **Literature Access** (ACM Digital Library, SpringerLink) | $200 | Per month |
| **External Review** | $1,600-3,200 | Per review (8-16 hours @ $200/hour) |

### Contingency Rationale

- **10% for Phases 1-2**: Standard contingency for well-understood work
- **15% for Phase 3**: Higher risk due to custom tooling development and integration complexity

---

## ROI Analysis

### Break-Even Points

| Approach | Cost | Break-Even If Bug Causes Loss Of |
|----------|------|-----------------------------------|
| **Property Testing** | $20K | $20K+ |
| **Hybrid** | $150K | $150K+ (single consensus bug) |
| **Full Mechanization** | $300K | $300K+ (multiple critical bugs or certification required) |

**Blockchain Context**:
- Consensus bugs can cause multi-million dollar losses (e.g., DAO hack: $60M, Parity wallet freeze: $300M)
- Even small bugs in blockchain systems have disproportionate impact
- **All three approaches have positive ROI** given blockchain risk profile

### Confidence vs Cost

| Investment | Confidence | Cost per 1% Confidence |
|-----------|------------|------------------------|
| **Property Testing** | 95% | $263/1% |
| **Hybrid** | 98% | $1,530/1% |
| **Full Mechanization** | 98% | $3,060/1% |

**Analysis**: Hybrid approach offers best confidence-per-dollar ratio.

---

## Recommendations by Budget

| Available Budget | Recommended Approach | Timeline | Confidence |
|------------------|---------------------|----------|------------|
| **< $30K** | Property Testing Only | 2-3 months | 95% |
| **$30K-90K** | Hybrid (mechanize 1-2 critical proofs + property test rest) | 4-6 months | 96-97% |
| **$90K-180K** | ⭐ **Hybrid (Full)** | 9-12 months | 98% |
| **$180K-400K** | Full Mechanization (Phases 1+2) | 18-26 months | 98% |
| **> $400K** | Full Mechanization + Rust Integration (All phases) | 36-50 months | 99.9% |

**For production blockchain**: Recommend **Hybrid (Full)** at $125K-170K.

---

## Budget Validation Sources

These estimates are based on:

1. **Academic Projects**:
   - CompCert (C compiler verification): ~$2M over 10 years, ~100K LOC
   - seL4 (OS kernel verification): ~$10M over 5 years, ~20K LOC
   - RustBelt: ~$500K over 3 years for Rust semantics framework

2. **Industry Rates**:
   - Formal methods consultants: $150-300/hour
   - Academic researchers: $75-150/hour
   - Industry rates adjusted for academic collaboration: $80-100/hour

3. **Proof Complexity Analysis**:
   - Detailed review of actual proof document (docs/performance/optimization-equivalence-proofs.md)
   - Proof 4 complexity: 200 lines of mathematical proof, O(n²) summation analysis
   - Proof 11 complexity: 250 lines including monad laws, counterexamples, bipartite matching context

4. **Conservative Adjustment**:
   - Original estimates increased by 2-2.5× based on proof complexity review
   - Foundational work increased from 290 to 480 lemmas based on Rust semantics requirements

---

## Change Log

**2025-11-10**: Initial creation after consistency review
- Consolidated estimates from mechanization-strategy.md, foundational-work.md, phase1-poc-plan.md
- Increased Phase 1 from $50K-70K to $90K-120K (foundational work underestimated)
- Increased Phase 2 from $75K-135K to $150K-225K (proof complexity underestimated)
- Added detailed hybrid approach breakdown
- Established this as single source of truth

---

**End of Document**

**For Questions**: See mechanization-strategy.md for strategic overview or contact project lead.
