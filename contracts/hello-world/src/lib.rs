#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Structure to model each NFT ticket with unique attributes
#[contracttype]
#[derive(Clone)]
pub struct Ticket {
    pub ticket_id: u64,
    pub owner: String,
    pub event_name: String,
    pub is_used: bool,      // To mark if ticket has been used/validated for entry
}

// Counter for ticket issuance
const COUNT_TICKET: Symbol = symbol_short!("COUNT_TKT");

// Mapping Ticket ID to Ticket struct
#[contracttype]
pub enum TicketBook {
    Ticket(u64)
}

#[contract]
pub struct EventTicketContract;

#[contractimpl]
impl EventTicketContract {
    // Mint a new NFT ticket for a user
    pub fn mint_ticket(env: Env, owner: String, event_name: String) -> u64 {
        let mut count_ticket: u64 = env.storage().instance().get(&COUNT_TICKET).unwrap_or(0);
        count_ticket += 1;
        let ticket = Ticket {
            ticket_id: count_ticket,
            owner: owner.clone(),
            event_name: event_name.clone(),
            is_used: false,
        };
        env.storage().instance().set(&TicketBook::Ticket(ticket.ticket_id), &ticket);
        env.storage().instance().set(&COUNT_TICKET, &count_ticket);
        log!(&env, "Minted ticket {} for {}", ticket.ticket_id, owner);
        ticket.ticket_id
    }

    // View ticket details
    pub fn view_ticket(env: Env, ticket_id: u64) -> Ticket {
        env.storage().instance().get(&TicketBook::Ticket(ticket_id)).unwrap_or(
            Ticket {
                ticket_id: 0,
                owner: String::from_str(&env, "Not_Found"),
                event_name: String::from_str(&env, "Not_Found"),
                is_used: true,
            }
        )
    }

    // Mark a ticket as used (entry validated)
    pub fn use_ticket(env: Env, ticket_id: u64) {
        let mut ticket = Self::view_ticket(env.clone(), ticket_id);
        if ticket.ticket_id == 0 || ticket.is_used == true {
            log!(&env, "Ticket not valid or already used");
            panic!("Ticket not valid or already used");
        }
        ticket.is_used = true;
        env.storage().instance().set(&TicketBook::Ticket(ticket_id), &ticket);
        log!(&env, "Ticket {} marked as used", ticket_id);
    }

    // Transfer ticket to new owner
    pub fn transfer_ticket(env: Env, ticket_id: u64, new_owner: String) {
        let mut ticket = Self::view_ticket(env.clone(), ticket_id);
        if ticket.ticket_id == 0 || ticket.is_used == true {
            log!(&env, "Ticket transfer not allowed");
            panic!("Ticket transfer not allowed");
        }
        ticket.owner = new_owner.clone();
        env.storage().instance().set(&TicketBook::Ticket(ticket_id), &ticket);
        log!(&env, "Ticket {} transferred to {}", ticket_id, new_owner);
    }
}
