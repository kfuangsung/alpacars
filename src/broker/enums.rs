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

/// Type of a tax ID or other government-issued identification number (`tax_id_type`).
///
/// A country-specific type is meant for accounts whose `country_of_tax_residence` is that
/// country (or an associated jurisdiction); a listed type can still be rejected when the
/// account is validated. Send an LEI in `identifications`, not here: `LEI` fails with 422.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TaxIdType {
    /// USA SSN (Social Security Number), for individuals
    #[serde(rename = "USA_SSN")]
    UsaSsn,
    /// USA EIN (Employer Identification Number), for entities
    #[serde(rename = "USA_EIN")]
    UsaEin,
    /// USA ITIN (Individual Taxpayer Identification Number), for individuals
    #[serde(rename = "USA_ITIN")]
    UsaItin,
    /// USA TIN (Taxpayer Identification Number), for entities
    #[serde(rename = "USA_TIN")]
    UsaTin,
    /// Aruba Persoonsnummer (Aruba TIN), for individuals and entities
    #[serde(rename = "ABW_PN")]
    AbwPn,
    /// Angola NIF (Número de Identificação Fiscal), for individuals and entities
    #[serde(rename = "AGO_NIF")]
    AgoNif,
    /// Albania NIPT (Numri i Identifikimit të Personit të Tatueshëm), for individuals and entities
    #[serde(rename = "ALB_NIPT")]
    AlbNipt,
    /// Andorra NRT (Número de Registre Tributari), for individuals and entities
    #[serde(rename = "AND_NRT")]
    AndNrt,
    /// UAE Emirates ID, for individuals
    #[serde(rename = "ARE_EID")]
    AreEid,
    /// UAE TRN (Tax Registration Number), for entities
    #[serde(rename = "ARE_TRN")]
    AreTrn,
    /// Legacy value for an Argentina CUIT. Use `ARG_AR_CUIT` instead.
    #[serde(rename = "ARG_AG_CUIT")]
    ArgAgCuit,
    /// Argentina CUIT (Clave Única de Identificación Tributaria), for individuals and entities
    #[serde(rename = "ARG_AR_CUIT")]
    ArgArCuit,
    /// Armenia SSN (Social Security Number), for individuals
    #[serde(rename = "ARM_SSN")]
    ArmSsn,
    /// Armenia TIN (Tax Identification Number), for entities
    #[serde(rename = "ARM_TIN")]
    ArmTin,
    /// Australia ABN (Australian business number), for entities
    #[serde(rename = "AUS_ABN")]
    AusAbn,
    /// Australia TFN (Tax File Number), for individuals and entities
    #[serde(rename = "AUS_TFN")]
    AusTfn,
    /// Austria UID-Nummer (VAT Identification Number), for entities
    #[serde(rename = "AUT_UID")]
    AutUid,
    /// Austria Versicherungsnummer (Social Insurance Number), for individuals
    #[serde(rename = "AUT_VNR")]
    AutVnr,
    /// Azerbaijan FIN (Fərdi İdentifikasiya Nömrəsi), for individuals
    #[serde(rename = "AZE_FIN")]
    AzeFin,
    /// Azerbaijan VÖEN (Tax Identification Number), for entities
    #[serde(rename = "AZE_VOEN")]
    AzeVoen,
    /// Belgium BCE/KBO (Enterprise Number), for entities
    #[serde(rename = "BEL_BCE")]
    BelBce,
    /// Belgium NN (Numéro National / Rijksregisternummer), for individuals
    #[serde(rename = "BEL_NN")]
    BelNn,
    /// Benin IFU (Identifiant Fiscal Unique), for individuals and entities
    #[serde(rename = "BEN_IFU")]
    BenIfu,
    /// Burkina Faso IFU (Identifiant Financier Unique), for individuals and entities
    #[serde(rename = "BFA_IFU")]
    BfaIfu,
    /// Bangladesh TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "BGD_TIN")]
    BgdTin,
    /// Bulgaria EGN (Edinen Grazhdanski Nomer), for individuals
    #[serde(rename = "BGR_EGN")]
    BgrEgn,
    /// Bulgaria EIK/BULSTAT (Entity Identification Code), for entities
    #[serde(rename = "BGR_EIK")]
    BgrEik,
    /// Bahrain CPR (Central Population Registry), for individuals
    #[serde(rename = "BHR_CPR")]
    BhrCpr,
    /// Bosnia and Herzegovina JMBG (Jedinstveni Matični Broj Građana), for individuals
    #[serde(rename = "BIH_JMBG")]
    BihJmbg,
    /// Belarus Personal Identification Number, for individuals
    #[serde(rename = "BLR_ID")]
    BlrId,
    /// Belarus UNP (Uchyotny Nomer Platelshchika), for entities
    #[serde(rename = "BLR_UNP")]
    BlrUnp,
    /// Belize TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "BLZ_TIN")]
    BlzTin,
    /// Bolivia NIT (Número de Identificación Tributaria), for individuals and entities
    #[serde(rename = "BOL_NIT")]
    BolNit,
    /// Brazil CNPJ (Cadastro Nacional da Pessoa Jurídica), for entities
    #[serde(rename = "BRA_CNPJ")]
    BraCnpj,
    /// Brazil CPF (Cadastro de Pessoas Físicas), for individuals
    #[serde(rename = "BRA_CPF")]
    BraCpf,
    /// Barbados TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "BRB_TIN")]
    BrbTin,
    /// Bhutan TPN (Taxpayer Number), for individuals and entities
    #[serde(rename = "BTN_TPN")]
    BtnTpn,
    /// Botswana TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "BWA_TIN")]
    BwaTin,
    /// Canada BN (Business Number), for entities
    #[serde(rename = "CAN_BN")]
    CanBn,
    /// Canada SIN (Social Insurance Number), for individuals
    #[serde(rename = "CAN_SIN")]
    CanSin,
    /// Switzerland AHV/AVS (Social Insurance Number), for individuals
    #[serde(rename = "CHE_AHV")]
    CheAhv,
    /// Switzerland UID (Unternehmens-Identifikationsnummer), for entities
    #[serde(rename = "CHE_UID")]
    CheUid,
    /// Chile RUT (Rol Único Tributario), for individuals and entities
    #[serde(rename = "CHL_RUT")]
    ChlRut,
    /// China Resident Identity Card Number (居民身份证号码), for individuals
    #[serde(rename = "CHN_RID")]
    ChnRid,
    /// China USCC (Unified Social Credit Code / 统一社会信用代码), for entities
    #[serde(rename = "CHN_USCC")]
    ChnUscc,
    /// Côte d'Ivoire NCC (Numéro de Compte Contribuable), for individuals and entities
    #[serde(rename = "CIV_NCC")]
    CivNcc,
    /// Cameroon NIU (Numéro d'Identifiant Unique), for individuals and entities
    #[serde(rename = "CMR_NIU")]
    CmrNiu,
    /// Cook Islands RMD Number (Revenue Management Division), for individuals and entities
    #[serde(rename = "COK_RMD")]
    CokRmd,
    /// Colombia CC (Cédula de Ciudadanía), for individuals
    #[serde(rename = "COL_CC")]
    ColCc,
    /// Colombia CE (Cédula de Extranjería), for individuals
    #[serde(rename = "COL_CE")]
    ColCe,
    /// Colombia NIT (Número de Identificación Tributaria), for individuals and entities
    #[serde(rename = "COL_NIT")]
    ColNit,
    /// Colombia NUIP (Número Único de Identificación Personal), for individuals
    #[serde(rename = "COL_NUIP")]
    ColNuip,
    /// Colombia PPT (Permiso por Protección Temporal), for individuals
    #[serde(rename = "COL_PPT")]
    ColPpt,
    /// Cabo Verde NIF (Número de Identificação Fiscal), for individuals and entities
    #[serde(rename = "CPV_NIF")]
    CpvNif,
    /// Costa Rica Cédula Jurídica (Legal Entity ID), for entities
    #[serde(rename = "CRI_CJ")]
    CriCj,
    /// Costa Rica DIMEX (Documento de Identificación Migratorio para Extranjeros), for individuals
    #[serde(rename = "CRI_DIMEX")]
    CriDimex,
    /// Costa Rica NITE (Número de Identificación Tributaria Especial), for individuals and entities
    #[serde(rename = "CRI_NITE")]
    CriNite,
    /// Costa Rica Personal Identification Number (PIN), for individuals
    #[serde(rename = "CRI_PIN")]
    CriPin,
    /// Curaçao CRIB (Centraal Registratie Informatie Belastingplichtige), for individuals and entities
    #[serde(rename = "CUW_CRIB")]
    CuwCrib,
    /// Cyprus TIC (Tax Identification Code), for individuals and entities
    #[serde(rename = "CYP_TIC")]
    CypTic,
    /// Czech Republic TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "CZE_TIN")]
    CzeTin,
    /// Germany Tax ID (Identifikationsnummer), for individuals
    #[serde(rename = "DEU_TAX_ID")]
    DeuTaxId,
    /// Germany USt-IdNr (Umsatzsteuer-Identifikationsnummer), for entities
    #[serde(rename = "DEU_UST_ID")]
    DeuUstId,
    /// Denmark CPR-nummer (Det Centrale Personregister), for individuals
    #[serde(rename = "DNK_CPR")]
    DnkCpr,
    /// Denmark CVR-nummer (Det Centrale Virksomhedsregister), for entities
    #[serde(rename = "DNK_CVR")]
    DnkCvr,
    /// Dominican Republic Cédula de Identidad y Electoral, for individuals
    #[serde(rename = "DOM_CED")]
    DomCed,
    /// Dominican Republic RNC (Registro Nacional del Contribuyente), for entities
    #[serde(rename = "DOM_RNC")]
    DomRnc,
    /// Algeria NIF (Numéro d'Identification Fiscale), for individuals and entities
    #[serde(rename = "DZA_NIF")]
    DzaNif,
    /// Ecuador RUC (Registro Unico de Contribuyentes), for individuals and entities
    #[serde(rename = "ECU_RUC")]
    EcuRuc,
    /// Egypt National ID Number, for individuals
    #[serde(rename = "EGY_NID")]
    EgyNid,
    /// Egypt Tax Registration Number, for entities
    #[serde(rename = "EGY_TRN")]
    EgyTrn,
    /// Spain DNI (Documento Nacional de Identidad), for individuals
    #[serde(rename = "ESP_DNI")]
    EspDni,
    /// Spain NIE (Número de Identidad de Extranjero), for individuals
    #[serde(rename = "ESP_NIE")]
    EspNie,
    /// Spain NIF (Número de Identificación Fiscal), for individuals and entities
    #[serde(rename = "ESP_NIF")]
    EspNif,
    /// Estonia Isikukood (Personal Identification Code), for individuals
    #[serde(rename = "EST_IK")]
    EstIk,
    /// Estonia Registrikood (Registry Code), for entities
    #[serde(rename = "EST_RC")]
    EstRc,
    /// Ethiopia TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "ETH_TIN")]
    EthTin,
    /// Finland HETU (Henkilötunnus), for individuals
    #[serde(rename = "FIN_HETU")]
    FinHetu,
    /// Finland Y-tunnus (Business ID), for entities
    #[serde(rename = "FIN_YTU")]
    FinYtu,
    /// Fiji TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "FJI_TIN")]
    FjiTin,
    /// France SIREN (Business tax ID), for entities
    #[serde(rename = "FRA_SIREN")]
    FraSiren,
    /// France SPI (Reference tax number), for individuals
    #[serde(rename = "FRA_SPI")]
    FraSpi,
    /// Gabon NIF (Numéro d'Identification Fiscale), for individuals and entities
    #[serde(rename = "GAB_NIF")]
    GabNif,
    /// UK CRN (Companies House Registration Number), for entities
    #[serde(rename = "GBR_CRN")]
    GbrCrn,
    /// UK NINO (National Insurance Number), for individuals
    #[serde(rename = "GBR_NINO")]
    GbrNino,
    /// UK UTR (Unique Taxpayer Reference), for individuals and entities
    #[serde(rename = "GBR_UTR")]
    GbrUtr,
    /// Georgia Personal Identification Number, for individuals
    #[serde(rename = "GEO_PID")]
    GeoPid,
    /// Georgia TIN (Tax Identification Number), for entities
    #[serde(rename = "GEO_TIN")]
    GeoTin,
    /// Ghana TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "GHA_TIN")]
    GhaTin,
    /// Greece AFM (Arithmos Forologikou Mitroou), for individuals and entities
    #[serde(rename = "GRC_AFM")]
    GrcAfm,
    /// Guatemala NIT (Número de Identificación Tributaria), for individuals and entities
    #[serde(rename = "GTM_NIT")]
    GtmNit,
    /// Hong Kong BRN (Business Registration Number), for entities
    #[serde(rename = "HKG_BRN")]
    HkgBrn,
    /// Hong Kong HKID (Hong Kong Identity Card), for individuals
    #[serde(rename = "HKG_HKID")]
    HkgHkid,
    /// Honduras RTN (Registro Tributario Nacional), for individuals and entities
    #[serde(rename = "HND_RTN")]
    HndRtn,
    /// Croatia OIB (Osobni Identifikacijski Broj), for individuals
    #[serde(rename = "HRV_OIB")]
    HrvOib,
    /// Haiti NIF (Numéro d'Identification Fiscale), for individuals and entities
    #[serde(rename = "HTI_NIF")]
    HtiNif,
    /// Hungary Adószám (Entity Tax Number), for entities
    #[serde(rename = "HUN_ENT")]
    HunEnt,
    /// Hungary TIN (Tax Identification Number), for individuals
    #[serde(rename = "HUN_TIN")]
    HunTin,
    /// Indonesia KTP (Indonesian Identity Card), for individuals
    #[serde(rename = "IDN_KTP")]
    IdnKtp,
    /// Indonesia NPWP (Nomor Pokok Wajib Pajak), for individuals and entities. The code is spelled `IDN_NWPW`, not `IDN_NPWP`.
    #[serde(rename = "IDN_NWPW")]
    IdnNwpw,
    /// India GSTIN (Goods and Services Tax Identification Number), for entities
    #[serde(rename = "IND_GSTIN")]
    IndGstin,
    /// India PAN (Permanent Account Number), for individuals
    #[serde(rename = "IND_PAN")]
    IndPan,
    /// Ireland CRO (Companies Registration Office Number), for entities
    #[serde(rename = "IRL_CRO")]
    IrlCro,
    /// Ireland PPS (Personal Public Service Number), for individuals
    #[serde(rename = "IRL_PPS")]
    IrlPps,
    /// Iran National ID (Kode Melli), for individuals
    #[serde(rename = "IRN_NID")]
    IrnNid,
    /// Iceland Kennitala (National Identification Number), for individuals
    #[serde(rename = "ISL_KT")]
    IslKt,
    /// Israel Company Number (Mispar Chevra), for entities
    #[serde(rename = "ISR_CORP")]
    IsrCorp,
    /// Israel Tax ID (Teudat Zehut), for individuals
    #[serde(rename = "ISR_TAX_ID")]
    IsrTaxId,
    /// Italy Partita IVA (VAT Number), for entities
    #[serde(rename = "ITA_PIVA")]
    ItaPiva,
    /// Italy Tax ID (Codice fiscale), for individuals
    #[serde(rename = "ITA_TAX_ID")]
    ItaTaxId,
    /// Jamaica TRN (Taxpayer Registration Number), for individuals and entities
    #[serde(rename = "JAM_TRN")]
    JamTrn,
    /// Jordan National ID Number, for individuals
    #[serde(rename = "JOR_NID")]
    JorNid,
    /// Japan Corporate Number (法人番号), for entities
    #[serde(rename = "JPN_CORP")]
    JpnCorp,
    /// Japan Tax ID (kojin bango), for individuals
    #[serde(rename = "JPN_TAX_ID")]
    JpnTaxId,
    /// Kazakhstan BIN (Business Identification Number), for entities
    #[serde(rename = "KAZ_BIN")]
    KazBin,
    /// Kazakhstan IIN (Individual Identification Number), for individuals
    #[serde(rename = "KAZ_IIN")]
    KazIin,
    /// Kenya KRA PIN (Kenya Revenue Authority PIN), for individuals and entities
    #[serde(rename = "KEN_KRA_PIN")]
    KenKraPin,
    /// Kyrgyzstan INN (Individual Tax Number), for individuals
    #[serde(rename = "KGZ_INN")]
    KgzInn,
    /// Cambodia National ID Number, for individuals
    #[serde(rename = "KHM_NID")]
    KhmNid,
    /// South Korea BRN (Business Registration Number), for entities
    #[serde(rename = "KOR_BRN")]
    KorBrn,
    /// South Korea RRN (Resident Registration Number), for individuals
    #[serde(rename = "KOR_RRN")]
    KorRrn,
    /// Kuwait Civil ID Number, for individuals
    #[serde(rename = "KWT_CID")]
    KwtCid,
    /// Lebanon TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "LBN_TIN")]
    LbnTin,
    /// Sri Lanka NIC (National Identity Card), for individuals
    #[serde(rename = "LKA_NIC")]
    LkaNic,
    /// Sri Lanka TIN (Tax Identification Number), for entities
    #[serde(rename = "LKA_TIN")]
    LkaTin,
    /// Lithuania Asmens Kodas (Personal Code), for individuals
    #[serde(rename = "LTU_AK")]
    LtuAk,
    /// Lithuania Juridinio Asmens Kodas (Legal Entity Code), for entities
    #[serde(rename = "LTU_JAK")]
    LtuJak,
    /// Luxembourg Matricule (National Identification Number), for individuals
    #[serde(rename = "LUX_NIN")]
    LuxNin,
    /// Luxembourg Numéro TVA (VAT Number), for entities
    #[serde(rename = "LUX_TVA")]
    LuxTva,
    /// Latvia Personas Kods (Personal Code), for individuals
    #[serde(rename = "LVA_PK")]
    LvaPk,
    /// Latvia Reģistrācijas Numurs (Registration Number), for entities
    #[serde(rename = "LVA_REG")]
    LvaReg,
    /// Macao Taxpayer Number, for individuals and entities
    #[serde(rename = "MAC_TIN")]
    MacTin,
    /// Morocco CIN (Carte d'Identité Nationale), for individuals
    #[serde(rename = "MAR_CIN")]
    MarCin,
    /// Morocco ICE (Identifiant Commun de l'Entreprise), for entities
    #[serde(rename = "MAR_ICE")]
    MarIce,
    /// Moldova IDNO (State Identification Number of Entity), for entities
    #[serde(rename = "MDA_IDNO")]
    MdaIdno,
    /// Moldova IDNP (Personal Identification Number), for individuals
    #[serde(rename = "MDA_IDNP")]
    MdaIdnp,
    /// Mexico CURP (Clave Única de Registro de Población), for individuals
    #[serde(rename = "MEX_CURP")]
    MexCurp,
    /// Mexico RFC (Registro Federal de Contribuyentes), for individuals and entities
    #[serde(rename = "MEX_RFC")]
    MexRfc,
    /// North Macedonia EDB (Unique Tax Number), for entities
    #[serde(rename = "MKD_EDB")]
    MkdEdb,
    /// North Macedonia EMBG (Unique Master Citizen Number), for individuals
    #[serde(rename = "MKD_EMBG")]
    MkdEmbg,
    /// Mali NIF (Numéro d'Identification Fiscale), for individuals and entities
    #[serde(rename = "MLI_NIF")]
    MliNif,
    /// Malta Identity Card Number, for individuals
    #[serde(rename = "MLT_ID")]
    MltId,
    /// Malta VAT Number, for entities
    #[serde(rename = "MLT_VAT")]
    MltVat,
    /// Montenegro Unique Master Citizen Number, for individuals
    #[serde(rename = "MNE_JMBG")]
    MneJmbg,
    /// Montenegro PIB (Tax Identification Number), for entities
    #[serde(rename = "MNE_PIB")]
    MnePib,
    /// Mongolia Register Number, for individuals
    #[serde(rename = "MNG_RD")]
    MngRd,
    /// Mozambique NUIT (Tax Identification Number), for individuals and entities
    #[serde(rename = "MOZ_NUIT")]
    MozNuit,
    /// Mauritius National Identity Card Number, for individuals
    #[serde(rename = "MUS_NID")]
    MusNid,
    /// Mauritius TAN (Entity Tax Account Number), for entities
    #[serde(rename = "MUS_TAN_ENT")]
    MusTanEnt,
    /// Mauritius TAN (Individual Tax Account Number), for individuals
    #[serde(rename = "MUS_TAN_IND")]
    MusTanInd,
    /// Malawi TPIN (Taxpayer Identification Number), for individuals and entities
    #[serde(rename = "MWI_TPIN")]
    MwiTpin,
    /// Malaysia MyKad (National Registration Identity Card), for individuals
    #[serde(rename = "MYS_MYKAD")]
    MysMykad,
    /// Malaysia ROC (Companies Commission Registration Number), for entities
    #[serde(rename = "MYS_ROC")]
    MysRoc,
    /// Namibia TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "NAM_TIN")]
    NamTin,
    /// Nigeria NIN (National Identification Number), for individuals
    #[serde(rename = "NGA_NIN")]
    NgaNin,
    /// Nigeria TIN (Tax Identification Number), for entities
    #[serde(rename = "NGA_TIN")]
    NgaTin,
    /// Nicaragua RUC (Registro Unico de Contribuyentes), for individuals and entities
    #[serde(rename = "NIC_RUC")]
    NicRuc,
    /// Netherlands Btw-identificatienummer (btw-id), for entities
    #[serde(rename = "NLD_BTW")]
    NldBtw,
    /// Netherlands KVK (Kamer van Koophandel), for entities
    #[serde(rename = "NLD_KVK")]
    NldKvk,
    /// Netherlands Omzetbelastingnummer (ob-nummer), for entities
    #[serde(rename = "NLD_OBN")]
    NldObn,
    /// Netherlands BSN (Burgerservicenummer), for individuals
    #[serde(rename = "NLD_TIN")]
    NldTin,
    /// Norway Fødselsnummer (National Identity Number), for individuals
    #[serde(rename = "NOR_FNR")]
    NorFnr,
    /// Norway Organisasjonsnummer (Organisation Number), for entities
    #[serde(rename = "NOR_ORG")]
    NorOrg,
    /// Nepal PAN (Permanent Account Number), for individuals and entities
    #[serde(rename = "NPL_PAN")]
    NplPan,
    /// New Zealand IRD Number (Inland Revenue Department), for individuals and entities
    #[serde(rename = "NZL_IRD")]
    NzlIrd,
    /// New Zealand NZBN (New Zealand Business Number), for entities
    #[serde(rename = "NZL_NZBN")]
    NzlNzbn,
    /// Oman Civil ID Number, for individuals
    #[serde(rename = "OMN_CID")]
    OmnCid,
    /// Pakistan NIC (National Identity Card), for individuals
    #[serde(rename = "PAK_NIC")]
    PakNic,
    /// Pakistan NTN (National Tax Number), for individuals and entities
    #[serde(rename = "PAK_NTN")]
    PakNtn,
    /// Panama RUC (Registro Unico de Contribuyentes), for individuals and entities
    #[serde(rename = "PAN_RUC")]
    PanRuc,
    /// Peru RUC (Registro Unico de Contribuyentes), for individuals and entities
    #[serde(rename = "PER_RUC")]
    PerRuc,
    /// Philippines TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "PHL_TIN")]
    PhlTin,
    /// Poland NIP (Numer Identyfikacji Podatkowej), for entities
    #[serde(rename = "POL_NIP")]
    PolNip,
    /// Poland PESEL (Polish National Identification Number), for individuals
    #[serde(rename = "POL_PESEL")]
    PolPesel,
    /// Portugal NIF (Número de Identificação Fiscal), for individuals and entities
    #[serde(rename = "PRT_NIF")]
    PrtNif,
    /// Paraguay RUC (Registro Unico de Contribuyentes), for individuals and entities
    #[serde(rename = "PRY_RUC")]
    PryRuc,
    /// Qatar QID (Qatar Identity Number), for individuals
    #[serde(rename = "QAT_QID")]
    QatQid,
    /// Romania CNP (Cod Numeric Personal), for individuals
    #[serde(rename = "ROU_CNP")]
    RouCnp,
    /// Romania CUI (Cod Unic de Înregistrare), for entities
    #[serde(rename = "ROU_CUI")]
    RouCui,
    /// Russia INN (Entity Tax Number), for entities
    #[serde(rename = "RUS_INN_ENT")]
    RusInnEnt,
    /// Russia INN (Individual Tax Number), for individuals
    #[serde(rename = "RUS_INN_IND")]
    RusInnInd,
    /// Rwanda TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "RWA_TIN")]
    RwaTin,
    /// Saudi Arabia TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "SAU_TIN")]
    SauTin,
    /// Saudi Arabia VAT Registration Number, for entities
    #[serde(rename = "SAU_VAT")]
    SauVat,
    /// Senegal NINEA (Numéro d'Identification Nationale des Entreprises), for entities
    #[serde(rename = "SEN_NINEA")]
    SenNinea,
    /// Singapore ASGD, for individuals
    #[serde(rename = "SGP_ASGD")]
    SgpAsgd,
    /// Singapore FIN, for individuals
    #[serde(rename = "SGP_FIN")]
    SgpFin,
    /// Singapore ITR, for individuals and entities
    #[serde(rename = "SGP_ITR")]
    SgpItr,
    /// Singapore NRIC, for individuals
    #[serde(rename = "SGP_NRIC")]
    SgpNric,
    /// Singapore UEN (Unique Entity Number), for entities
    #[serde(rename = "SGP_UEN")]
    SgpUen,
    /// El Salvador NIT (Número de Identificación Tributaria), for individuals and entities
    #[serde(rename = "SLV_NIT")]
    SlvNit,
    /// Serbia JMBG (Unique Master Citizen Number), for individuals
    #[serde(rename = "SRB_JMBG")]
    SrbJmbg,
    /// Serbia PIB (Tax Identification Number), for entities
    #[serde(rename = "SRB_PIB")]
    SrbPib,
    /// Slovakia IČO (Identification Number of Organisation), for entities
    #[serde(rename = "SVK_ICO")]
    SvkIco,
    /// Slovakia TIN (Tax Identification Number), for individuals
    #[serde(rename = "SVK_TIN")]
    SvkTin,
    /// Slovenia Davčna Številka (Tax Number), for individuals and entities
    #[serde(rename = "SVN_DAS")]
    SvnDas,
    /// Slovenia Matična Številka (Registration Number), for entities
    #[serde(rename = "SVN_MS")]
    SvnMs,
    /// Sweden Organisationsnummer (Organisation Number), for entities
    #[serde(rename = "SWE_ORG")]
    SweOrg,
    /// Sweden Tax ID (Personnummer), for individuals
    #[serde(rename = "SWE_TAX_ID")]
    SweTaxId,
    /// Eswatini TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "SWZ_TIN")]
    SwzTin,
    /// Sint Maarten CRIB (Centraal Registratie Informatie Belastingplichtige), for individuals and entities
    #[serde(rename = "SXM_CRIB")]
    SxmCrib,
    /// Syria TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "SYR_TIN")]
    SyrTin,
    /// Thailand National ID Card Number (เลขประจำตัวประชาชน), for individuals
    #[serde(rename = "THA_NID")]
    ThaNid,
    /// Thailand Tax ID (เลขประจำตัวผู้เสียภาษี), for entities
    #[serde(rename = "THA_TIN")]
    ThaTin,
    /// Tajikistan TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "TJK_TIN")]
    TjkTin,
    /// Trinidad and Tobago BIR (Board of Inland Revenue File Number), for individuals and entities
    #[serde(rename = "TTO_BIR")]
    TtoBir,
    /// Tunisia CIN (Carte d'Identité Nationale), for individuals
    #[serde(rename = "TUN_CIN")]
    TunCin,
    /// Tunisia Matricule Fiscal (Tax Identification Number), for entities
    #[serde(rename = "TUN_MF")]
    TunMf,
    /// Turkey TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "TUR_TIN")]
    TurTin,
    /// Taiwan GUI (Unified Business Number / 統一編號), for entities
    #[serde(rename = "TWN_GUI")]
    TwnGui,
    /// Taiwan National ID Number (身分證字號), for individuals
    #[serde(rename = "TWN_NID")]
    TwnNid,
    /// Tanzania TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "TZA_TIN")]
    TzaTin,
    /// Uganda TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "UGA_TIN")]
    UgaTin,
    /// Ukraine EDRPOU (Entity Identification Code), for entities
    #[serde(rename = "UKR_EDRPOU")]
    UkrEdrpou,
    /// Ukraine RNTRC (Individual Tax Number), for individuals
    #[serde(rename = "UKR_RNTRC")]
    UkrRntrc,
    /// Uruguay CI (Cédula de Identidad), for individuals
    #[serde(rename = "URY_CI")]
    UryCi,
    /// Uruguay RUT (Rol Único Tributario), for entities
    #[serde(rename = "URY_RUT")]
    UryRut,
    /// Uzbekistan INN (Entity Tax Number), for entities
    #[serde(rename = "UZB_INN")]
    UzbInn,
    /// Uzbekistan PINFL (Personal Identification Number), for individuals
    #[serde(rename = "UZB_PINFL")]
    UzbPinfl,
    /// Venezuela RIF (Registro de Información Fiscal), for individuals and entities
    #[serde(rename = "VEN_RIF")]
    VenRif,
    /// Vietnam MST Branch (Mã số thuế đơn vị phụ thuộc), for entities
    #[serde(rename = "VNM_MST_BRN")]
    VnmMstBrn,
    /// Vietnam MST Enterprise (Mã số thuế doanh nghiệp), for individuals and entities
    #[serde(rename = "VNM_MST_CORP")]
    VnmMstCorp,
    /// Vietnam PIN (Personal identification number / Số định danh cá nhân), for individuals
    #[serde(rename = "VNM_MST_PIN")]
    VnmMstPin,
    /// Samoa TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "WSM_TIN")]
    WsmTin,
    /// Kosovo Personal Identification Number, for individuals
    #[serde(rename = "XXK_PID")]
    XxkPid,
    /// South Africa Company Registration Number, for entities
    #[serde(rename = "ZAF_CRN")]
    ZafCrn,
    /// South African ID Number, for individuals
    #[serde(rename = "ZAF_ID")]
    ZafId,
    /// Zambia TPIN (Taxpayer Identification Number), for individuals and entities
    #[serde(rename = "ZMB_TPIN")]
    ZmbTpin,
    /// Zimbabwe TIN (Tax Identification Number), for individuals and entities
    #[serde(rename = "ZWE_TIN")]
    ZweTin,
    /// National ID number, if a tax ID number is not available
    #[serde(rename = "NATIONAL_ID")]
    NationalId,
    /// Passport number, if a tax ID number is not available
    #[serde(rename = "PASSPORT")]
    Passport,
    /// Permanent resident number, if a tax ID number is not available
    #[serde(rename = "PERMANENT_RESIDENT")]
    PermanentResident,
    /// Driver's license number, if a tax ID number is not available
    #[serde(rename = "DRIVER_LICENSE")]
    DriverLicense,
    /// Other government issued identifier, if a tax ID number is not available
    #[serde(rename = "OTHER_GOV_ID")]
    OtherGovId,
    /// Other Tax IDs
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

