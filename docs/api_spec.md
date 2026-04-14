# API Specification

## Overview
Localhost API for interacting with the vault.

Base URL:
http://127.0.0.1:PORT

---

## Authentication
- Token-based (local)
- Future: keypair-based auth

---

## Endpoints

### Store Data
POST /data

Request:
{
  "data": "...",
  "tags": ["example"]
}

Response:
{
  "id": "uuid"
}

---

### Retrieve Data
GET /data/{id}

Response:
{
  "data": "...",
  "metadata": {}
}

---

### Delete Data
DELETE /data/{id}

---

### List Data
GET /data

---

### Permissions
POST /permissions

---

### Audit Logs
GET /audit

---

## Security Notes
- API only binds to localhost
- All requests authenticated
- Rate limiting recommended

---

## Future Extensions
- P2P sync endpoints
- ZK proof endpoints
- External app sandboxing
