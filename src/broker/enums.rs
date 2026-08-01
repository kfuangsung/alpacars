use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountSubType {
    Traditional,
    Roth,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Trading,
    Custodial,
    #[serde(rename = "donor_advised")]
    DonorAdvised,
    Ira,
    Hsa,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingSource {
    EmploymentIncome,
    Investments,
    Inheritance,
    BusinessIncome,
    Savings,
    Family,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EmploymentStatus {
    Unemployed,
    Employed,
    Student,
    Retired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
pub enum ACHRelationshipStatus {
    Queued,
    Approved,
    Pending,
    CancelRequested,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountType {
    Checking,
    Savings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferType {
    Ach,
    Wire,
    Iat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferDirection {
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JournalEntryType {
    Cash,
    Security,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JournalStatus {
    Pending,
    ActivityCreated,
    Executed,
    Rejected,
    Canceled,
    Refused,
    Corrected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortfolioStatus {
    Active,
    Inactive,
    NeedsAdjustment,
}
