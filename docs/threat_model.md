# Threat Model

## Assets
- User data
- Encryption keys
- Metadata
- Access policies

---

## Threats

### 1. Unauthorized Access
- Malicious local apps
- Compromised user sessions

### 2. Data Leakage
- Metadata exposure
- Improper encryption

### 3. Key Compromise
- Weak key storage
- Memory exposure

### 4. Network Attacks
- MITM
- Replay attacks

---

## Assumptions
- User device is trusted (baseline)
- OS protections are in place
- Physical attacks are out of scope (initially)

---

## Mitigations

### Encryption
- Strong AEAD (AES-GCM or ChaCha20-Poly1305)

### Key Isolation
- Keys never leave device
- Secure memory usage where possible

### Authentication
- Token or key-based local API auth

### Auditing
- Immutable audit logs

---

## Future Considerations
- Hardware-backed keys (TPM/Secure Enclave)
- Zero-knowledge proofs
- Remote attestation
