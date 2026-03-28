#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};


// Tracks global ticket statistics across the platform
#[contracttype]
#[derive(Clone)]
pub struct TicketStats {
    pub total_minted: u64,   // total tickets ever minted
    pub total_active: u64,   // tickets currently valid
    pub total_revoked: u64,  // tickets that have been revoked
}

// Symbol key for TicketStats in storage
const ALL_TICKETS: Symbol = symbol_short!("ALL_TCKT");

// Counter for generating unique ticket IDs
const COUNT_TICKET: Symbol = symbol_short!("C_TCKT");

// Maps a ticket_id to its Ticket struct
#[contracttype]
pub enum Ticketbook {
    Ticket(u64)
}

// Maps a holder's address string to their ticket_id (0 = none)
#[contracttype]
pub enum HolderBook {
    Holder(String)
}

// Core NFT ticket data
#[contracttype]
#[derive(Clone)]
pub struct Ticket {
    pub ticket_id: u64,          // unique identifier
    pub event_name: String,      // name of the event this ticket grants access to
    pub holder: String,          // address/identity of the ticket holder
    pub issued_at: u64,          // ledger timestamp when ticket was minted
    pub is_active: bool,         // true = valid for entry, false = revoked
}


#[contract]
pub struct NftTicketGatingContract;

#[contractimpl]
impl NftTicketGatingContract {

    /// Mint a new NFT ticket for a given holder and event.
    /// One holder can hold at most one active ticket at a time.
    /// Returns the newly minted ticket_id.
    pub fn mint_ticket(env: Env, holder: String, event_name: String) -> u64 {

        // Ensure the holder doesn't already have an active ticket
        let existing_id: u64 = env.storage().instance()
            .get(&HolderBook::Holder(holder.clone()))
            .unwrap_or(0);

        if existing_id != 0 {
            let existing = Self::view_ticket(env.clone(), existing_id);
            if existing.is_active {
                log!(&env, "Holder already owns an active ticket: {}", existing_id);
                panic!("Holder already owns an active ticket!");
            }
        }

        // Generate a new unique ticket ID
        let mut count: u64 = env.storage().instance()
            .get(&COUNT_TICKET)
            .unwrap_or(0);
        count += 1;

        let timestamp = env.ledger().timestamp();

        let ticket = Ticket {
            ticket_id: count,
            event_name,
            holder: holder.clone(),
            issued_at: timestamp,
            is_active: true,
        };

        // Update global stats
        let mut stats = Self::view_all_stats(env.clone());
        stats.total_minted += 1;
        stats.total_active += 1;

        // Persist ticket, holder index, counter, and stats
        env.storage().instance().set(&Ticketbook::Ticket(count), &ticket);
        env.storage().instance().set(&HolderBook::Holder(holder), &count);
        env.storage().instance().set(&COUNT_TICKET, &count);
        env.storage().instance().set(&ALL_TICKETS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Ticket minted with ID: {}", count);
        count
    }


    /// Verify whether a given ticket_id grants active access.
    /// Returns true if the ticket exists and has not been revoked.
    pub fn verify_access(env: Env, ticket_id: u64) -> bool {
        let ticket = Self::view_ticket(env.clone(), ticket_id);

        if ticket.ticket_id == 0 {
            log!(&env, "Ticket ID {} does not exist", ticket_id);
            return false;
        }

        if ticket.is_active {
            log!(&env, "Access GRANTED for Ticket-ID: {}", ticket_id);
        } else {
            log!(&env, "Access DENIED  for Ticket-ID: {} (revoked)", ticket_id);
        }

        ticket.is_active
    }


    /// Revoke (invalidate) a ticket by its ticket_id.
    /// Once revoked, verify_access will return false for this ticket.
    pub fn revoke_ticket(env: Env, ticket_id: u64) {
        let mut ticket = Self::view_ticket(env.clone(), ticket_id);

        if ticket.ticket_id == 0 {
            log!(&env, "Ticket ID {} not found", ticket_id);
            panic!("Ticket not found!");
        }

        if !ticket.is_active {
            log!(&env, "Ticket ID {} is already revoked", ticket_id);
            panic!("Ticket is already revoked!");
        }

        ticket.is_active = false;

        let mut stats = Self::view_all_stats(env.clone());
        stats.total_active -= 1;
        stats.total_revoked += 1;

        env.storage().instance().set(&Ticketbook::Ticket(ticket_id), &ticket);
        env.storage().instance().set(&ALL_TICKETS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Ticket-ID: {} has been revoked", ticket_id);
    }


    /// Fetch full details of a ticket by its ticket_id.
    /// Returns a Ticket with ticket_id = 0 if not found.
    pub fn view_ticket(env: Env, ticket_id: u64) -> Ticket {
        env.storage().instance()
            .get(&Ticketbook::Ticket(ticket_id))
            .unwrap_or(Ticket {
                ticket_id: 0,
                event_name: String::from_str(&env, "Not_Found"),
                holder:     String::from_str(&env, "Not_Found"),
                issued_at:  0,
                is_active:  false,
            })
    }


    /// Returns the global platform-wide ticket statistics.
    pub fn view_all_stats(env: Env) -> TicketStats {
        env.storage().instance()
            .get(&ALL_TICKETS)
            .unwrap_or(TicketStats {
                total_minted: 0,
                total_active: 0,
                total_revoked: 0,
            })
    }
}