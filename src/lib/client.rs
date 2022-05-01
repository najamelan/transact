use crate::import::*;

/// Represents a client account.
/// There are 2 types of balance: available and held.
/// Held corresponds to funds from disputed transactions.
//
#[derive(Clone, PartialEq, Debug)]
//
pub struct Client {
    pub(crate) available: BigDecimal,
    pub(crate) held: BigDecimal,
    pub(crate) id: u16,
    pub(crate) locked: bool,
}

impl Client {
    /// Create a new client with the given ID.
    //
    pub fn new(id: u16) -> Self {
        Self {
            available: BigDecimal::default(),
            held: BigDecimal::default(),
            locked: false,
            id,
        }
    }

    /// The unique identifier for this account.
    //
    pub fn id(&self) -> u16 {
        self.id
    }

    /// The available funds for the client. These are the funds they dispose of
    /// for withdrawal.
    //
    pub fn available(&self) -> BigDecimal {
        self.available.clone()
    }

    /// Disputed funds are funds for a deposit the client wishes to undo.
    //
    pub fn held(&self) -> BigDecimal {
        self.held.clone()
    }

    /// The total of available and disputed funds.
    /// This is infallible because the setters for available and held guarantee the sum
    /// can be represented in an f64.
    //
    pub fn total(&self) -> BigDecimal {
        &self.available + &self.held
    }

    /// After a charge-back the account will be locked and no further transactions
    /// will be allowed.
    //
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Lock this account. No further transactions will be allowed.
    //
    pub fn lock(&mut self) -> &mut Self {
        self.locked = true;
        self
    }
}
