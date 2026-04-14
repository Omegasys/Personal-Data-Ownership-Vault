# Encryption Design

## Goals
- Confidentiality
- Integrity
- Forward security (future)

---

## Algorithms

### Symmetric Encryption
- AES-256-GCM or ChaCha20-Poly1305

### Asymmetric
- Ed25519 (signing)
- X25519 (key exchange)

---

## Key Hierarchy

Master Key (User Root)
│
├── Data Encryption Keys (DEKs)
│   └── Used per object/file
│
└── Signing Keys
    └── Identity verification

---

## Encryption Flow

1. Generate DEK
2. Encrypt data with DEK
3. Encrypt DEK with master key
4. Store:
   - encrypted data
   - encrypted DEK
   - metadata

---

## Key Storage
- Stored locally (encrypted)
- Optionally derived from passphrase

---

## Rotation Strategy
- Periodic key rotation
- Re-encrypt DEKs when needed

---

## Integrity
- Authenticated encryption (AEAD)
- Signature verification for shared data
