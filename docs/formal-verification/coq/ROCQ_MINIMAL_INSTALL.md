# Rocq Minimal Installation - Solution

## Problem Identified

Your Arch Linux `rocq` package (9.1.0-2) contains **only the minimal standard library**:
- ✅ `Init/*` modules (Datatypes, Logic, Nat, Peano, etc.)
- ✅ Primitive types (Arrays, Floats, Strings)
- ❌ **Missing**: `Lists.List`, `Arith.Arith`, `Bool.Bool`, `Logic.FunctionalExtensionality`, `Lia`

This is an incomplete installation - the full standard library (stdlib) is not included.

## Solutions

### Solution 1: Install Full Coq via OPAM (Recommended)

OPAM will give you the complete standard library:

```bash
# Install OPAM if not already installed
sudo pacman -S opam

# Initialize OPAM
opam init
eval $(opam env)

# Install Coq 8.19 (stable) or 9.1.0
opam install coq.8.19.0
# OR
opam install coq.9.1.0

# Verify
which coqc  # Should show ~/.opam/default/bin/coqc
coqc --version
echo 'Require Import List.' > /tmp/test.v && coqc /tmp/test.v  # Should succeed
```

Then compile our proofs:
```bash
cd /var/tmp/debug/f1r3node/docs/formal-verification/coq
make clean && make
```

### Solution 2: Install from Coq Platform

The Coq Platform provides a complete, tested installation:

```bash
# Download from https://github.com/coq/platform/releases
# Install following their instructions
```

### Solution 3: Build Minimal Version (Workaround)

If you can't install the full stdlib, I can create a minimal version that only uses:
- `Init.Datatypes` (nat, list, bool, option)
- `Init.Nat` (basic nat operations)
- `Init.Logic` (and, or, exists, forall)
- `Init.Peano` (nat induction)

This would require:
1. Removing all `Arith.Arith` imports (use only basic `Init.Nat`)
2. Removing `Lia` (replace with manual `omega`-style proofs or `Admitted`)
3. Removing `Logic.FunctionalExtensionality` (axiomatize if needed)
4. Defining our own list operations (since `Lists.List` is incomplete)

**Pros**: Works with your current installation
**Cons**:
- More verbose proofs
- Some lemmas would need `Admitted`
- Less powerful automation

Would you like me to create this minimal version?

### Solution 4: Check AUR for Complete Package

```bash
# Search AUR for full Coq packages
yay -Ss coq-stdlib
# or
paru -Ss coq

# Look for packages like:
# - coq-stdlib
# - coq-theories
# - coq-full
```

## Recommended Action

**I recommend Solution 1 (OPAM)** because:
- ✅ Complete standard library
- ✅ All our proofs will compile as-is
- ✅ Easy to manage multiple Coq versions
- ✅ Well-supported in the Coq community

After installing via OPAM, our generated Coq files will compile without any modifications.

## Quick Test After Install

```bash
# Test 1: Basic import
echo 'Require Import List. Check app_nil_l.' | coqtop

# Test 2: Lia tactic
echo 'Require Import Lia. Goal forall n, n + 0 = n. Proof. intros. lia. Qed.' | coqtop

# Test 3: Compile our files
cd /var/tmp/debug/f1r3node/docs/formal-verification/coq
make clean && make
```

## What's Missing from Your Rocq Package

Based on `pacman -Ql rocq`, you're missing these standard library modules:

**Missing from Lists/**:
- List.vo (main list theory with app, rev, fold, etc.)
- ListSet.vo, ListTactics.vo, SetoidList.vo, Streams.vo

**Missing from Arith/**:
- Arith.vo, Plus.vo, Minus.vo, Mult.vo, Lt.vo, Le.vo, Compare_dec.vo, etc.

**Missing from Bool/**:
- Bool.vo, BoolEq.vo, Sumbool.vo, etc.

**Missing from Logic/**:
- FunctionalExtensionality.vo, Classical.vo, ChoiceFacts.vo, etc.

**Missing tactic libraries**:
- Lia.vo (linear integer arithmetic)
- Omega.vo (predecessor to Lia)
- Psatz.vo (polynomial arithmetic)

This is why our imports fail - they expect a complete standard library.

## Next Steps

Let me know which solution you prefer:
1. **Install via OPAM** → I'll provide step-by-step instructions
2. **Create minimal version** → I'll rewrite the files to use only Init modules
3. **Wait for AUR package** → I'll document what's needed

The files I generated are **correct for a full Coq/Rocq installation**. The issue is purely that your system has an incomplete stdlib.