/// Agreement on an account's `agreements` (Create / Update an Account).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(rename = "custodian_customer_agreement")]
    CustodianCustomer,
    #[serde(rename = "trusted_contact_consent")]
    TrustedContactConsent,
    #[serde(rename = "master_securities_lending_agreement")]
    MasterSecuritiesLending,
    #[serde(rename = "certifications_and_resolutions_statement")]
    CertificationsAndResolutionsStatement,
    #[serde(rename = "joint_account_agreement")]
    JointAccount,
    /// Alpaca Foreign Markets (Global Equities) Addendum: sign it before the customer trades
    /// assets on non-US exchanges.
    #[serde(rename = "foreign_markets_addendum")]
    ForeignMarketsAddendum,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// `document_type` of an owner document (`POST /v1/accounts/{account_id}/documents/upload`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// How a recipient bank is identified: an `ABA` routing number or a `BIC` (SWIFT) code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum BankCodeType {
    Aba,
    Bic,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// `status` of a recipient bank.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum BankStatus {
    Queued,
    SentToClearing,
    Approved,
    Rejected,
    Canceled,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// `status` filter on `GET /v1/accounts/{account_id}/recipient_banks`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum BankStatusFilter {
    Active,
    Inactive,
}

/// Who pays a wire transfer's fees: the end `user`, or the broker via the monthly `invoice`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum FeePaymentMethod {
    User,
    Invoice,
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
    PartialLiquidation,
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
    /// Created by Alpaca, e.g. scheduled rebalancing or subscription creation.
    System,
    /// Created with `POST /v1/rebalancing/runs`.
    Api,
    /// Created by a user in the Broker API dashboard.
    Dashboard,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// How a rebalancing subscription's trades are marked. The API treats an omitted marker as
/// `Discretionary`, which it rejects unless the correspondent's advisor is marked
/// discretionary; send `Unsolicited` otherwise. Subscriptions created before markers
/// existed return `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TradeMarker {
    Discretionary,
    Unsolicited,
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

/// Entities `list_accounts` can include in each account (`entities`); omit for all of them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AccountEntity {
    Agreements,
    Contact,
    Disclosures,
    Documents,
    Identity,
    TradingConfigurations,
    TrustedContact,
}

/// Status of a rebalancing run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum RunStatus {
    Queued,
    InProgress,
    Canceled,
    CanceledMidRun,
    Error,
    Timeout,
    CompletedAdjusted,
    CompletedSuccess,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}
