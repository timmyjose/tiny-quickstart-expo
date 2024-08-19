use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Product {
    Assets,
    Auth,
    Balance,
    BalancePlus,
    Beacon,
    CraBaseReport,
    CraCashflowInsights,
    CraIncomeInsights,
    CraPartnerInsights,
    CreditDetails,
    DepositSwitch,
    Employment,
    Identity,
    IdentityMatch,
    IdentityVerification,
    Income,
    IncomeVerification,
    Investments,
    InvestmentsAuth,
    Layer,
    Liabilities,
    PaymentInitiation,
    ProcessorIdentity,
    ProcessorPayments,
    Profile,
    RecurringTransactions,
    Signal,
    StandingOrders,
    Statements,
    Transactions,
    Transfer,
}
