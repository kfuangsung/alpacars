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

/// `document_type` of an owner document (`POST /v1/accounts/{account_id}/documents/upload`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum DocumentType {
    #[serde(rename = "account_approval_letter")]
    AccountApprovalLetter,
    #[serde(rename = "address_verification")]
    AddressVerification,
    #[serde(rename = "cip_result")]
    CipResult,
    #[serde(rename = "company_formation")]
    CompanyFormation,
    #[serde(rename = "date_of_birth_verification")]
    DateOfBirthVerification,
    #[serde(rename = "entity_operating_document")]
    EntityOperatingDocument,
    #[serde(rename = "entity_registration")]
    EntityRegistration,
    #[serde(rename = "hio_declaration_form")]
    HioDeclarationForm,
    #[serde(rename = "identity_verification")]
    IdentityVerification,
    #[serde(rename = "limited_trading_authorization")]
    LimitedTradingAuthorization,
    #[serde(rename = "pep_declaration_form")]
    PepDeclarationForm,
    #[serde(rename = "tax_id_verification")]
    TaxIdVerification,
    #[serde(rename = "w8ben")]
    W8ben,
    #[serde(rename = "w9")]
    W9,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Type of an account document (`type` on `GET /v1/accounts/{account_id}/documents`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum TradeDocumentType {
    #[serde(rename = "account_statement")]
    AccountStatement,
    #[serde(rename = "trade_confirmation")]
    TradeConfirmation,
    #[serde(rename = "account_application")]
    AccountApplication,
    #[serde(rename = "crypto_account_statement")]
    CryptoAccountStatement,
    #[serde(rename = "options_application")]
    OptionsApplication,
    #[serde(rename = "tax_statement")]
    TaxStatement,
    #[serde(rename = "trade_confirmation_json")]
    TradeConfirmationJson,
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

/// Wire values are lowercase (`ach`, `wire`), per the Broker transfer references.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

/// Agreement a party signs once, independent of any account (`POST /v1/agreements`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PartyAgreementType {
    MarginAgreement,
    CertificationsAndResolutionsStatement,
    NasdaqAgreement,
    NyseAgreement,
    OauthApiAgreement,
    OpraAgreement,
    TrustedContactConsent,
    CustodianCustomerAgreement,
}

/// Agreement signed for a specific account (`POST /v1/agreements` with `account_id`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PerAccountAgreementType {
    AccountAgreement,
    CustomerAgreement,
    OptionsAgreement,
    MasterSecuritiesLendingAgreement,
    CryptoAgreement,
    DvpCustomerAgreement,
    EtcAgreement,
    JointAccountAgreement,
}

/// Type of a [`SignedAgreement`](crate::broker::models::SignedAgreement): any party-level or
/// per-account agreement type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SignedAgreementType {
    MarginAgreement,
    CertificationsAndResolutionsStatement,
    NasdaqAgreement,
    NyseAgreement,
    OauthApiAgreement,
    OpraAgreement,
    TrustedContactConsent,
    CustodianCustomerAgreement,
    AccountAgreement,
    CustomerAgreement,
    OptionsAgreement,
    MasterSecuritiesLendingAgreement,
    CryptoAgreement,
    DvpCustomerAgreement,
    EtcAgreement,
    JointAccountAgreement,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

// ── ACATS (Beta) ─────────────────────────────────────────────────────────────
// Every variant is renamed explicitly: values such as `TYPE_401K` do not survive
// `rename_all` case conversion.

