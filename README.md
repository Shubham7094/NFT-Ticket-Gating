# NFT Ticket Gating

## Table of Contents
- [Project Title](#nft-ticket-gating)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)
- [Contract Functions](#contract-functions)
- [Data Structures](#data-structures)

---

## Project Description

**NFT Ticket Gating** is a decentralized smart contract built on the **Stellar blockchain** using the **Soroban SDK**. It enables event organizers and platform administrators to issue, verify, and revoke NFT-based access tickets on-chain. Each ticket is uniquely identified and tied to a holder's address, granting or restricting entry to events, exclusive content, or gated communities — all enforced transparently through smart contract logic without relying on any centralized intermediary.

---

## Project Vision

The traditional ticketing industry is plagued by fraud, scalping, and opaque ownership transfers. Our vision is to replace paper tickets and fragile QR codes with **verifiable, tamper-proof, on-chain NFT tickets** that:

- Give event organizers **complete control** over who gets access.
- Give attendees **transparent proof of ownership** that cannot be forged.
- Create a foundation for a **trustless access economy** — where gating logic runs on code, not on human gatekeepers.

By building on Stellar's fast and low-cost network with Soroban smart contracts, NFT Ticket Gating aims to make blockchain-based access control practical for real-world events of any scale.

---

## Key Features

| Feature | Description |
|---|---|
| 🎟️ **NFT Ticket Minting** | Organizers can mint a unique on-chain ticket for any holder tied to a specific event name. |
| ✅ **On-chain Access Verification** | Anyone can verify in real time whether a ticket ID grants valid access — no backend required. |
| 🚫 **Ticket Revocation** | Organizers can revoke any active ticket instantly, preventing further access without deleting history. |
| 📊 **Global Stats Tracking** | The contract maintains a live count of total minted, active, and revoked tickets across the platform. |
| 🔒 **One Active Ticket Per Holder** | A holder cannot receive a new active ticket while they already hold a valid one, preventing double-minting. |
| 🕒 **Immutable Issuance Timestamp** | Each ticket records the ledger timestamp at the moment of minting, providing a verifiable audit trail. |

---

## Future Scope

- **Transferability** — Allow holders to transfer their ticket to another address, with organizer-controlled transfer restrictions (e.g., soulbound tickets for non-transferable events).
- **Tiered Access Levels** — Introduce ticket tiers (VIP, General, Backstage) with different access permissions enforced at the contract level.
- **Expiry Timestamps** — Add configurable expiry windows so tickets automatically become invalid after the event date, removing the need for manual revocation.
- **Batch Minting** — Enable organizers to mint hundreds of tickets in a single transaction for large-scale events.
- **Marketplace Integration** — Build a secondary marketplace contract where holders can list, sell, or auction tickets with on-chain royalty enforcement.
- **Multi-Event Support** — Extend the contract to support multiple concurrent events managed by different organizers under a single deployment.
- **Frontend dApp** — Develop a web-based dashboard for organizers to manage tickets and for attendees to view and present their NFT ticket at entry gates via QR-code-linked wallet signatures.

---

## Contract Functions

### `mint_ticket(env, holder, event_name) → u64`
Mints a new NFT ticket for the specified holder for a given event. Enforces one-active-ticket-per-holder. Returns the unique `ticket_id`.

### `verify_access(env, ticket_id) → bool`
Checks whether the given `ticket_id` is valid and active. Returns `true` for active tickets, `false` for revoked or non-existent ones.

### `revoke_ticket(env, ticket_id)`
Permanently invalidates an active ticket. Once revoked, `verify_access` will return `false`. The historical record of the ticket remains on-chain.

### `view_ticket(env, ticket_id) → Ticket`
Returns the full on-chain data for a ticket: ID, event name, holder address, issuance timestamp, and active status.

### `view_all_stats(env) → TicketStats`
Returns platform-wide statistics: total tickets minted, currently active, and revoked.

---

## Data Structures

### `Ticket`
```rust
pub struct Ticket {
    pub ticket_id: u64,      // unique identifier
    pub event_name: String,  // event this ticket grants access to
    pub holder: String,      // holder's address / identity
    pub issued_at: u64,      // ledger timestamp at minting
    pub is_active: bool,     // true = valid, false = revoked
}
```

### `TicketStats`
```rust
pub struct TicketStats {
    pub total_minted: u64,   // all tickets ever created
    pub total_active: u64,   // currently valid tickets
    pub total_revoked: u64,  // revoked tickets
}
```

---

*Built with ❤️ using [Soroban SDK](https://soroban.stellar.org/) on the Stellar Network.*# NFT Ticket Gating

## Table of Contents
- [Project Title](#nft-ticket-gating)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)
- [Contract Functions](#contract-functions)
- [Data Structures](#data-structures)

---

## Project Description

**NFT Ticket Gating** is a decentralized smart contract built on the **Stellar blockchain** using the **Soroban SDK**. It enables event organizers and platform administrators to issue, verify, and revoke NFT-based access tickets on-chain. Each ticket is uniquely identified and tied to a holder's address, granting or restricting entry to events, exclusive content, or gated communities — all enforced transparently through smart contract logic without relying on any centralized intermediary.

---

## Project Vision

The traditional ticketing industry is plagued by fraud, scalping, and opaque ownership transfers. Our vision is to replace paper tickets and fragile QR codes with **verifiable, tamper-proof, on-chain NFT tickets** that:

- Give event organizers **complete control** over who gets access.
- Give attendees **transparent proof of ownership** that cannot be forged.
- Create a foundation for a **trustless access economy** — where gating logic runs on code, not on human gatekeepers.

By building on Stellar's fast and low-cost network with Soroban smart contracts, NFT Ticket Gating aims to make blockchain-based access control practical for real-world events of any scale.

---

## Key Features

| Feature | Description |
|---|---|
| 🎟️ **NFT Ticket Minting** | Organizers can mint a unique on-chain ticket for any holder tied to a specific event name. |
| ✅ **On-chain Access Verification** | Anyone can verify in real time whether a ticket ID grants valid access — no backend required. |
| 🚫 **Ticket Revocation** | Organizers can revoke any active ticket instantly, preventing further access without deleting history. |
| 📊 **Global Stats Tracking** | The contract maintains a live count of total minted, active, and revoked tickets across the platform. |
| 🔒 **One Active Ticket Per Holder** | A holder cannot receive a new active ticket while they already hold a valid one, preventing double-minting. |
| 🕒 **Immutable Issuance Timestamp** | Each ticket records the ledger timestamp at the moment of minting, providing a verifiable audit trail. |

---

## Future Scope

- **Transferability** — Allow holders to transfer their ticket to another address, with organizer-controlled transfer restrictions (e.g., soulbound tickets for non-transferable events).
- **Tiered Access Levels** — Introduce ticket tiers (VIP, General, Backstage) with different access permissions enforced at the contract level.
- **Expiry Timestamps** — Add configurable expiry windows so tickets automatically become invalid after the event date, removing the need for manual revocation.
- **Batch Minting** — Enable organizers to mint hundreds of tickets in a single transaction for large-scale events.
- **Marketplace Integration** — Build a secondary marketplace contract where holders can list, sell, or auction tickets with on-chain royalty enforcement.
- **Multi-Event Support** — Extend the contract to support multiple concurrent events managed by different organizers under a single deployment.
- **Frontend dApp** — Develop a web-based dashboard for organizers to manage tickets and for attendees to view and present their NFT ticket at entry gates via QR-code-linked wallet signatures.

---

## Contract Functions

### `mint_ticket(env, holder, event_name) → u64`
Mints a new NFT ticket for the specified holder for a given event. Enforces one-active-ticket-per-holder. Returns the unique `ticket_id`.

### `verify_access(env, ticket_id) → bool`
Checks whether the given `ticket_id` is valid and active. Returns `true` for active tickets, `false` for revoked or non-existent ones.

### `revoke_ticket(env, ticket_id)`
Permanently invalidates an active ticket. Once revoked, `verify_access` will return `false`. The historical record of the ticket remains on-chain.

### `view_ticket(env, ticket_id) → Ticket`
Returns the full on-chain data for a ticket: ID, event name, holder address, issuance timestamp, and active status.

### `view_all_stats(env) → TicketStats`
Returns platform-wide statistics: total tickets minted, currently active, and revoked.

---

## Data Structures
# NFT Ticket Gating

## Table of Contents
- [Project Title](#nft-ticket-gating)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)
- [Contract Functions](#contract-functions)
- [Data Structures](#data-structures)

---

## Project Description

**NFT Ticket Gating** is a decentralized smart contract built on the **Stellar blockchain** using the **Soroban SDK**. It enables event organizers and platform administrators to issue, verify, and revoke NFT-based access tickets on-chain. Each ticket is uniquely identified and tied to a holder's address, granting or restricting entry to events, exclusive content, or gated communities — all enforced transparently through smart contract logic without relying on any centralized intermediary.

---

## Project Vision

The traditional ticketing industry is plagued by fraud, scalping, and opaque ownership transfers. Our vision is to replace paper tickets and fragile QR codes with **verifiable, tamper-proof, on-chain NFT tickets** that:

- Give event organizers **complete control** over who gets access.
- Give attendees **transparent proof of ownership** that cannot be forged.
- Create a foundation for a **trustless access economy** — where gating logic runs on code, not on human gatekeepers.

By building on Stellar's fast and low-cost network with Soroban smart contracts, NFT Ticket Gating aims to make blockchain-based access control practical for real-world events of any scale.

---

## Key Features

| Feature | Description |
|---|---|
| 🎟️ **NFT Ticket Minting** | Organizers can mint a unique on-chain ticket for any holder tied to a specific event name. |
| ✅ **On-chain Access Verification** | Anyone can verify in real time whether a ticket ID grants valid access — no backend required. |
| 🚫 **Ticket Revocation** | Organizers can revoke any active ticket instantly, preventing further access without deleting history. |
| 📊 **Global Stats Tracking** | The contract maintains a live count of total minted, active, and revoked tickets across the platform. |
| 🔒 **One Active Ticket Per Holder** | A holder cannot receive a new active ticket while they already hold a valid one, preventing double-minting. |
| 🕒 **Immutable Issuance Timestamp** | Each ticket records the ledger timestamp at the moment of minting, providing a verifiable audit trail. |

---

## Future Scope

- **Transferability** — Allow holders to transfer their ticket to another address, with organizer-controlled transfer restrictions (e.g., soulbound tickets for non-transferable events).
- **Tiered Access Levels** — Introduce ticket tiers (VIP, General, Backstage) with different access permissions enforced at the contract level.
- **Expiry Timestamps** — Add configurable expiry windows so tickets automatically become invalid after the event date, removing the need for manual revocation.
- **Batch Minting** — Enable organizers to mint hundreds of tickets in a single transaction for large-scale events.
- **Marketplace Integration** — Build a secondary marketplace contract where holders can list, sell, or auction tickets with on-chain royalty enforcement.
- **Multi-Event Support** — Extend the contract to support multiple concurrent events managed by different organizers under a single deployment.
- **Frontend dApp** — Develop a web-based dashboard for organizers to manage tickets and for attendees to view and present their NFT ticket at entry gates via QR-code-linked wallet signatures.

---

## Contract Functions

### `mint_ticket(env, holder, event_name) → u64`
Mints a new NFT ticket for the specified holder for a given event. Enforces one-active-ticket-per-holder. Returns the unique `ticket_id`.

### `verify_access(env, ticket_id) → bool`
Checks whether the given `ticket_id` is valid and active. Returns `true` for active tickets, `false` for revoked or non-existent ones.

### `revoke_ticket(env, ticket_id)`
Permanently invalidates an active ticket. Once revoked, `verify_access` will return `false`. The historical record of the ticket remains on-chain.

### `view_ticket(env, ticket_id) → Ticket`
Returns the full on-chain data for a ticket: ID, event name, holder address, issuance timestamp, and active status.

### `view_all_stats(env) → TicketStats`
Returns platform-wide statistics: total tickets minted, currently active, and revoked.

---

## Data Structures

### `Ticket`
```rust
pub struct Ticket {
    pub ticket_id: u64,      // unique identifier
    pub event_name: String,  // event this ticket grants access to
    pub holder: String,      // holder's address / identity
    pub issued_at: u64,      // ledger timestamp at minting
    pub is_active: bool,     // true = valid, false = revoked
}
```

### `TicketStats`
```rust
pub struct TicketStats {
    pub total_minted: u64,   // all tickets ever created
    pub total_active: u64,   // currently valid tickets
    pub total_revoked: u64,  // revoked tickets
}
```

---
### `Ticket`
```rust
pub struct Ticket {
    pub ticket_id: u64,      // unique identifier
    pub event_name: String,  // event this ticket grants access to
    pub holder: String,      // holder's address / identity
    pub issued_at: u64,      // ledger timestamp at minting
    pub is_active: bool,     // true = valid, false = revoked
}
```

### `TicketStats`
```rust
pub struct TicketStats {
    pub total_minted: u64,   // all tickets ever created
    pub total_active: u64,   // currently valid tickets
    pub total_revoked: u64,  // revoked tickets
}
```

---

*Built with ❤️ using [Soroban SDK](https://soroban.stellar.org/) on the Stellar Network.*

## Contract Details

Contract ID: CDF3W45BPZQHEDLF7KNDSD43KVPYH3BBMWU2F7H4UEVSSFRHWEXGC7F5
<img width="1888" height="968" alt="image" src="https://github.com/user-attachments/assets/d492858e-d24e-487e-a4ca-f7ca46b3009d" />

