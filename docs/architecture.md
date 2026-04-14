# Architecture Overview

## High-Level Design

The Personal Data Ownership Vault is a modular system composed of:

- Core Layer (vault logic, identity, permissions)
- Crypto Layer (encryption, keys, signatures)
- Storage Layer (local + optional distributed)
- API Layer (local service interface)
- Privacy Layer (data minimization and sharing)

---

## Component Breakdown

### 1. Core
Handles:
- Vault lifecycle
- Access control
- Identity binding
- Audit logging

### 2. Crypto
Responsible for:
- Encryption / decryption
- Key management
- Signing / verification

### 3. Storage
- Local encrypted storage
- Object-based chunking
- Indexed metadata

### 4. API
- Localhost REST/gRPC interface
- Authenticated access
- App integrations

### 5. Privacy
- Selective disclosure
- Data anonymization
- Future ZK integration

---

## Data Flow

1. Data enters via API/CLI
2. Permissions validated
3. Data encrypted
4. Stored in local storage
5. Indexed for retrieval
6. Audit log updated

---

## Security Boundaries

- Vault core is trusted
- Network is untrusted
- External apps must authenticate
