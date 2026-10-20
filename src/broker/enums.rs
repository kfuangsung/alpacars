use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AccountSubType {
    Traditional,
    Roth,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AccountType {
    Trading,
    Custodial,
    #[serde(rename = "donor_advised")]
    DonorAdvised,
    Ira,
    Hsa,
    /// Returned on reads only; accounts of this type cannot be created through the API.
    Trust,
    /// Returned on reads only; accounts of this type cannot be created through the API.
    #[serde(rename = "omnibus_non_disclosed")]
    OmnibusNonDisclosed,
    /// Returned on reads only; accounts of this type cannot be created through the API.
    #[serde(rename = "omnibus_sub")]
    OmnibusSub,
    /// Returned on reads only; accounts of this type cannot be created through the API.
    Joint,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TaxIdType {
    #[serde(rename = "USA_SSN")]
    UsaSsn,
    #[serde(rename = "USA_ITIN")]
    UsaItin,
    #[serde(rename = "ARG_AR_CUIT")]
    ArgArCuit,
    #[serde(rename = "AUS_TFN")]
    AusTfn,
    #[serde(rename = "AUS_ABN")]
    AusAbn,
    #[serde(rename = "BRA_CPF")]
    BraCpf,
    #[serde(rename = "GBR_UTR")]
    GbrUtr,
    #[serde(rename = "GBR_NINO")]
    GbrNino,
    #[serde(rename = "IND_PAN")]
    IndPan,
    #[serde(rename = "NATIONAL_ID")]
    NationalId,
    #[serde(rename = "PASSPORT")]
    Passport,
    #[serde(rename = "PERMANENT_RESIDENT")]
    PermanentResident,
    #[serde(rename = "DRIVER_LICENSE")]
    DriverLicense,
    #[serde(rename = "OTHER_GOV_ID")]
    OtherGovId,
    #[serde(rename = "NOT_SPECIFIED")]
    NotSpecified,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VisaType {
    B1,
    B2,
    DACA,
    E1,
    E2,
    E3,
    F1,
    G4,
    H1B,
    J1,
    L1,
    #[serde(rename = "OTHER")]
    Other,
    O1,
    TN1,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FundingSource {
    EmploymentIncome,
    Investments,
    Inheritance,
    BusinessIncome,
    Savings,
    Family,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum EmploymentStatus {
    Unemployed,
    Employed,
    Student,
    Retired,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AgreementType {
    #[serde(rename = "margin_agreement")]
    Margin,
    #[serde(rename = "account_agreement")]
    Account,
    #[serde(rename = "customer_agreement")]
    Customer,
    #[serde(rename = "crypto_agreement")]
    Crypto,
    #[serde(rename = "options_agreement")]
    Options,
    #[serde(rename = "custodial_customer_agreement")]
    CustodialCustomer,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum DocumentType {
    DriversLicense,
    NationalIdCard,
    Passport,
    PassportCard,
    VisaPermit,
    ResidencePermit,
    TaxReturn,
    FinancialStatement,
    BankStatement,
    FundSource,
    AccountTransferIn,
    LimitedPoa,
    Other,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TradeDocumentType {
    AccountStatement,
    TradeConfirmation,
    TaxForm1099B,
    TaxForm1099Div,
    TaxForm1099Int,
    TaxForm1099Misc,
    TaxForm1099R,
    TaxFormW8ben,
    TaxFormW8bene,
    TaxFormW9,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum UploadDocumentMimeType {
    #[serde(rename = "application/pdf")]
    Pdf,
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/png")]
    Png,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ACHRelationshipStatus {
    Queued,
    Approved,
    Pending,
    CancelRequested,
    Rejected,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum BankAccountType {
    Checking,
    Savings,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TransferType {
    Ach,
    Wire,
    Iat,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TransferStatus {
    Queued,
    ApprovalPending,
    Pending,
    SentToClearing,
    Rejected,
    Canceled,
    Approved,
    Complete,
    ReturnedFromClearing,
    Returned,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TransferDirection {
    Incoming,
    Outgoing,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Journal kind. Alpaca's wire values are `JNLC` (cash) and `JNLS` (securities).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum JournalEntryType {
    #[serde(rename = "JNLC")]
    Cash,
    #[serde(rename = "JNLS")]
    Security,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum JournalStatus {
    Pending,
    ActivityCreated,
    Executed,
    Rejected,
    Canceled,
    Refused,
    /// Wire value `correct`.
    #[serde(rename = "correct")]
    Corrected,
    /// In queue, not yet processed.
    Queued,
    /// Sent to Alpaca's booking system, not yet processed.
    SentToClearing,
    Deleted,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PortfolioStatus {
    Active,
    Inactive,
    NeedsAdjustment,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// What a portfolio or run weight allocates to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum WeightType {
    Cash,
    Asset,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Kind of rebalancing run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RunType {
    FullRebalance,
    InvestCash,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Who started a rebalancing run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RunInitiatedFrom {
    System,
    Api,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}