/// Direction of an ACATS transfer relative to the Alpaca account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsTransferDirection {
    #[serde(rename = "INCOMING")]
    Incoming,
    #[serde(rename = "OUTGOING")]
    Outgoing,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Simplified ACATS transfer status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsTransferStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "SETTLED")]
    Settled,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Detailed DTCC status of an ACATS transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsDtccStatus {
    #[serde(rename = "REQUEST")]
    Request,
    #[serde(rename = "REQUEST_ADJUST")]
    RequestAdjust,
    #[serde(rename = "REQUEST_ADJUST_PAST")]
    RequestAdjustPast,
    #[serde(rename = "REQUEST_PAST")]
    RequestPast,
    #[serde(rename = "REQUEST_REJECT")]
    RequestReject,
    #[serde(rename = "REVIEW")]
    Review,
    #[serde(rename = "REVIEW_ADJUST_DELIVERER")]
    ReviewAdjustDeliverer,
    #[serde(rename = "REVIEW_ERROR")]
    ReviewError,
    #[serde(rename = "REVIEW_ACCELERATE")]
    ReviewAccelerate,
    #[serde(rename = "REVIEW_ADJUST_RECEIVER_ACCELERATE")]
    ReviewAdjustReceiverAccelerate,
    #[serde(rename = "SETTLE_PREP")]
    SettlePrep,
    #[serde(rename = "SETTLE_CLOSE")]
    SettleClose,
    #[serde(rename = "CLOSE_PURGE")]
    ClosePurge,
    #[serde(rename = "REQUEST_PTR")]
    RequestPtr,
    #[serde(rename = "MEMO_PURGE_PARTIAL_TRANSFER_REQUEST_RECEIVER")]
    MemoPurgePartialTransferRequestReceiver,
    #[serde(rename = "REJECT")]
    Reject,
    #[serde(rename = "SYSTEM_REJECTED")]
    SystemRejected,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// DTCC transfer type code (`FUL` full, `PTR` partial, `RCR` residual credit, ...).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsDtccTransferType {
    #[serde(rename = "FUL")]
    Ful,
    #[serde(rename = "FRV")]
    Frv,
    #[serde(rename = "MFC")]
    Mfc,
    #[serde(rename = "PTD")]
    Ptd,
    #[serde(rename = "PTR")]
    Ptr,
    #[serde(rename = "RCL")]
    Rcl,
    #[serde(rename = "RCR")]
    Rcr,
    #[serde(rename = "PTF")]
    Ptf,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Registration type of the account on an ACATS transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsAccountType {
    #[serde(rename = "AGENCY")]
    Agency,
    #[serde(rename = "BANK_CUSTODY")]
    BankCustody,
    #[serde(rename = "BENEFICIARY")]
    Beneficiary,
    #[serde(rename = "BENEFICIARY_ROTH_IRA")]
    BeneficiaryRothIra,
    #[serde(rename = "CORPORATE")]
    Corporate,
    #[serde(rename = "CO_TRUSTEE")]
    CoTrustee,
    #[serde(rename = "COVERDELL_IRA")]
    CoverdellIra,
    #[serde(rename = "CUSTODIAN_UGMA")]
    CustodianUgma,
    #[serde(rename = "DIRECT_ROLLOVER")]
    DirectRollover,
    #[serde(rename = "ESTATE")]
    Estate,
    #[serde(rename = "HSA")]
    Hsa,
    #[serde(rename = "IRA")]
    Ira,
    #[serde(rename = "JOINT")]
    Joint,
    #[serde(rename = "MEDICAL_SAVINGS_ACCOUNT")]
    MedicalSavingsAccount,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "QUALIFIED_OR_PROFIT_SHARING_OR_PENSION")]
    QualifiedOrProfitSharingOrPension,
    #[serde(rename = "ROTH_401K")]
    Roth401k,
    #[serde(rename = "ROTH_IRA")]
    RothIra,
    #[serde(rename = "ROTH_SEP")]
    RothSep,
    #[serde(rename = "ROTH_SIMPLE")]
    RothSimple,
    #[serde(rename = "ROTH_SOLO_401K")]
    RothSolo401k,
    #[serde(rename = "SARSEP")]
    Sarsep,
    #[serde(rename = "SEP_IRA")]
    SepIra,
    #[serde(rename = "SIMPLE_IRA")]
    SimpleIra,
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "SOLO_401K")]
    Solo401k,
    #[serde(rename = "TRUST")]
    Trust,
    #[serde(rename = "TYPE_401K")]
    Type401k,
    #[serde(rename = "TYPE_403B")]
    Type403b,
    #[serde(rename = "TYPE_457_PLAN")]
    Type457Plan,
    #[serde(rename = "TYPE_529_PLAN")]
    Type529Plan,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Kind of contra party on an ACATS transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsContraBrokerType {
    #[serde(rename = "BANK")]
    Bank,
    #[serde(rename = "BROKER")]
    Broker,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Why an ACATS transfer was rejected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsRejectionReason {
    #[serde(rename = "SS_TAX_ID_MISMATCH")]
    SsTaxIdMismatch,
    #[serde(rename = "ACCOUNT_TITLE_MISMATCH")]
    AccountTitleMismatch,
    #[serde(rename = "DOCUMENTATION_NEEDED")]
    DocumentationNeeded,
    #[serde(rename = "ACCOUNT_FLAT")]
    AccountFlat,
    #[serde(rename = "INVALID_ACCOUNT_NUMBER")]
    InvalidAccountNumber,
    #[serde(rename = "DUPLICATE")]
    Duplicate,
    #[serde(rename = "ACCOUNT_IN_DISTRIBUTION_OR_TRANSFER")]
    AccountInDistributionOrTransfer,
    #[serde(rename = "CLIENT_RESCINDED")]
    ClientRescinded,
    #[serde(rename = "MISSING_AUTHORIZATION_SIGNATURE")]
    MissingAuthorizationSignature,
    #[serde(rename = "ACCOUNT_VIOLATES_CREDIT_POLICY_OF_RECEIVING_FIRM")]
    AccountViolatesCreditPolicyOfReceivingFirm,
    #[serde(rename = "UNRECOGNIZED_FOR_RESIDUAL_CREDIT_BALANCE")]
    UnrecognizedForResidualCreditBalance,
    #[serde(rename = "PARTIAL_TRANSFER_DELIVERER_INITIATED_REJECT")]
    PartialTransferDelivererInitiatedReject,
    #[serde(rename = "FAIL_REVERSAL_REJECT")]
    FailReversalReject,
    #[serde(rename = "RECLAIM_REJECT")]
    ReclaimReject,
    #[serde(rename = "MUTUAL_FUND_CLEANUP")]
    MutualFundCleanup,
    #[serde(rename = "SYSTEM_REJECTED")]
    SystemRejected,
    #[serde(rename = "INVALID_PARTICIPANT")]
    InvalidParticipant,
    #[serde(rename = "ASSOCIATED_RECORD_NOT_PRESENT_OR_INVALID")]
    AssociatedRecordNotPresentOrInvalid,
    #[serde(rename = "MISSED_CUTOFF")]
    MissedCutoff,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Position type of an ACATS asset. Requests may only use `Long` and `Short`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsPosition {
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "SHORT")]
    Short,
    #[serde(rename = "CREDIT")]
    Credit,
    #[serde(rename = "DEBIT")]
    Debit,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Asset class of an asset on an ACATS transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsAssetClass {
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "OTHER")]
    Other,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Status of one asset on an ACATS transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsAssetStatus {
    #[serde(rename = "NOT_VALIDATED")]
    NotValidated,
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "SETTLED")]
    Settled,
    #[serde(rename = "REVERSED")]
    Reversed,
    #[serde(rename = "INTERFACE_REJECTED")]
    InterfaceRejected,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Sort order for listing ACATS transfers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsSortDirection {
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,
}

/// Field to sort ACATS transfers by.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AcatsSortBy {
    #[serde(rename = "account_number")]
    AccountNumber,
    #[serde(rename = "total_transfer_value")]
    TotalTransferValue,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "contra_account_number")]
    ContraAccountNumber,
    #[serde(rename = "contra_broker_number")]
    ContraBrokerNumber,
    #[serde(rename = "external_transfer_id")]
    ExternalTransferId,
    #[serde(rename = "settled_at")]
    SettledAt,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "transfer_type")]
    TransferType,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}
