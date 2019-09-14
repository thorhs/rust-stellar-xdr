// GENERATED CODE
//
// Generated from stdin by xdrgen.
//
// DO NOT EDIT

pub const MASK_ACCOUNT_FLAGS: i64 = 7i64;

pub const MASK_OFFERENTRY_FLAGS: i64 = 1i64;

pub const MASK_TRUSTLINE_FLAGS: i64 = 1i64;

pub const MAX_OPS_PER_TX: i64 = 100i64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountEntry {
    pub accountID: AccountID,
    pub balance: int64,
    pub seqNum: SequenceNumber,
    pub numSubEntries: uint32,
    pub inflationDest: Option<Box<AccountID>>,
    pub flags: uint32,
    pub homeDomain: string32,
    pub thresholds: Thresholds,
    pub signers: Vec<Signer>,
    pub ext: AccountEntryExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountEntryExt {
    Const0,
    Const1(AccountEntryExtV1),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AccountEntryExtV1 {
    pub liabilities: Liabilities,
    pub ext: AccountEntryExtV1Ext,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountEntryExtV1Ext {
    Const0,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountFlags {
    AUTH_REQUIRED_FLAG = 1isize,
    AUTH_REVOCABLE_FLAG = 2isize,
    AUTH_IMMUTABLE_FLAG = 4isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountMergeResult {
    ACCOUNT_MERGE_SUCCESS(int64),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountMergeResultCode {
    ACCOUNT_MERGE_SUCCESS = 0isize,
    ACCOUNT_MERGE_MALFORMED = -1isize,
    ACCOUNT_MERGE_NO_ACCOUNT = -2isize,
    ACCOUNT_MERGE_IMMUTABLE_SET = -3isize,
    ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4isize,
    ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5isize,
    ACCOUNT_MERGE_DEST_FULL = -6isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AllowTrustOp {
    pub trustor: AccountID,
    pub asset: AllowTrustOpAsset,
    pub authorize: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AllowTrustOpAsset {
    ASSET_TYPE_CREDIT_ALPHANUM4([u8; 4i64 as usize]),
    ASSET_TYPE_CREDIT_ALPHANUM12([u8; 12i64 as usize]),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AllowTrustResult {
    ALLOW_TRUST_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AllowTrustResultCode {
    ALLOW_TRUST_SUCCESS = 0isize,
    ALLOW_TRUST_MALFORMED = -1isize,
    ALLOW_TRUST_NO_TRUST_LINE = -2isize,
    ALLOW_TRUST_TRUST_NOT_REQUIRED = -3isize,
    ALLOW_TRUST_CANT_REVOKE = -4isize,
    ALLOW_TRUST_SELF_NOT_ALLOWED = -5isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Asset {
    ASSET_TYPE_NATIVE,
    ASSET_TYPE_CREDIT_ALPHANUM4(AssetAlphanum4),
    ASSET_TYPE_CREDIT_ALPHANUM12(AssetAlphanum12),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AssetAlphanum12 {
    pub assetCode: [u8; 12i64 as usize],
    pub issuer: AccountID,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AssetAlphanum4 {
    pub assetCode: [u8; 4i64 as usize],
    pub issuer: AccountID,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AssetType {
    ASSET_TYPE_NATIVE = 0isize,
    ASSET_TYPE_CREDIT_ALPHANUM4 = 1isize,
    ASSET_TYPE_CREDIT_ALPHANUM12 = 2isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Auth {
    pub unused: i32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthCert {
    pub pubkey: Curve25519Public,
    pub expiration: uint64,
    pub sig: Signature,
}

pub enum AuthenticatedMessage {
    Const0(AuthenticatedMessageV0),
}

pub struct AuthenticatedMessageV0 {
    pub sequence: uint64,
    pub message: StellarMessage,
    pub mac: HmacSha256Mac,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BucketEntry {
    LIVEENTRY(LedgerEntry),
    INITENTRY(LedgerEntry),
    DEADENTRY(LedgerKey),
    METAENTRY(BucketMetadata),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BucketEntryType {
    METAENTRY = -1isize,
    LIVEENTRY = 0isize,
    DEADENTRY = 1isize,
    INITENTRY = 2isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BucketMetadata {
    pub ledgerVersion: uint32,
    pub ext: BucketMetadataExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BucketMetadataExt {
    Const0,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BumpSequenceOp {
    pub bumpTo: SequenceNumber,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BumpSequenceResult {
    BUMP_SEQUENCE_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BumpSequenceResultCode {
    BUMP_SEQUENCE_SUCCESS = 0isize,
    BUMP_SEQUENCE_BAD_SEQ = -1isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ChangeTrustOp {
    pub line: Asset,
    pub limit: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChangeTrustResult {
    CHANGE_TRUST_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChangeTrustResultCode {
    CHANGE_TRUST_SUCCESS = 0isize,
    CHANGE_TRUST_MALFORMED = -1isize,
    CHANGE_TRUST_NO_ISSUER = -2isize,
    CHANGE_TRUST_INVALID_LIMIT = -3isize,
    CHANGE_TRUST_LOW_RESERVE = -4isize,
    CHANGE_TRUST_SELF_NOT_ALLOWED = -5isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ClaimOfferAtom {
    pub sellerID: AccountID,
    pub offerID: int64,
    pub assetSold: Asset,
    pub amountSold: int64,
    pub assetBought: Asset,
    pub amountBought: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CreateAccountOp {
    pub destination: AccountID,
    pub startingBalance: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateAccountResult {
    CREATE_ACCOUNT_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateAccountResultCode {
    CREATE_ACCOUNT_SUCCESS = 0isize,
    CREATE_ACCOUNT_MALFORMED = -1isize,
    CREATE_ACCOUNT_UNDERFUNDED = -2isize,
    CREATE_ACCOUNT_LOW_RESERVE = -3isize,
    CREATE_ACCOUNT_ALREADY_EXIST = -4isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CreatePassiveSellOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CryptoKeyType {
    KEY_TYPE_ED25519 = 0isize,
    KEY_TYPE_PRE_AUTH_TX = 1isize,
    KEY_TYPE_HASH_X = 2isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Curve25519Public {
    pub key: [u8; 32i64 as usize],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Curve25519Secret {
    pub key: [u8; 32i64 as usize],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataEntry {
    pub accountID: AccountID,
    pub dataName: string64,
    pub dataValue: DataValue,
    pub ext: DataEntryExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DataEntryExt {
    Const0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataValue(pub Vec<u8>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecoratedSignature {
    pub hint: SignatureHint,
    pub signature: Signature,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DontHave {
    pub type_: MessageType,
    pub reqHash: uint256,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EnvelopeType {
    ENVELOPE_TYPE_SCP = 1isize,
    ENVELOPE_TYPE_TX = 2isize,
    ENVELOPE_TYPE_AUTH = 3isize,
    ENVELOPE_TYPE_SCPVALUE = 4isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    pub code: ErrorCode,
    pub msg: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    ERR_MISC = 0isize,
    ERR_DATA = 1isize,
    ERR_CONF = 2isize,
    ERR_AUTH = 3isize,
    ERR_LOAD = 4isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Hash(pub [u8; 32i64 as usize]);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hello {
    pub ledgerVersion: uint32,
    pub overlayVersion: uint32,
    pub overlayMinVersion: uint32,
    pub networkID: Hash,
    pub versionStr: String,
    pub listeningPort: i32,
    pub peerID: NodeID,
    pub cert: AuthCert,
    pub nonce: uint256,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HmacSha256Key {
    pub key: [u8; 32i64 as usize],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HmacSha256Mac {
    pub mac: [u8; 32i64 as usize],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IPAddrType {
    IPv4 = 0isize,
    IPv6 = 1isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InflationPayout {
    pub destination: AccountID,
    pub amount: int64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InflationResult {
    INFLATION_SUCCESS(Vec<InflationPayout>),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InflationResultCode {
    INFLATION_SUCCESS = 0isize,
    INFLATION_NOT_TIME = -1isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerCloseValueSignature {
    pub nodeID: NodeID,
    pub signature: Signature,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerEntry {
    pub lastModifiedLedgerSeq: uint32,
    pub data: LedgerEntryData,
    pub ext: LedgerEntryExt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryChange {
    LEDGER_ENTRY_CREATED(LedgerEntry),
    LEDGER_ENTRY_UPDATED(LedgerEntry),
    LEDGER_ENTRY_REMOVED(LedgerKey),
    LEDGER_ENTRY_STATE(LedgerEntry),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryChangeType {
    LEDGER_ENTRY_CREATED = 0isize,
    LEDGER_ENTRY_UPDATED = 1isize,
    LEDGER_ENTRY_REMOVED = 2isize,
    LEDGER_ENTRY_STATE = 3isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerEntryChanges(pub Vec<LedgerEntryChange>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryData {
    ACCOUNT(AccountEntry),
    TRUSTLINE(TrustLineEntry),
    OFFER(OfferEntry),
    DATA(DataEntry),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryExt {
    Const0,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerEntryType {
    ACCOUNT = 0isize,
    TRUSTLINE = 1isize,
    OFFER = 2isize,
    DATA = 3isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerHeader {
    pub ledgerVersion: uint32,
    pub previousLedgerHash: Hash,
    pub scpValue: StellarValue,
    pub txSetResultHash: Hash,
    pub bucketListHash: Hash,
    pub ledgerSeq: uint32,
    pub totalCoins: int64,
    pub feePool: int64,
    pub inflationSeq: uint32,
    pub idPool: uint64,
    pub baseFee: uint32,
    pub baseReserve: uint32,
    pub maxTxSetSize: uint32,
    pub skipList: [Hash; 4i64 as usize],
    pub ext: LedgerHeaderExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerHeaderExt {
    Const0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerHeaderHistoryEntry {
    pub hash: Hash,
    pub header: LedgerHeader,
    pub ext: LedgerHeaderHistoryEntryExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerHeaderHistoryEntryExt {
    Const0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerKey {
    ACCOUNT(LedgerKeyAccount),
    TRUSTLINE(LedgerKeyTrustline),
    OFFER(LedgerKeyOffer),
    DATA(LedgerKeyData),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LedgerKeyAccount {
    pub accountID: AccountID,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerKeyData {
    pub accountID: AccountID,
    pub dataName: string64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LedgerKeyOffer {
    pub sellerID: AccountID,
    pub offerID: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LedgerKeyTrustline {
    pub accountID: AccountID,
    pub asset: Asset,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerSCPMessages {
    pub ledgerSeq: uint32,
    pub messages: Vec<SCPEnvelope>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerUpgrade {
    LEDGER_UPGRADE_VERSION(uint32),
    LEDGER_UPGRADE_BASE_FEE(uint32),
    LEDGER_UPGRADE_MAX_TX_SET_SIZE(uint32),
    LEDGER_UPGRADE_BASE_RESERVE(uint32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LedgerUpgradeType {
    LEDGER_UPGRADE_VERSION = 1isize,
    LEDGER_UPGRADE_BASE_FEE = 2isize,
    LEDGER_UPGRADE_MAX_TX_SET_SIZE = 3isize,
    LEDGER_UPGRADE_BASE_RESERVE = 4isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Liabilities {
    pub buying: int64,
    pub selling: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ManageBuyOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub buyAmount: int64,
    pub price: Price,
    pub offerID: int64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManageBuyOfferResult {
    MANAGE_BUY_OFFER_SUCCESS(ManageOfferSuccessResult),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageBuyOfferResultCode {
    MANAGE_BUY_OFFER_SUCCESS = 0isize,
    MANAGE_BUY_OFFER_MALFORMED = -1isize,
    MANAGE_BUY_OFFER_SELL_NO_TRUST = -2isize,
    MANAGE_BUY_OFFER_BUY_NO_TRUST = -3isize,
    MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED = -4isize,
    MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED = -5isize,
    MANAGE_BUY_OFFER_LINE_FULL = -6isize,
    MANAGE_BUY_OFFER_UNDERFUNDED = -7isize,
    MANAGE_BUY_OFFER_CROSS_SELF = -8isize,
    MANAGE_BUY_OFFER_SELL_NO_ISSUER = -9isize,
    MANAGE_BUY_OFFER_BUY_NO_ISSUER = -10isize,
    MANAGE_BUY_OFFER_NOT_FOUND = -11isize,
    MANAGE_BUY_OFFER_LOW_RESERVE = -12isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManageDataOp {
    pub dataName: string64,
    pub dataValue: Option<DataValue>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageDataResult {
    MANAGE_DATA_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageDataResultCode {
    MANAGE_DATA_SUCCESS = 0isize,
    MANAGE_DATA_NOT_SUPPORTED_YET = -1isize,
    MANAGE_DATA_NAME_NOT_FOUND = -2isize,
    MANAGE_DATA_LOW_RESERVE = -3isize,
    MANAGE_DATA_INVALID_NAME = -4isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageOfferEffect {
    MANAGE_OFFER_CREATED = 0isize,
    MANAGE_OFFER_UPDATED = 1isize,
    MANAGE_OFFER_DELETED = 2isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManageOfferSuccessResult {
    pub offersClaimed: Vec<ClaimOfferAtom>,
    pub offer: ManageOfferSuccessResultOffer,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageOfferSuccessResultOffer {
    MANAGE_OFFER_CREATED(OfferEntry),
    MANAGE_OFFER_UPDATED(OfferEntry),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ManageSellOfferOp {
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
    pub offerID: int64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManageSellOfferResult {
    MANAGE_SELL_OFFER_SUCCESS(ManageOfferSuccessResult),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ManageSellOfferResultCode {
    MANAGE_SELL_OFFER_SUCCESS = 0isize,
    MANAGE_SELL_OFFER_MALFORMED = -1isize,
    MANAGE_SELL_OFFER_SELL_NO_TRUST = -2isize,
    MANAGE_SELL_OFFER_BUY_NO_TRUST = -3isize,
    MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED = -4isize,
    MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED = -5isize,
    MANAGE_SELL_OFFER_LINE_FULL = -6isize,
    MANAGE_SELL_OFFER_UNDERFUNDED = -7isize,
    MANAGE_SELL_OFFER_CROSS_SELF = -8isize,
    MANAGE_SELL_OFFER_SELL_NO_ISSUER = -9isize,
    MANAGE_SELL_OFFER_BUY_NO_ISSUER = -10isize,
    MANAGE_SELL_OFFER_NOT_FOUND = -11isize,
    MANAGE_SELL_OFFER_LOW_RESERVE = -12isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Memo {
    MEMO_NONE,
    MEMO_TEXT(String),
    MEMO_ID(uint64),
    MEMO_HASH(Hash),
    MEMO_RETURN(Hash),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoType {
    MEMO_NONE = 0isize,
    MEMO_TEXT = 1isize,
    MEMO_ID = 2isize,
    MEMO_HASH = 3isize,
    MEMO_RETURN = 4isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    ERROR_MSG = 0isize,
    AUTH = 2isize,
    DONT_HAVE = 3isize,
    GET_PEERS = 4isize,
    PEERS = 5isize,
    GET_TX_SET = 6isize,
    TX_SET = 7isize,
    TRANSACTION = 8isize,
    GET_SCP_QUORUMSET = 9isize,
    SCP_QUORUMSET = 10isize,
    SCP_MESSAGE = 11isize,
    GET_SCP_STATE = 12isize,
    HELLO = 13isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OfferEntry {
    pub sellerID: AccountID,
    pub offerID: int64,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: int64,
    pub price: Price,
    pub flags: uint32,
    pub ext: OfferEntryExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OfferEntryExt {
    Const0,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OfferEntryFlags {
    PASSIVE_FLAG = 1isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Operation {
    pub sourceAccount: Option<Box<AccountID>>,
    pub body: OperationBody,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationBody {
    CREATE_ACCOUNT(CreateAccountOp),
    PAYMENT(PaymentOp),
    PATH_PAYMENT(PathPaymentOp),
    MANAGE_SELL_OFFER(ManageSellOfferOp),
    CREATE_PASSIVE_SELL_OFFER(CreatePassiveSellOfferOp),
    SET_OPTIONS(SetOptionsOp),
    CHANGE_TRUST(ChangeTrustOp),
    ALLOW_TRUST(AllowTrustOp),
    ACCOUNT_MERGE(AccountID),
    INFLATION,
    MANAGE_DATA(ManageDataOp),
    BUMP_SEQUENCE(BumpSequenceOp),
    MANAGE_BUY_OFFER(ManageBuyOfferOp),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationMeta {
    pub changes: LedgerEntryChanges,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationResult {
    opINNER(OperationResultTr),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OperationResultCode {
    opINNER = 0isize,
    opBAD_AUTH = -1isize,
    opNO_ACCOUNT = -2isize,
    opNOT_SUPPORTED = -3isize,
    opTOO_MANY_SUBENTRIES = -4isize,
    opEXCEEDED_WORK_LIMIT = -5isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationResultTr {
    CREATE_ACCOUNT(CreateAccountResult),
    PAYMENT(PaymentResult),
    PATH_PAYMENT(PathPaymentResult),
    MANAGE_SELL_OFFER(ManageSellOfferResult),
    CREATE_PASSIVE_SELL_OFFER(ManageSellOfferResult),
    SET_OPTIONS(SetOptionsResult),
    CHANGE_TRUST(ChangeTrustResult),
    ALLOW_TRUST(AllowTrustResult),
    ACCOUNT_MERGE(AccountMergeResult),
    INFLATION(InflationResult),
    MANAGE_DATA(ManageDataResult),
    BUMP_SEQUENCE(BumpSequenceResult),
    MANAGE_BUY_OFFER(ManageBuyOfferResult),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OperationType {
    CREATE_ACCOUNT = 0isize,
    PAYMENT = 1isize,
    PATH_PAYMENT = 2isize,
    MANAGE_SELL_OFFER = 3isize,
    CREATE_PASSIVE_SELL_OFFER = 4isize,
    SET_OPTIONS = 5isize,
    CHANGE_TRUST = 6isize,
    ALLOW_TRUST = 7isize,
    ACCOUNT_MERGE = 8isize,
    INFLATION = 9isize,
    MANAGE_DATA = 10isize,
    BUMP_SEQUENCE = 11isize,
    MANAGE_BUY_OFFER = 12isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathPaymentOp {
    pub sendAsset: Asset,
    pub sendMax: int64,
    pub destination: AccountID,
    pub destAsset: Asset,
    pub destAmount: int64,
    pub path: Vec<Asset>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PathPaymentResult {
    PATH_PAYMENT_SUCCESS(PathPaymentResultSuccess),
    PATH_PAYMENT_NO_ISSUER(Asset),
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PathPaymentResultCode {
    PATH_PAYMENT_SUCCESS = 0isize,
    PATH_PAYMENT_MALFORMED = -1isize,
    PATH_PAYMENT_UNDERFUNDED = -2isize,
    PATH_PAYMENT_SRC_NO_TRUST = -3isize,
    PATH_PAYMENT_SRC_NOT_AUTHORIZED = -4isize,
    PATH_PAYMENT_NO_DESTINATION = -5isize,
    PATH_PAYMENT_NO_TRUST = -6isize,
    PATH_PAYMENT_NOT_AUTHORIZED = -7isize,
    PATH_PAYMENT_LINE_FULL = -8isize,
    PATH_PAYMENT_NO_ISSUER = -9isize,
    PATH_PAYMENT_TOO_FEW_OFFERS = -10isize,
    PATH_PAYMENT_OFFER_CROSS_SELF = -11isize,
    PATH_PAYMENT_OVER_SENDMAX = -12isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathPaymentResultSuccess {
    pub offers: Vec<ClaimOfferAtom>,
    pub last: SimplePaymentResult,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PaymentOp {
    pub destination: AccountID,
    pub asset: Asset,
    pub amount: int64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentResult {
    PAYMENT_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentResultCode {
    PAYMENT_SUCCESS = 0isize,
    PAYMENT_MALFORMED = -1isize,
    PAYMENT_UNDERFUNDED = -2isize,
    PAYMENT_SRC_NO_TRUST = -3isize,
    PAYMENT_SRC_NOT_AUTHORIZED = -4isize,
    PAYMENT_NO_DESTINATION = -5isize,
    PAYMENT_NO_TRUST = -6isize,
    PAYMENT_NOT_AUTHORIZED = -7isize,
    PAYMENT_LINE_FULL = -8isize,
    PAYMENT_NO_ISSUER = -9isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PeerAddress {
    pub ip: PeerAddressIp,
    pub port: uint32,
    pub numFailures: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PeerAddressIp {
    IPv4([u8; 4i64 as usize]),
    IPv6([u8; 16i64 as usize]),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Price {
    pub n: int32,
    pub d: int32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PublicKey {
    PUBLIC_KEY_TYPE_ED25519(uint256),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PublicKeyType {
    PUBLIC_KEY_TYPE_ED25519 = 0isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCPBallot {
    pub counter: uint32,
    pub value: Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCPEnvelope {
    pub statement: SCPStatement,
    pub signature: Signature,
}

pub enum SCPHistoryEntry {
    Const0(SCPHistoryEntryV0),
}

pub struct SCPHistoryEntryV0 {
    pub quorumSets: Vec<SCPQuorumSet>,
    pub ledgerMessages: LedgerSCPMessages,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCPNomination {
    pub quorumSetHash: Hash,
    pub votes: Vec<Value>,
    pub accepted: Vec<Value>,
}

pub struct SCPQuorumSet {
    pub threshold: uint32,
    pub validators: Vec<PublicKey>,
    pub innerSets: Vec<SCPQuorumSet>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCPStatement {
    pub nodeID: NodeID,
    pub slotIndex: uint64,
    pub pledges: SCPStatementPledges,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SCPStatementPledges {
    SCP_ST_PREPARE(SCPStatementPledgesPrepare),
    SCP_ST_CONFIRM(SCPStatementPledgesConfirm),
    SCP_ST_EXTERNALIZE(SCPStatementPledgesExternalize),
    SCP_ST_NOMINATE(SCPNomination),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCPStatementPledgesConfirm {
    pub ballot: SCPBallot,
    pub nPrepared: uint32,
    pub nCommit: uint32,
    pub nH: uint32,
    pub quorumSetHash: Hash,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCPStatementPledgesExternalize {
    pub commit: SCPBallot,
    pub nH: uint32,
    pub commitQuorumSetHash: Hash,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SCPStatementPledgesPrepare {
    pub quorumSetHash: Hash,
    pub ballot: SCPBallot,
    pub prepared: Option<Box<SCPBallot>>,
    pub preparedPrime: Option<Box<SCPBallot>>,
    pub nC: uint32,
    pub nH: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SCPStatementType {
    SCP_ST_PREPARE = 0isize,
    SCP_ST_CONFIRM = 1isize,
    SCP_ST_EXTERNALIZE = 2isize,
    SCP_ST_NOMINATE = 3isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetOptionsOp {
    pub inflationDest: Option<Box<AccountID>>,
    pub clearFlags: Option<uint32>,
    pub setFlags: Option<uint32>,
    pub masterWeight: Option<uint32>,
    pub lowThreshold: Option<uint32>,
    pub medThreshold: Option<uint32>,
    pub highThreshold: Option<uint32>,
    pub homeDomain: Option<string32>,
    pub signer: Option<Box<Signer>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetOptionsResult {
    SET_OPTIONS_SUCCESS,
    default,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetOptionsResultCode {
    SET_OPTIONS_SUCCESS = 0isize,
    SET_OPTIONS_LOW_RESERVE = -1isize,
    SET_OPTIONS_TOO_MANY_SIGNERS = -2isize,
    SET_OPTIONS_BAD_FLAGS = -3isize,
    SET_OPTIONS_INVALID_INFLATION = -4isize,
    SET_OPTIONS_CANT_CHANGE = -5isize,
    SET_OPTIONS_UNKNOWN_FLAG = -6isize,
    SET_OPTIONS_THRESHOLD_OUT_OF_RANGE = -7isize,
    SET_OPTIONS_BAD_SIGNER = -8isize,
    SET_OPTIONS_INVALID_HOME_DOMAIN = -9isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature(pub Vec<u8>);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SignatureHint(pub [u8; 4i64 as usize]);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Signer {
    pub key: SignerKey,
    pub weight: uint32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SignerKey {
    SIGNER_KEY_TYPE_ED25519(uint256),
    SIGNER_KEY_TYPE_PRE_AUTH_TX(uint256),
    SIGNER_KEY_TYPE_HASH_X(uint256),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SignerKeyType {
    SIGNER_KEY_TYPE_ED25519 = 0isize,
    SIGNER_KEY_TYPE_PRE_AUTH_TX = 1isize,
    SIGNER_KEY_TYPE_HASH_X = 2isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SimplePaymentResult {
    pub destination: AccountID,
    pub asset: Asset,
    pub amount: int64,
}

pub enum StellarMessage {
    ERROR_MSG(Error),
    HELLO(Hello),
    AUTH(Auth),
    DONT_HAVE(DontHave),
    GET_PEERS,
    PEERS(Vec<PeerAddress>),
    GET_TX_SET(uint256),
    TX_SET(TransactionSet),
    TRANSACTION(TransactionEnvelope),
    GET_SCP_QUORUMSET(uint256),
    SCP_QUORUMSET(SCPQuorumSet),
    SCP_MESSAGE(SCPEnvelope),
    GET_SCP_STATE(uint32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StellarValue {
    pub txSetHash: Hash,
    pub closeTime: TimePoint,
    pub upgrades: Vec<UpgradeType>,
    pub ext: StellarValueExt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StellarValueExt {
    STELLAR_VALUE_BASIC,
    STELLAR_VALUE_SIGNED(LedgerCloseValueSignature),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StellarValueType {
    STELLAR_VALUE_BASIC = 0isize,
    STELLAR_VALUE_SIGNED = 1isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ThresholdIndexes {
    THRESHOLD_MASTER_WEIGHT = 0isize,
    THRESHOLD_LOW = 1isize,
    THRESHOLD_MED = 2isize,
    THRESHOLD_HIGH = 3isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Thresholds(pub [u8; 4i64 as usize]);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TimeBounds {
    pub minTime: TimePoint,
    pub maxTime: TimePoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    pub sourceAccount: AccountID,
    pub fee: uint32,
    pub seqNum: SequenceNumber,
    pub timeBounds: Option<Box<TimeBounds>>,
    pub memo: Memo,
    pub operations: Vec<Operation>,
    pub ext: TransactionExt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionEnvelope {
    pub tx: Transaction,
    pub signatures: Vec<DecoratedSignature>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionExt {
    Const0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionHistoryEntry {
    pub ledgerSeq: uint32,
    pub txSet: TransactionSet,
    pub ext: TransactionHistoryEntryExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionHistoryEntryExt {
    Const0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionHistoryResultEntry {
    pub ledgerSeq: uint32,
    pub txResultSet: TransactionResultSet,
    pub ext: TransactionHistoryResultEntryExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionHistoryResultEntryExt {
    Const0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionMeta {
    Const0(Vec<OperationMeta>),
    Const1(TransactionMetaV1),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionMetaV1 {
    pub txChanges: LedgerEntryChanges,
    pub operations: Vec<OperationMeta>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionResult {
    pub feeCharged: int64,
    pub result: TransactionResultResult,
    pub ext: TransactionResultExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionResultCode {
    txSUCCESS = 0isize,
    txFAILED = -1isize,
    txTOO_EARLY = -2isize,
    txTOO_LATE = -3isize,
    txMISSING_OPERATION = -4isize,
    txBAD_SEQ = -5isize,
    txBAD_AUTH = -6isize,
    txINSUFFICIENT_BALANCE = -7isize,
    txNO_ACCOUNT = -8isize,
    txINSUFFICIENT_FEE = -9isize,
    txBAD_AUTH_EXTRA = -10isize,
    txINTERNAL_ERROR = -11isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransactionResultExt {
    Const0,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionResultPair {
    pub transactionHash: Hash,
    pub result: TransactionResult,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionResultResult {
    txSUCCESS(Vec<OperationResult>),
    txFAILED(Vec<OperationResult>),
    default,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionResultSet {
    pub results: Vec<TransactionResultPair>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionSet {
    pub previousLedgerHash: Hash,
    pub txs: Vec<TransactionEnvelope>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionSignaturePayload {
    pub networkId: Hash,
    pub taggedTransaction: TransactionSignaturePayloadTaggedTransaction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionSignaturePayloadTaggedTransaction {
    ENVELOPE_TYPE_TX(Transaction),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TrustLineEntry {
    pub accountID: AccountID,
    pub asset: Asset,
    pub balance: int64,
    pub limit: int64,
    pub flags: uint32,
    pub ext: TrustLineEntryExt,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrustLineEntryExt {
    Const0,
    Const1(TrustLineEntryExtV1),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TrustLineEntryExtV1 {
    pub liabilities: Liabilities,
    pub ext: TrustLineEntryExtV1Ext,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrustLineEntryExtV1Ext {
    Const0,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrustLineFlags {
    AUTHORIZED_FLAG = 1isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpgradeType(pub Vec<u8>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Value(pub Vec<u8>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct string32(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct string64(pub String);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct uint256(pub [u8; 32i64 as usize]);

pub type AccountID = PublicKey;

pub type NodeID = PublicKey;

pub type SequenceNumber = int64;

pub type TimePoint = uint64;

pub type int32 = i32;

pub type int64 = i64;

pub type uint32 = u32;

pub type uint64 = u64;

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)?
            + self.balance.pack(out)?
            + self.seqNum.pack(out)?
            + self.numSubEntries.pack(out)?
            + self.inflationDest.pack(out)?
            + self.flags.pack(out)?
            + self.homeDomain.pack(out)?
            + self.thresholds.pack(out)?
            + xdr_codec::pack_flex(&self.signers, Some(20i64 as usize), out)?
            + self.ext.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountEntryExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &AccountEntryExt::Const0 => (0i64 as i32).pack(out)?,
            &AccountEntryExt::Const1(ref val) => (1i64 as i32).pack(out)? + val.pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountEntryExtV1 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.liabilities.pack(out)? + self.ext.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountEntryExtV1Ext {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &AccountEntryExtV1Ext::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountFlags {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountMergeResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &AccountMergeResult::ACCOUNT_MERGE_SUCCESS(ref val) => {
                (AccountMergeResultCode::ACCOUNT_MERGE_SUCCESS as i32).pack(out)? + val.pack(out)?
            }
            &AccountMergeResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AccountMergeResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AllowTrustOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.trustor.pack(out)? + self.asset.pack(out)? + self.authorize.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AllowTrustOpAsset {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &AllowTrustOpAsset::ASSET_TYPE_CREDIT_ALPHANUM4(ref val) => {
                (AssetType::ASSET_TYPE_CREDIT_ALPHANUM4 as i32).pack(out)?
                    + xdr_codec::pack_opaque_array(&val[..], val.len(), out)?
            }
            &AllowTrustOpAsset::ASSET_TYPE_CREDIT_ALPHANUM12(ref val) => {
                (AssetType::ASSET_TYPE_CREDIT_ALPHANUM12 as i32).pack(out)?
                    + xdr_codec::pack_opaque_array(&val[..], val.len(), out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AllowTrustResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &AllowTrustResult::ALLOW_TRUST_SUCCESS => {
                (AllowTrustResultCode::ALLOW_TRUST_SUCCESS as i32).pack(out)?
            }
            &AllowTrustResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AllowTrustResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Asset {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &Asset::ASSET_TYPE_NATIVE => (AssetType::ASSET_TYPE_NATIVE as i32).pack(out)?,
            &Asset::ASSET_TYPE_CREDIT_ALPHANUM4(ref val) => {
                (AssetType::ASSET_TYPE_CREDIT_ALPHANUM4 as i32).pack(out)? + val.pack(out)?
            }
            &Asset::ASSET_TYPE_CREDIT_ALPHANUM12(ref val) => {
                (AssetType::ASSET_TYPE_CREDIT_ALPHANUM12 as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AssetAlphanum12 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(
            xdr_codec::pack_opaque_array(&self.assetCode[..], self.assetCode.len(), out)?
                + self.issuer.pack(out)?
                + 0,
        )
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AssetAlphanum4 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(
            xdr_codec::pack_opaque_array(&self.assetCode[..], self.assetCode.len(), out)?
                + self.issuer.pack(out)?
                + 0,
        )
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AssetType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Auth {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.unused.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AuthCert {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.pubkey.pack(out)? + self.expiration.pack(out)? + self.sig.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AuthenticatedMessage {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &AuthenticatedMessage::Const0(ref val) => (0i64 as i32).pack(out)? + val.pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for AuthenticatedMessageV0 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sequence.pack(out)? + self.message.pack(out)? + self.mac.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BucketEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &BucketEntry::LIVEENTRY(ref val) => {
                (BucketEntryType::LIVEENTRY as i32).pack(out)? + val.pack(out)?
            }
            &BucketEntry::INITENTRY(ref val) => {
                (BucketEntryType::INITENTRY as i32).pack(out)? + val.pack(out)?
            }
            &BucketEntry::DEADENTRY(ref val) => {
                (BucketEntryType::DEADENTRY as i32).pack(out)? + val.pack(out)?
            }
            &BucketEntry::METAENTRY(ref val) => {
                (BucketEntryType::METAENTRY as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BucketEntryType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BucketMetadata {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerVersion.pack(out)? + self.ext.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BucketMetadataExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &BucketMetadataExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BumpSequenceOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.bumpTo.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BumpSequenceResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &BumpSequenceResult::BUMP_SEQUENCE_SUCCESS => {
                (BumpSequenceResultCode::BUMP_SEQUENCE_SUCCESS as i32).pack(out)?
            }
            &BumpSequenceResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for BumpSequenceResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ChangeTrustOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.line.pack(out)? + self.limit.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ChangeTrustResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ChangeTrustResult::CHANGE_TRUST_SUCCESS => {
                (ChangeTrustResultCode::CHANGE_TRUST_SUCCESS as i32).pack(out)?
            }
            &ChangeTrustResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ChangeTrustResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ClaimOfferAtom {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sellerID.pack(out)?
            + self.offerID.pack(out)?
            + self.assetSold.pack(out)?
            + self.amountSold.pack(out)?
            + self.assetBought.pack(out)?
            + self.amountBought.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CreateAccountOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.destination.pack(out)? + self.startingBalance.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CreateAccountResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &CreateAccountResult::CREATE_ACCOUNT_SUCCESS => {
                (CreateAccountResultCode::CREATE_ACCOUNT_SUCCESS as i32).pack(out)?
            }
            &CreateAccountResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CreateAccountResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CreatePassiveSellOfferOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.selling.pack(out)?
            + self.buying.pack(out)?
            + self.amount.pack(out)?
            + self.price.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for CryptoKeyType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Curve25519Public {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(&self.key[..], self.key.len(), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Curve25519Secret {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(&self.key[..], self.key.len(), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DataEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)?
            + self.dataName.pack(out)?
            + self.dataValue.pack(out)?
            + self.ext.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DataEntryExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &DataEntryExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DataValue {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(
            &self.0,
            Some(64i64 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DecoratedSignature {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.hint.pack(out)? + self.signature.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for DontHave {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.type_.pack(out)? + self.reqHash.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for EnvelopeType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Error {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.code.pack(out)?
            + xdr_codec::pack_string(&self.msg, Some(100i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ErrorCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Hash {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Hello {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerVersion.pack(out)?
            + self.overlayVersion.pack(out)?
            + self.overlayMinVersion.pack(out)?
            + self.networkID.pack(out)?
            + xdr_codec::pack_string(&self.versionStr, Some(100i64 as usize), out)?
            + self.listeningPort.pack(out)?
            + self.peerID.pack(out)?
            + self.cert.pack(out)?
            + self.nonce.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for HmacSha256Key {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(&self.key[..], self.key.len(), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for HmacSha256Mac {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(&self.mac[..], self.mac.len(), out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for IPAddrType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for InflationPayout {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.destination.pack(out)? + self.amount.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for InflationResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &InflationResult::INFLATION_SUCCESS(ref val) => {
                (InflationResultCode::INFLATION_SUCCESS as i32).pack(out)?
                    + xdr_codec::pack_flex(&val, None, out)?
            }
            &InflationResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for InflationResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerCloseValueSignature {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.nodeID.pack(out)? + self.signature.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.lastModifiedLedgerSeq.pack(out)? + self.data.pack(out)? + self.ext.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryChange {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerEntryChange::LEDGER_ENTRY_CREATED(ref val) => {
                (LedgerEntryChangeType::LEDGER_ENTRY_CREATED as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryChange::LEDGER_ENTRY_UPDATED(ref val) => {
                (LedgerEntryChangeType::LEDGER_ENTRY_UPDATED as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryChange::LEDGER_ENTRY_REMOVED(ref val) => {
                (LedgerEntryChangeType::LEDGER_ENTRY_REMOVED as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryChange::LEDGER_ENTRY_STATE(ref val) => {
                (LedgerEntryChangeType::LEDGER_ENTRY_STATE as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryChangeType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryChanges {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.0, None, out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryData {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerEntryData::ACCOUNT(ref val) => {
                (LedgerEntryType::ACCOUNT as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryData::TRUSTLINE(ref val) => {
                (LedgerEntryType::TRUSTLINE as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryData::OFFER(ref val) => {
                (LedgerEntryType::OFFER as i32).pack(out)? + val.pack(out)?
            }
            &LedgerEntryData::DATA(ref val) => {
                (LedgerEntryType::DATA as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerEntryExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerEntryType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerHeader {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerVersion.pack(out)?
            + self.previousLedgerHash.pack(out)?
            + self.scpValue.pack(out)?
            + self.txSetResultHash.pack(out)?
            + self.bucketListHash.pack(out)?
            + self.ledgerSeq.pack(out)?
            + self.totalCoins.pack(out)?
            + self.feePool.pack(out)?
            + self.inflationSeq.pack(out)?
            + self.idPool.pack(out)?
            + self.baseFee.pack(out)?
            + self.baseReserve.pack(out)?
            + self.maxTxSetSize.pack(out)?
            + xdr_codec::pack_array(&self.skipList[..], self.skipList.len(), out, None)?
            + self.ext.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerHeaderExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerHeaderExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerHeaderHistoryEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.hash.pack(out)? + self.header.pack(out)? + self.ext.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerHeaderHistoryEntryExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerHeaderHistoryEntryExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKey {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerKey::ACCOUNT(ref val) => {
                (LedgerEntryType::ACCOUNT as i32).pack(out)? + val.pack(out)?
            }
            &LedgerKey::TRUSTLINE(ref val) => {
                (LedgerEntryType::TRUSTLINE as i32).pack(out)? + val.pack(out)?
            }
            &LedgerKey::OFFER(ref val) => {
                (LedgerEntryType::OFFER as i32).pack(out)? + val.pack(out)?
            }
            &LedgerKey::DATA(ref val) => {
                (LedgerEntryType::DATA as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKeyAccount {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKeyData {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)? + self.dataName.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKeyOffer {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sellerID.pack(out)? + self.offerID.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerKeyTrustline {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)? + self.asset.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerSCPMessages {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerSeq.pack(out)? + xdr_codec::pack_flex(&self.messages, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerUpgrade {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &LedgerUpgrade::LEDGER_UPGRADE_VERSION(ref val) => {
                (LedgerUpgradeType::LEDGER_UPGRADE_VERSION as i32).pack(out)? + val.pack(out)?
            }
            &LedgerUpgrade::LEDGER_UPGRADE_BASE_FEE(ref val) => {
                (LedgerUpgradeType::LEDGER_UPGRADE_BASE_FEE as i32).pack(out)? + val.pack(out)?
            }
            &LedgerUpgrade::LEDGER_UPGRADE_MAX_TX_SET_SIZE(ref val) => {
                (LedgerUpgradeType::LEDGER_UPGRADE_MAX_TX_SET_SIZE as i32).pack(out)?
                    + val.pack(out)?
            }
            &LedgerUpgrade::LEDGER_UPGRADE_BASE_RESERVE(ref val) => {
                (LedgerUpgradeType::LEDGER_UPGRADE_BASE_RESERVE as i32).pack(out)?
                    + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for LedgerUpgradeType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Liabilities {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.buying.pack(out)? + self.selling.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageBuyOfferOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.selling.pack(out)?
            + self.buying.pack(out)?
            + self.buyAmount.pack(out)?
            + self.price.pack(out)?
            + self.offerID.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageBuyOfferResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ManageBuyOfferResult::MANAGE_BUY_OFFER_SUCCESS(ref val) => {
                (ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SUCCESS as i32).pack(out)?
                    + val.pack(out)?
            }
            &ManageBuyOfferResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageBuyOfferResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageDataOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.dataName.pack(out)? + self.dataValue.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageDataResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ManageDataResult::MANAGE_DATA_SUCCESS => {
                (ManageDataResultCode::MANAGE_DATA_SUCCESS as i32).pack(out)?
            }
            &ManageDataResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageDataResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageOfferEffect {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageOfferSuccessResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.offersClaimed, None, out)? + self.offer.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageOfferSuccessResultOffer {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ManageOfferSuccessResultOffer::MANAGE_OFFER_CREATED(ref val) => {
                (ManageOfferEffect::MANAGE_OFFER_CREATED as i32).pack(out)? + val.pack(out)?
            }
            &ManageOfferSuccessResultOffer::MANAGE_OFFER_UPDATED(ref val) => {
                (ManageOfferEffect::MANAGE_OFFER_UPDATED as i32).pack(out)? + val.pack(out)?
            }
            &ManageOfferSuccessResultOffer::default => {
                return Err(xdr_codec::Error::invalidcase(-1))
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageSellOfferOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.selling.pack(out)?
            + self.buying.pack(out)?
            + self.amount.pack(out)?
            + self.price.pack(out)?
            + self.offerID.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageSellOfferResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &ManageSellOfferResult::MANAGE_SELL_OFFER_SUCCESS(ref val) => {
                (ManageSellOfferResultCode::MANAGE_SELL_OFFER_SUCCESS as i32).pack(out)?
                    + val.pack(out)?
            }
            &ManageSellOfferResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ManageSellOfferResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Memo {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &Memo::MEMO_NONE => (MemoType::MEMO_NONE as i32).pack(out)?,
            &Memo::MEMO_TEXT(ref val) => {
                (MemoType::MEMO_TEXT as i32).pack(out)?
                    + xdr_codec::pack_string(&val, Some(28i64 as usize), out)?
            }
            &Memo::MEMO_ID(ref val) => (MemoType::MEMO_ID as i32).pack(out)? + val.pack(out)?,
            &Memo::MEMO_HASH(ref val) => (MemoType::MEMO_HASH as i32).pack(out)? + val.pack(out)?,
            &Memo::MEMO_RETURN(ref val) => {
                (MemoType::MEMO_RETURN as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MemoType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for MessageType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OfferEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sellerID.pack(out)?
            + self.offerID.pack(out)?
            + self.selling.pack(out)?
            + self.buying.pack(out)?
            + self.amount.pack(out)?
            + self.price.pack(out)?
            + self.flags.pack(out)?
            + self.ext.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OfferEntryExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &OfferEntryExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OfferEntryFlags {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Operation {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sourceAccount.pack(out)? + self.body.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationBody {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &OperationBody::CREATE_ACCOUNT(ref val) => {
                (OperationType::CREATE_ACCOUNT as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::PAYMENT(ref val) => {
                (OperationType::PAYMENT as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::PATH_PAYMENT(ref val) => {
                (OperationType::PATH_PAYMENT as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::MANAGE_SELL_OFFER(ref val) => {
                (OperationType::MANAGE_SELL_OFFER as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::CREATE_PASSIVE_SELL_OFFER(ref val) => {
                (OperationType::CREATE_PASSIVE_SELL_OFFER as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::SET_OPTIONS(ref val) => {
                (OperationType::SET_OPTIONS as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::CHANGE_TRUST(ref val) => {
                (OperationType::CHANGE_TRUST as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::ALLOW_TRUST(ref val) => {
                (OperationType::ALLOW_TRUST as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::ACCOUNT_MERGE(ref val) => {
                (OperationType::ACCOUNT_MERGE as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::INFLATION => (OperationType::INFLATION as i32).pack(out)?,
            &OperationBody::MANAGE_DATA(ref val) => {
                (OperationType::MANAGE_DATA as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::BUMP_SEQUENCE(ref val) => {
                (OperationType::BUMP_SEQUENCE as i32).pack(out)? + val.pack(out)?
            }
            &OperationBody::MANAGE_BUY_OFFER(ref val) => {
                (OperationType::MANAGE_BUY_OFFER as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationMeta {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.changes.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &OperationResult::opINNER(ref val) => {
                (OperationResultCode::opINNER as i32).pack(out)? + val.pack(out)?
            }
            &OperationResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationResultTr {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &OperationResultTr::CREATE_ACCOUNT(ref val) => {
                (OperationType::CREATE_ACCOUNT as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::PAYMENT(ref val) => {
                (OperationType::PAYMENT as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::PATH_PAYMENT(ref val) => {
                (OperationType::PATH_PAYMENT as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::MANAGE_SELL_OFFER(ref val) => {
                (OperationType::MANAGE_SELL_OFFER as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::CREATE_PASSIVE_SELL_OFFER(ref val) => {
                (OperationType::CREATE_PASSIVE_SELL_OFFER as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::SET_OPTIONS(ref val) => {
                (OperationType::SET_OPTIONS as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::CHANGE_TRUST(ref val) => {
                (OperationType::CHANGE_TRUST as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::ALLOW_TRUST(ref val) => {
                (OperationType::ALLOW_TRUST as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::ACCOUNT_MERGE(ref val) => {
                (OperationType::ACCOUNT_MERGE as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::INFLATION(ref val) => {
                (OperationType::INFLATION as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::MANAGE_DATA(ref val) => {
                (OperationType::MANAGE_DATA as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::BUMP_SEQUENCE(ref val) => {
                (OperationType::BUMP_SEQUENCE as i32).pack(out)? + val.pack(out)?
            }
            &OperationResultTr::MANAGE_BUY_OFFER(ref val) => {
                (OperationType::MANAGE_BUY_OFFER as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for OperationType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PathPaymentOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sendAsset.pack(out)?
            + self.sendMax.pack(out)?
            + self.destination.pack(out)?
            + self.destAsset.pack(out)?
            + self.destAmount.pack(out)?
            + xdr_codec::pack_flex(&self.path, Some(5i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PathPaymentResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &PathPaymentResult::PATH_PAYMENT_SUCCESS(ref val) => {
                (PathPaymentResultCode::PATH_PAYMENT_SUCCESS as i32).pack(out)? + val.pack(out)?
            }
            &PathPaymentResult::PATH_PAYMENT_NO_ISSUER(ref val) => {
                (PathPaymentResultCode::PATH_PAYMENT_NO_ISSUER as i32).pack(out)? + val.pack(out)?
            }
            &PathPaymentResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PathPaymentResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PathPaymentResultSuccess {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.offers, None, out)? + self.last.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PaymentOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.destination.pack(out)? + self.asset.pack(out)? + self.amount.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PaymentResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &PaymentResult::PAYMENT_SUCCESS => {
                (PaymentResultCode::PAYMENT_SUCCESS as i32).pack(out)?
            }
            &PaymentResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PaymentResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PeerAddress {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ip.pack(out)? + self.port.pack(out)? + self.numFailures.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PeerAddressIp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &PeerAddressIp::IPv4(ref val) => {
                (IPAddrType::IPv4 as i32).pack(out)?
                    + xdr_codec::pack_opaque_array(&val[..], val.len(), out)?
            }
            &PeerAddressIp::IPv6(ref val) => {
                (IPAddrType::IPv6 as i32).pack(out)?
                    + xdr_codec::pack_opaque_array(&val[..], val.len(), out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Price {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.n.pack(out)? + self.d.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PublicKey {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &PublicKey::PUBLIC_KEY_TYPE_ED25519(ref val) => {
                (PublicKeyType::PUBLIC_KEY_TYPE_ED25519 as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for PublicKeyType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPBallot {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.counter.pack(out)? + self.value.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPEnvelope {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.statement.pack(out)? + self.signature.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPHistoryEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SCPHistoryEntry::Const0(ref val) => (0i64 as i32).pack(out)? + val.pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPHistoryEntryV0 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.quorumSets, None, out)? + self.ledgerMessages.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPNomination {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.quorumSetHash.pack(out)?
            + xdr_codec::pack_flex(&self.votes, None, out)?
            + xdr_codec::pack_flex(&self.accepted, None, out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPQuorumSet {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.threshold.pack(out)?
            + xdr_codec::pack_flex(&self.validators, None, out)?
            + xdr_codec::pack_flex(&self.innerSets, None, out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatement {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.nodeID.pack(out)? + self.slotIndex.pack(out)? + self.pledges.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementPledges {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SCPStatementPledges::SCP_ST_PREPARE(ref val) => {
                (SCPStatementType::SCP_ST_PREPARE as i32).pack(out)? + val.pack(out)?
            }
            &SCPStatementPledges::SCP_ST_CONFIRM(ref val) => {
                (SCPStatementType::SCP_ST_CONFIRM as i32).pack(out)? + val.pack(out)?
            }
            &SCPStatementPledges::SCP_ST_EXTERNALIZE(ref val) => {
                (SCPStatementType::SCP_ST_EXTERNALIZE as i32).pack(out)? + val.pack(out)?
            }
            &SCPStatementPledges::SCP_ST_NOMINATE(ref val) => {
                (SCPStatementType::SCP_ST_NOMINATE as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementPledgesConfirm {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ballot.pack(out)?
            + self.nPrepared.pack(out)?
            + self.nCommit.pack(out)?
            + self.nH.pack(out)?
            + self.quorumSetHash.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementPledgesExternalize {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.commit.pack(out)? + self.nH.pack(out)? + self.commitQuorumSetHash.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementPledgesPrepare {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.quorumSetHash.pack(out)?
            + self.ballot.pack(out)?
            + self.prepared.pack(out)?
            + self.preparedPrime.pack(out)?
            + self.nC.pack(out)?
            + self.nH.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SCPStatementType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SetOptionsOp {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.inflationDest.pack(out)?
            + self.clearFlags.pack(out)?
            + self.setFlags.pack(out)?
            + self.masterWeight.pack(out)?
            + self.lowThreshold.pack(out)?
            + self.medThreshold.pack(out)?
            + self.highThreshold.pack(out)?
            + self.homeDomain.pack(out)?
            + self.signer.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SetOptionsResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SetOptionsResult::SET_OPTIONS_SUCCESS => {
                (SetOptionsResultCode::SET_OPTIONS_SUCCESS as i32).pack(out)?
            }
            &SetOptionsResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SetOptionsResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Signature {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(
            &self.0,
            Some(64i64 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SignatureHint {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Signer {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.key.pack(out)? + self.weight.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SignerKey {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &SignerKey::SIGNER_KEY_TYPE_ED25519(ref val) => {
                (SignerKeyType::SIGNER_KEY_TYPE_ED25519 as i32).pack(out)? + val.pack(out)?
            }
            &SignerKey::SIGNER_KEY_TYPE_PRE_AUTH_TX(ref val) => {
                (SignerKeyType::SIGNER_KEY_TYPE_PRE_AUTH_TX as i32).pack(out)? + val.pack(out)?
            }
            &SignerKey::SIGNER_KEY_TYPE_HASH_X(ref val) => {
                (SignerKeyType::SIGNER_KEY_TYPE_HASH_X as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SignerKeyType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for SimplePaymentResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.destination.pack(out)? + self.asset.pack(out)? + self.amount.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for StellarMessage {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &StellarMessage::ERROR_MSG(ref val) => {
                (MessageType::ERROR_MSG as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::HELLO(ref val) => {
                (MessageType::HELLO as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::AUTH(ref val) => {
                (MessageType::AUTH as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::DONT_HAVE(ref val) => {
                (MessageType::DONT_HAVE as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::GET_PEERS => (MessageType::GET_PEERS as i32).pack(out)?,
            &StellarMessage::PEERS(ref val) => {
                (MessageType::PEERS as i32).pack(out)?
                    + xdr_codec::pack_flex(&val, Some(100i64 as usize), out)?
            }
            &StellarMessage::GET_TX_SET(ref val) => {
                (MessageType::GET_TX_SET as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::TX_SET(ref val) => {
                (MessageType::TX_SET as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::TRANSACTION(ref val) => {
                (MessageType::TRANSACTION as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::GET_SCP_QUORUMSET(ref val) => {
                (MessageType::GET_SCP_QUORUMSET as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::SCP_QUORUMSET(ref val) => {
                (MessageType::SCP_QUORUMSET as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::SCP_MESSAGE(ref val) => {
                (MessageType::SCP_MESSAGE as i32).pack(out)? + val.pack(out)?
            }
            &StellarMessage::GET_SCP_STATE(ref val) => {
                (MessageType::GET_SCP_STATE as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for StellarValue {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.txSetHash.pack(out)?
            + self.closeTime.pack(out)?
            + xdr_codec::pack_flex(&self.upgrades, Some(6i64 as usize), out)?
            + self.ext.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for StellarValueExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &StellarValueExt::STELLAR_VALUE_BASIC => {
                (StellarValueType::STELLAR_VALUE_BASIC as i32).pack(out)?
            }
            &StellarValueExt::STELLAR_VALUE_SIGNED(ref val) => {
                (StellarValueType::STELLAR_VALUE_SIGNED as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for StellarValueType {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for ThresholdIndexes {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Thresholds {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TimeBounds {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.minTime.pack(out)? + self.maxTime.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Transaction {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.sourceAccount.pack(out)?
            + self.fee.pack(out)?
            + self.seqNum.pack(out)?
            + self.timeBounds.pack(out)?
            + self.memo.pack(out)?
            + xdr_codec::pack_flex(&self.operations, Some(MAX_OPS_PER_TX as usize), out)?
            + self.ext.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionEnvelope {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.tx.pack(out)?
            + xdr_codec::pack_flex(&self.signatures, Some(20i64 as usize), out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TransactionExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionHistoryEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerSeq.pack(out)? + self.txSet.pack(out)? + self.ext.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionHistoryEntryExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TransactionHistoryEntryExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionHistoryResultEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.ledgerSeq.pack(out)? + self.txResultSet.pack(out)? + self.ext.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionHistoryResultEntryExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TransactionHistoryResultEntryExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionMeta {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TransactionMeta::Const0(ref val) => {
                (0i64 as i32).pack(out)? + xdr_codec::pack_flex(&val, None, out)?
            }
            &TransactionMeta::Const1(ref val) => (1i64 as i32).pack(out)? + val.pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionMetaV1 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.txChanges.pack(out)? + xdr_codec::pack_flex(&self.operations, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.feeCharged.pack(out)? + self.result.pack(out)? + self.ext.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResultCode {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResultExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TransactionResultExt::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResultPair {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.transactionHash.pack(out)? + self.result.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResultResult {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TransactionResultResult::txSUCCESS(ref val) => {
                (TransactionResultCode::txSUCCESS as i32).pack(out)?
                    + xdr_codec::pack_flex(&val, None, out)?
            }
            &TransactionResultResult::txFAILED(ref val) => {
                (TransactionResultCode::txFAILED as i32).pack(out)?
                    + xdr_codec::pack_flex(&val, None, out)?
            }
            &TransactionResultResult::default => return Err(xdr_codec::Error::invalidcase(-1)),
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionResultSet {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_flex(&self.results, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionSet {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.previousLedgerHash.pack(out)? + xdr_codec::pack_flex(&self.txs, None, out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionSignaturePayload {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.networkId.pack(out)? + self.taggedTransaction.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TransactionSignaturePayloadTaggedTransaction {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TransactionSignaturePayloadTaggedTransaction::ENVELOPE_TYPE_TX(ref val) => {
                (EnvelopeType::ENVELOPE_TYPE_TX as i32).pack(out)? + val.pack(out)?
            }
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TrustLineEntry {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.accountID.pack(out)?
            + self.asset.pack(out)?
            + self.balance.pack(out)?
            + self.limit.pack(out)?
            + self.flags.pack(out)?
            + self.ext.pack(out)?
            + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TrustLineEntryExt {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TrustLineEntryExt::Const0 => (0i64 as i32).pack(out)?,
            &TrustLineEntryExt::Const1(ref val) => (1i64 as i32).pack(out)? + val.pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TrustLineEntryExtV1 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(self.liabilities.pack(out)? + self.ext.pack(out)? + 0)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TrustLineEntryExtV1Ext {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(match self {
            &TrustLineEntryExtV1Ext::Const0 => (0i64 as i32).pack(out)?,
        })
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for TrustLineFlags {
    #[inline]
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok((*self as i32).pack(out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for UpgradeType {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(
            &self.0,
            Some(128i64 as usize),
            out,
        )?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for Value {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_flex(&self.0, None, out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for string32 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(&self.0, Some(32i64 as usize), out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for string64 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_string(&self.0, Some(64i64 as usize), out)?)
    }
}

impl<Out: xdr_codec::Write> xdr_codec::Pack<Out> for uint256 {
    fn pack(&self, out: &mut Out) -> xdr_codec::Result<usize> {
        Ok(xdr_codec::pack_opaque_array(
            &self.0[..],
            self.0.len(),
            out,
        )?)
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountEntry, usize)> {
        let mut sz = 0;
        Ok((
            AccountEntry {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                balance: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                seqNum: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                numSubEntries: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                inflationDest: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                flags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                homeDomain: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                thresholds: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signers: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(20i64 as usize))?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountEntryExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountEntryExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => AccountEntryExt::Const0,
                x if x == (1i32 as i32) => AccountEntryExt::Const1({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountEntryExtV1 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountEntryExtV1, usize)> {
        let mut sz = 0;
        Ok((
            AccountEntryExtV1 {
                liabilities: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountEntryExtV1Ext {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountEntryExtV1Ext, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => AccountEntryExtV1Ext::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountFlags {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountFlags, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AccountFlags::AUTH_REQUIRED_FLAG as i32 => {
                        AccountFlags::AUTH_REQUIRED_FLAG
                    }
                    x if x == AccountFlags::AUTH_REVOCABLE_FLAG as i32 => {
                        AccountFlags::AUTH_REVOCABLE_FLAG
                    }
                    x if x == AccountFlags::AUTH_IMMUTABLE_FLAG as i32 => {
                        AccountFlags::AUTH_IMMUTABLE_FLAG
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountMergeResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountMergeResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => AccountMergeResult::ACCOUNT_MERGE_SUCCESS({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => AccountMergeResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AccountMergeResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AccountMergeResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_SUCCESS as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_SUCCESS
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_MALFORMED as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_MALFORMED
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_NO_ACCOUNT as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_NO_ACCOUNT
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_IMMUTABLE_SET as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_IMMUTABLE_SET
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_HAS_SUB_ENTRIES as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_HAS_SUB_ENTRIES
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_SEQNUM_TOO_FAR as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_SEQNUM_TOO_FAR
                    }
                    x if x == AccountMergeResultCode::ACCOUNT_MERGE_DEST_FULL as i32 => {
                        AccountMergeResultCode::ACCOUNT_MERGE_DEST_FULL
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AllowTrustOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AllowTrustOp, usize)> {
        let mut sz = 0;
        Ok((
            AllowTrustOp {
                trustor: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                asset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                authorize: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AllowTrustOpAsset {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AllowTrustOpAsset, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => AllowTrustOpAsset::ASSET_TYPE_CREDIT_ALPHANUM4({
                    let (v, fsz) = {
                        let mut buf: [u8; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 4i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => AllowTrustOpAsset::ASSET_TYPE_CREDIT_ALPHANUM12({
                    let (v, fsz) = {
                        let mut buf: [u8; 12i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 12i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AllowTrustResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AllowTrustResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => AllowTrustResult::ALLOW_TRUST_SUCCESS,
                _ => AllowTrustResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AllowTrustResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AllowTrustResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AllowTrustResultCode::ALLOW_TRUST_SUCCESS as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_SUCCESS
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_MALFORMED as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_MALFORMED
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_NO_TRUST_LINE as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_NO_TRUST_LINE
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_TRUST_NOT_REQUIRED as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_TRUST_NOT_REQUIRED
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_CANT_REVOKE as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_CANT_REVOKE
                    }
                    x if x == AllowTrustResultCode::ALLOW_TRUST_SELF_NOT_ALLOWED as i32 => {
                        AllowTrustResultCode::ALLOW_TRUST_SELF_NOT_ALLOWED
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Asset {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Asset, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => Asset::ASSET_TYPE_NATIVE,
                x if x == (1i32 as i32) => Asset::ASSET_TYPE_CREDIT_ALPHANUM4({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => Asset::ASSET_TYPE_CREDIT_ALPHANUM12({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AssetAlphanum12 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AssetAlphanum12, usize)> {
        let mut sz = 0;
        Ok((
            AssetAlphanum12 {
                assetCode: {
                    let (v, fsz) = {
                        let mut buf: [u8; 12i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 12i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
                issuer: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AssetAlphanum4 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AssetAlphanum4, usize)> {
        let mut sz = 0;
        Ok((
            AssetAlphanum4 {
                assetCode: {
                    let (v, fsz) = {
                        let mut buf: [u8; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 4i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
                issuer: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AssetType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(AssetType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == AssetType::ASSET_TYPE_NATIVE as i32 => AssetType::ASSET_TYPE_NATIVE,
                    x if x == AssetType::ASSET_TYPE_CREDIT_ALPHANUM4 as i32 => {
                        AssetType::ASSET_TYPE_CREDIT_ALPHANUM4
                    }
                    x if x == AssetType::ASSET_TYPE_CREDIT_ALPHANUM12 as i32 => {
                        AssetType::ASSET_TYPE_CREDIT_ALPHANUM12
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Auth {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Auth, usize)> {
        let mut sz = 0;
        Ok((
            Auth {
                unused: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AuthCert {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AuthCert, usize)> {
        let mut sz = 0;
        Ok((
            AuthCert {
                pubkey: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                expiration: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                sig: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AuthenticatedMessage {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AuthenticatedMessage, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => AuthenticatedMessage::Const0({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for AuthenticatedMessageV0 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(AuthenticatedMessageV0, usize)> {
        let mut sz = 0;
        Ok((
            AuthenticatedMessageV0 {
                sequence: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                message: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                mac: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BucketEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(BucketEntry, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => BucketEntry::LIVEENTRY({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => BucketEntry::INITENTRY({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => BucketEntry::DEADENTRY({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (-1i32 as i32) => BucketEntry::METAENTRY({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BucketEntryType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(BucketEntryType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == BucketEntryType::METAENTRY as i32 => BucketEntryType::METAENTRY,
                    x if x == BucketEntryType::LIVEENTRY as i32 => BucketEntryType::LIVEENTRY,
                    x if x == BucketEntryType::DEADENTRY as i32 => BucketEntryType::DEADENTRY,
                    x if x == BucketEntryType::INITENTRY as i32 => BucketEntryType::INITENTRY,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BucketMetadata {
    fn unpack(input: &mut In) -> xdr_codec::Result<(BucketMetadata, usize)> {
        let mut sz = 0;
        Ok((
            BucketMetadata {
                ledgerVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BucketMetadataExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(BucketMetadataExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => BucketMetadataExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BumpSequenceOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(BumpSequenceOp, usize)> {
        let mut sz = 0;
        Ok((
            BumpSequenceOp {
                bumpTo: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BumpSequenceResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(BumpSequenceResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => BumpSequenceResult::BUMP_SEQUENCE_SUCCESS,
                _ => BumpSequenceResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for BumpSequenceResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(BumpSequenceResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == BumpSequenceResultCode::BUMP_SEQUENCE_SUCCESS as i32 => {
                        BumpSequenceResultCode::BUMP_SEQUENCE_SUCCESS
                    }
                    x if x == BumpSequenceResultCode::BUMP_SEQUENCE_BAD_SEQ as i32 => {
                        BumpSequenceResultCode::BUMP_SEQUENCE_BAD_SEQ
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ChangeTrustOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ChangeTrustOp, usize)> {
        let mut sz = 0;
        Ok((
            ChangeTrustOp {
                line: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                limit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ChangeTrustResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ChangeTrustResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ChangeTrustResult::CHANGE_TRUST_SUCCESS,
                _ => ChangeTrustResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ChangeTrustResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ChangeTrustResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_SUCCESS as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_SUCCESS
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_MALFORMED as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_MALFORMED
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_NO_ISSUER as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_NO_ISSUER
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_INVALID_LIMIT as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_INVALID_LIMIT
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_LOW_RESERVE as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_LOW_RESERVE
                    }
                    x if x == ChangeTrustResultCode::CHANGE_TRUST_SELF_NOT_ALLOWED as i32 => {
                        ChangeTrustResultCode::CHANGE_TRUST_SELF_NOT_ALLOWED
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ClaimOfferAtom {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ClaimOfferAtom, usize)> {
        let mut sz = 0;
        Ok((
            ClaimOfferAtom {
                sellerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                assetSold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amountSold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                assetBought: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amountBought: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CreateAccountOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CreateAccountOp, usize)> {
        let mut sz = 0;
        Ok((
            CreateAccountOp {
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                startingBalance: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CreateAccountResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CreateAccountResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => CreateAccountResult::CREATE_ACCOUNT_SUCCESS,
                _ => CreateAccountResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CreateAccountResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(CreateAccountResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_SUCCESS as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_SUCCESS
                    }
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_MALFORMED as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_MALFORMED
                    }
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_UNDERFUNDED as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_UNDERFUNDED
                    }
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_LOW_RESERVE as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_LOW_RESERVE
                    }
                    x if x == CreateAccountResultCode::CREATE_ACCOUNT_ALREADY_EXIST as i32 => {
                        CreateAccountResultCode::CREATE_ACCOUNT_ALREADY_EXIST
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CreatePassiveSellOfferOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(CreatePassiveSellOfferOp, usize)> {
        let mut sz = 0;
        Ok((
            CreatePassiveSellOfferOp {
                selling: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                buying: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                price: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for CryptoKeyType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(CryptoKeyType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == CryptoKeyType::KEY_TYPE_ED25519 as i32 => {
                        CryptoKeyType::KEY_TYPE_ED25519
                    }
                    x if x == CryptoKeyType::KEY_TYPE_PRE_AUTH_TX as i32 => {
                        CryptoKeyType::KEY_TYPE_PRE_AUTH_TX
                    }
                    x if x == CryptoKeyType::KEY_TYPE_HASH_X as i32 => {
                        CryptoKeyType::KEY_TYPE_HASH_X
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Curve25519Public {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Curve25519Public, usize)> {
        let mut sz = 0;
        Ok((
            Curve25519Public {
                key: {
                    let (v, fsz) = {
                        let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Curve25519Secret {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Curve25519Secret, usize)> {
        let mut sz = 0;
        Ok((
            Curve25519Secret {
                key: {
                    let (v, fsz) = {
                        let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DataEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DataEntry, usize)> {
        let mut sz = 0;
        Ok((
            DataEntry {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dataName: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dataValue: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DataEntryExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DataEntryExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => DataEntryExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DataValue {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DataValue, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, Some(64i64 as usize))?;
                sz = usz;
                DataValue(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DecoratedSignature {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DecoratedSignature, usize)> {
        let mut sz = 0;
        Ok((
            DecoratedSignature {
                hint: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signature: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for DontHave {
    fn unpack(input: &mut In) -> xdr_codec::Result<(DontHave, usize)> {
        let mut sz = 0;
        Ok((
            DontHave {
                type_: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                reqHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for EnvelopeType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(EnvelopeType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == EnvelopeType::ENVELOPE_TYPE_SCP as i32 => {
                        EnvelopeType::ENVELOPE_TYPE_SCP
                    }
                    x if x == EnvelopeType::ENVELOPE_TYPE_TX as i32 => {
                        EnvelopeType::ENVELOPE_TYPE_TX
                    }
                    x if x == EnvelopeType::ENVELOPE_TYPE_AUTH as i32 => {
                        EnvelopeType::ENVELOPE_TYPE_AUTH
                    }
                    x if x == EnvelopeType::ENVELOPE_TYPE_SCPVALUE as i32 => {
                        EnvelopeType::ENVELOPE_TYPE_SCPVALUE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Error {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Error, usize)> {
        let mut sz = 0;
        Ok((
            Error {
                code: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                msg: {
                    let (v, fsz) = xdr_codec::unpack_string(input, Some(100i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ErrorCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ErrorCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ErrorCode::ERR_MISC as i32 => ErrorCode::ERR_MISC,
                    x if x == ErrorCode::ERR_DATA as i32 => ErrorCode::ERR_DATA,
                    x if x == ErrorCode::ERR_CONF as i32 => ErrorCode::ERR_CONF,
                    x if x == ErrorCode::ERR_AUTH as i32 => ErrorCode::ERR_AUTH,
                    x if x == ErrorCode::ERR_LOAD as i32 => ErrorCode::ERR_LOAD,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Hash {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Hash, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                Hash(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Hello {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Hello, usize)> {
        let mut sz = 0;
        Ok((
            Hello {
                ledgerVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                overlayVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                overlayMinVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                networkID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                versionStr: {
                    let (v, fsz) = xdr_codec::unpack_string(input, Some(100i64 as usize))?;
                    sz += fsz;
                    v
                },
                listeningPort: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                peerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                cert: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nonce: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for HmacSha256Key {
    fn unpack(input: &mut In) -> xdr_codec::Result<(HmacSha256Key, usize)> {
        let mut sz = 0;
        Ok((
            HmacSha256Key {
                key: {
                    let (v, fsz) = {
                        let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for HmacSha256Mac {
    fn unpack(input: &mut In) -> xdr_codec::Result<(HmacSha256Mac, usize)> {
        let mut sz = 0;
        Ok((
            HmacSha256Mac {
                mac: {
                    let (v, fsz) = {
                        let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for IPAddrType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(IPAddrType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == IPAddrType::IPv4 as i32 => IPAddrType::IPv4,
                    x if x == IPAddrType::IPv6 as i32 => IPAddrType::IPv6,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for InflationPayout {
    fn unpack(input: &mut In) -> xdr_codec::Result<(InflationPayout, usize)> {
        let mut sz = 0;
        Ok((
            InflationPayout {
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for InflationResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(InflationResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => InflationResult::INFLATION_SUCCESS({
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                }),
                _ => InflationResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for InflationResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(InflationResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == InflationResultCode::INFLATION_SUCCESS as i32 => {
                        InflationResultCode::INFLATION_SUCCESS
                    }
                    x if x == InflationResultCode::INFLATION_NOT_TIME as i32 => {
                        InflationResultCode::INFLATION_NOT_TIME
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerCloseValueSignature {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerCloseValueSignature, usize)> {
        let mut sz = 0;
        Ok((
            LedgerCloseValueSignature {
                nodeID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signature: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntry, usize)> {
        let mut sz = 0;
        Ok((
            LedgerEntry {
                lastModifiedLedgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                data: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryChange {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryChange, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerEntryChange::LEDGER_ENTRY_CREATED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => LedgerEntryChange::LEDGER_ENTRY_UPDATED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => LedgerEntryChange::LEDGER_ENTRY_REMOVED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => LedgerEntryChange::LEDGER_ENTRY_STATE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryChangeType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryChangeType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == LedgerEntryChangeType::LEDGER_ENTRY_CREATED as i32 => {
                        LedgerEntryChangeType::LEDGER_ENTRY_CREATED
                    }
                    x if x == LedgerEntryChangeType::LEDGER_ENTRY_UPDATED as i32 => {
                        LedgerEntryChangeType::LEDGER_ENTRY_UPDATED
                    }
                    x if x == LedgerEntryChangeType::LEDGER_ENTRY_REMOVED as i32 => {
                        LedgerEntryChangeType::LEDGER_ENTRY_REMOVED
                    }
                    x if x == LedgerEntryChangeType::LEDGER_ENTRY_STATE as i32 => {
                        LedgerEntryChangeType::LEDGER_ENTRY_STATE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryChanges {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryChanges, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_flex(input, None)?;
                sz = usz;
                LedgerEntryChanges(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryData {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryData, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerEntryData::ACCOUNT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => LedgerEntryData::TRUSTLINE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => LedgerEntryData::OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => LedgerEntryData::DATA({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerEntryExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerEntryType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerEntryType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == LedgerEntryType::ACCOUNT as i32 => LedgerEntryType::ACCOUNT,
                    x if x == LedgerEntryType::TRUSTLINE as i32 => LedgerEntryType::TRUSTLINE,
                    x if x == LedgerEntryType::OFFER as i32 => LedgerEntryType::OFFER,
                    x if x == LedgerEntryType::DATA as i32 => LedgerEntryType::DATA,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerHeader {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerHeader, usize)> {
        let mut sz = 0;
        Ok((
            LedgerHeader {
                ledgerVersion: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                previousLedgerHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                scpValue: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                txSetResultHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                bucketListHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ledgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                totalCoins: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                feePool: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                inflationSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                idPool: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                baseFee: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                baseReserve: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                maxTxSetSize: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                skipList: {
                    let (v, fsz) = {
                        #[inline]
                        fn uninit_ptr_setter<T>(p: &mut T, v: T) {
                            unsafe { ::std::ptr::write(p, v) }
                        }
                        #[inline]
                        fn uninit_ptr_dropper<T>(p: &mut T) {
                            unsafe { ::std::ptr::drop_in_place(p) }
                        }
                        let mut buf: [Hash; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let res =
                            ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                                xdr_codec::unpack_array_with(
                                    input,
                                    &mut buf[..],
                                    4i64 as usize,
                                    uninit_ptr_setter,
                                    uninit_ptr_dropper,
                                    None,
                                )
                            }));
                        let sz = match res {
                            Ok(Ok(sz)) => sz,
                            Ok(Err(err)) => {
                                ::std::mem::forget(buf);
                                return Err(err);
                            }
                            Err(panic) => {
                                ::std::mem::forget(buf);
                                ::std::panic::resume_unwind(panic);
                            }
                        };
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerHeaderExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerHeaderExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerHeaderExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerHeaderHistoryEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerHeaderHistoryEntry, usize)> {
        let mut sz = 0;
        Ok((
            LedgerHeaderHistoryEntry {
                hash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                header: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerHeaderHistoryEntryExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerHeaderHistoryEntryExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerHeaderHistoryEntryExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKey {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKey, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => LedgerKey::ACCOUNT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => LedgerKey::TRUSTLINE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => LedgerKey::OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => LedgerKey::DATA({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKeyAccount {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKeyAccount, usize)> {
        let mut sz = 0;
        Ok((
            LedgerKeyAccount {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKeyData {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKeyData, usize)> {
        let mut sz = 0;
        Ok((
            LedgerKeyData {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dataName: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKeyOffer {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKeyOffer, usize)> {
        let mut sz = 0;
        Ok((
            LedgerKeyOffer {
                sellerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerKeyTrustline {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerKeyTrustline, usize)> {
        let mut sz = 0;
        Ok((
            LedgerKeyTrustline {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                asset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerSCPMessages {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerSCPMessages, usize)> {
        let mut sz = 0;
        Ok((
            LedgerSCPMessages {
                ledgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                messages: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerUpgrade {
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerUpgrade, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (1i32 as i32) => LedgerUpgrade::LEDGER_UPGRADE_VERSION({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => LedgerUpgrade::LEDGER_UPGRADE_BASE_FEE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => LedgerUpgrade::LEDGER_UPGRADE_MAX_TX_SET_SIZE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (4i32 as i32) => LedgerUpgrade::LEDGER_UPGRADE_BASE_RESERVE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for LedgerUpgradeType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(LedgerUpgradeType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == LedgerUpgradeType::LEDGER_UPGRADE_VERSION as i32 => {
                        LedgerUpgradeType::LEDGER_UPGRADE_VERSION
                    }
                    x if x == LedgerUpgradeType::LEDGER_UPGRADE_BASE_FEE as i32 => {
                        LedgerUpgradeType::LEDGER_UPGRADE_BASE_FEE
                    }
                    x if x == LedgerUpgradeType::LEDGER_UPGRADE_MAX_TX_SET_SIZE as i32 => {
                        LedgerUpgradeType::LEDGER_UPGRADE_MAX_TX_SET_SIZE
                    }
                    x if x == LedgerUpgradeType::LEDGER_UPGRADE_BASE_RESERVE as i32 => {
                        LedgerUpgradeType::LEDGER_UPGRADE_BASE_RESERVE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Liabilities {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Liabilities, usize)> {
        let mut sz = 0;
        Ok((
            Liabilities {
                buying: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                selling: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageBuyOfferOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageBuyOfferOp, usize)> {
        let mut sz = 0;
        Ok((
            ManageBuyOfferOp {
                selling: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                buying: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                buyAmount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                price: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageBuyOfferResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageBuyOfferResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ManageBuyOfferResult::MANAGE_BUY_OFFER_SUCCESS({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => ManageBuyOfferResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageBuyOfferResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageBuyOfferResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SUCCESS as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SUCCESS
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_MALFORMED as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_MALFORMED
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SELL_NO_TRUST as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SELL_NO_TRUST
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_BUY_NO_TRUST as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_BUY_NO_TRUST
                    }
                    x if x
                        == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED
                            as i32 =>
                    {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED
                    }
                    x if x
                        == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED as i32 =>
                    {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_LINE_FULL as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_LINE_FULL
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_UNDERFUNDED as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_UNDERFUNDED
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_CROSS_SELF as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_CROSS_SELF
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SELL_NO_ISSUER as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_SELL_NO_ISSUER
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_BUY_NO_ISSUER as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_BUY_NO_ISSUER
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_NOT_FOUND as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_NOT_FOUND
                    }
                    x if x == ManageBuyOfferResultCode::MANAGE_BUY_OFFER_LOW_RESERVE as i32 => {
                        ManageBuyOfferResultCode::MANAGE_BUY_OFFER_LOW_RESERVE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageDataOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageDataOp, usize)> {
        let mut sz = 0;
        Ok((
            ManageDataOp {
                dataName: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                dataValue: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageDataResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageDataResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ManageDataResult::MANAGE_DATA_SUCCESS,
                _ => ManageDataResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageDataResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageDataResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ManageDataResultCode::MANAGE_DATA_SUCCESS as i32 => {
                        ManageDataResultCode::MANAGE_DATA_SUCCESS
                    }
                    x if x == ManageDataResultCode::MANAGE_DATA_NOT_SUPPORTED_YET as i32 => {
                        ManageDataResultCode::MANAGE_DATA_NOT_SUPPORTED_YET
                    }
                    x if x == ManageDataResultCode::MANAGE_DATA_NAME_NOT_FOUND as i32 => {
                        ManageDataResultCode::MANAGE_DATA_NAME_NOT_FOUND
                    }
                    x if x == ManageDataResultCode::MANAGE_DATA_LOW_RESERVE as i32 => {
                        ManageDataResultCode::MANAGE_DATA_LOW_RESERVE
                    }
                    x if x == ManageDataResultCode::MANAGE_DATA_INVALID_NAME as i32 => {
                        ManageDataResultCode::MANAGE_DATA_INVALID_NAME
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageOfferEffect {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageOfferEffect, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ManageOfferEffect::MANAGE_OFFER_CREATED as i32 => {
                        ManageOfferEffect::MANAGE_OFFER_CREATED
                    }
                    x if x == ManageOfferEffect::MANAGE_OFFER_UPDATED as i32 => {
                        ManageOfferEffect::MANAGE_OFFER_UPDATED
                    }
                    x if x == ManageOfferEffect::MANAGE_OFFER_DELETED as i32 => {
                        ManageOfferEffect::MANAGE_OFFER_DELETED
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageOfferSuccessResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageOfferSuccessResult, usize)> {
        let mut sz = 0;
        Ok((
            ManageOfferSuccessResult {
                offersClaimed: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
                offer: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageOfferSuccessResultOffer {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageOfferSuccessResultOffer, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ManageOfferSuccessResultOffer::MANAGE_OFFER_CREATED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => ManageOfferSuccessResultOffer::MANAGE_OFFER_UPDATED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => ManageOfferSuccessResultOffer::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageSellOfferOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageSellOfferOp, usize)> {
        let mut sz = 0;
        Ok((
            ManageSellOfferOp {
                selling: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                buying: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                price: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageSellOfferResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageSellOfferResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => ManageSellOfferResult::MANAGE_SELL_OFFER_SUCCESS({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => ManageSellOfferResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ManageSellOfferResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ManageSellOfferResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_SUCCESS as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_SUCCESS
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_MALFORMED as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_MALFORMED
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_SELL_NO_TRUST as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_SELL_NO_TRUST
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_BUY_NO_TRUST as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_BUY_NO_TRUST
                    }
                    x if x
                        == ManageSellOfferResultCode::MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED
                            as i32 =>
                    {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED
                    }
                    x if x
                        == ManageSellOfferResultCode::MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED
                            as i32 =>
                    {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_LINE_FULL as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_LINE_FULL
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_UNDERFUNDED as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_UNDERFUNDED
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_CROSS_SELF as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_CROSS_SELF
                    }
                    x if x
                        == ManageSellOfferResultCode::MANAGE_SELL_OFFER_SELL_NO_ISSUER as i32 =>
                    {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_SELL_NO_ISSUER
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_BUY_NO_ISSUER as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_BUY_NO_ISSUER
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_NOT_FOUND as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_NOT_FOUND
                    }
                    x if x == ManageSellOfferResultCode::MANAGE_SELL_OFFER_LOW_RESERVE as i32 => {
                        ManageSellOfferResultCode::MANAGE_SELL_OFFER_LOW_RESERVE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Memo {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Memo, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => Memo::MEMO_NONE,
                x if x == (1i32 as i32) => Memo::MEMO_TEXT({
                    let (v, fsz) = xdr_codec::unpack_string(input, Some(28i64 as usize))?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => Memo::MEMO_ID({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => Memo::MEMO_HASH({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (4i32 as i32) => Memo::MEMO_RETURN({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MemoType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(MemoType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == MemoType::MEMO_NONE as i32 => MemoType::MEMO_NONE,
                    x if x == MemoType::MEMO_TEXT as i32 => MemoType::MEMO_TEXT,
                    x if x == MemoType::MEMO_ID as i32 => MemoType::MEMO_ID,
                    x if x == MemoType::MEMO_HASH as i32 => MemoType::MEMO_HASH,
                    x if x == MemoType::MEMO_RETURN as i32 => MemoType::MEMO_RETURN,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for MessageType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(MessageType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == MessageType::ERROR_MSG as i32 => MessageType::ERROR_MSG,
                    x if x == MessageType::AUTH as i32 => MessageType::AUTH,
                    x if x == MessageType::DONT_HAVE as i32 => MessageType::DONT_HAVE,
                    x if x == MessageType::GET_PEERS as i32 => MessageType::GET_PEERS,
                    x if x == MessageType::PEERS as i32 => MessageType::PEERS,
                    x if x == MessageType::GET_TX_SET as i32 => MessageType::GET_TX_SET,
                    x if x == MessageType::TX_SET as i32 => MessageType::TX_SET,
                    x if x == MessageType::TRANSACTION as i32 => MessageType::TRANSACTION,
                    x if x == MessageType::GET_SCP_QUORUMSET as i32 => {
                        MessageType::GET_SCP_QUORUMSET
                    }
                    x if x == MessageType::SCP_QUORUMSET as i32 => MessageType::SCP_QUORUMSET,
                    x if x == MessageType::SCP_MESSAGE as i32 => MessageType::SCP_MESSAGE,
                    x if x == MessageType::GET_SCP_STATE as i32 => MessageType::GET_SCP_STATE,
                    x if x == MessageType::HELLO as i32 => MessageType::HELLO,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OfferEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OfferEntry, usize)> {
        let mut sz = 0;
        Ok((
            OfferEntry {
                sellerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                offerID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                selling: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                buying: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                price: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                flags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OfferEntryExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OfferEntryExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => OfferEntryExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OfferEntryFlags {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(OfferEntryFlags, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == OfferEntryFlags::PASSIVE_FLAG as i32 => OfferEntryFlags::PASSIVE_FLAG,
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Operation {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Operation, usize)> {
        let mut sz = 0;
        Ok((
            Operation {
                sourceAccount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                body: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationBody {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationBody, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => OperationBody::CREATE_ACCOUNT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => OperationBody::PAYMENT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => OperationBody::PATH_PAYMENT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => OperationBody::MANAGE_SELL_OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (4i32 as i32) => OperationBody::CREATE_PASSIVE_SELL_OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (5i32 as i32) => OperationBody::SET_OPTIONS({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (6i32 as i32) => OperationBody::CHANGE_TRUST({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (7i32 as i32) => OperationBody::ALLOW_TRUST({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (8i32 as i32) => OperationBody::ACCOUNT_MERGE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (9i32 as i32) => OperationBody::INFLATION,
                x if x == (10i32 as i32) => OperationBody::MANAGE_DATA({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (11i32 as i32) => OperationBody::BUMP_SEQUENCE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (12i32 as i32) => OperationBody::MANAGE_BUY_OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationMeta {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationMeta, usize)> {
        let mut sz = 0;
        Ok((
            OperationMeta {
                changes: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => OperationResult::opINNER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => OperationResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == OperationResultCode::opINNER as i32 => OperationResultCode::opINNER,
                    x if x == OperationResultCode::opBAD_AUTH as i32 => {
                        OperationResultCode::opBAD_AUTH
                    }
                    x if x == OperationResultCode::opNO_ACCOUNT as i32 => {
                        OperationResultCode::opNO_ACCOUNT
                    }
                    x if x == OperationResultCode::opNOT_SUPPORTED as i32 => {
                        OperationResultCode::opNOT_SUPPORTED
                    }
                    x if x == OperationResultCode::opTOO_MANY_SUBENTRIES as i32 => {
                        OperationResultCode::opTOO_MANY_SUBENTRIES
                    }
                    x if x == OperationResultCode::opEXCEEDED_WORK_LIMIT as i32 => {
                        OperationResultCode::opEXCEEDED_WORK_LIMIT
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationResultTr {
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationResultTr, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => OperationResultTr::CREATE_ACCOUNT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => OperationResultTr::PAYMENT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => OperationResultTr::PATH_PAYMENT({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => OperationResultTr::MANAGE_SELL_OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (4i32 as i32) => OperationResultTr::CREATE_PASSIVE_SELL_OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (5i32 as i32) => OperationResultTr::SET_OPTIONS({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (6i32 as i32) => OperationResultTr::CHANGE_TRUST({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (7i32 as i32) => OperationResultTr::ALLOW_TRUST({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (8i32 as i32) => OperationResultTr::ACCOUNT_MERGE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (9i32 as i32) => OperationResultTr::INFLATION({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (10i32 as i32) => OperationResultTr::MANAGE_DATA({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (11i32 as i32) => OperationResultTr::BUMP_SEQUENCE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (12i32 as i32) => OperationResultTr::MANAGE_BUY_OFFER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for OperationType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(OperationType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == OperationType::CREATE_ACCOUNT as i32 => OperationType::CREATE_ACCOUNT,
                    x if x == OperationType::PAYMENT as i32 => OperationType::PAYMENT,
                    x if x == OperationType::PATH_PAYMENT as i32 => OperationType::PATH_PAYMENT,
                    x if x == OperationType::MANAGE_SELL_OFFER as i32 => {
                        OperationType::MANAGE_SELL_OFFER
                    }
                    x if x == OperationType::CREATE_PASSIVE_SELL_OFFER as i32 => {
                        OperationType::CREATE_PASSIVE_SELL_OFFER
                    }
                    x if x == OperationType::SET_OPTIONS as i32 => OperationType::SET_OPTIONS,
                    x if x == OperationType::CHANGE_TRUST as i32 => OperationType::CHANGE_TRUST,
                    x if x == OperationType::ALLOW_TRUST as i32 => OperationType::ALLOW_TRUST,
                    x if x == OperationType::ACCOUNT_MERGE as i32 => OperationType::ACCOUNT_MERGE,
                    x if x == OperationType::INFLATION as i32 => OperationType::INFLATION,
                    x if x == OperationType::MANAGE_DATA as i32 => OperationType::MANAGE_DATA,
                    x if x == OperationType::BUMP_SEQUENCE as i32 => OperationType::BUMP_SEQUENCE,
                    x if x == OperationType::MANAGE_BUY_OFFER as i32 => {
                        OperationType::MANAGE_BUY_OFFER
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PathPaymentOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PathPaymentOp, usize)> {
        let mut sz = 0;
        Ok((
            PathPaymentOp {
                sendAsset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                sendMax: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                destAsset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                destAmount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                path: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(5i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PathPaymentResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PathPaymentResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => PathPaymentResult::PATH_PAYMENT_SUCCESS({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (-9i32 as i32) => PathPaymentResult::PATH_PAYMENT_NO_ISSUER({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                _ => PathPaymentResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PathPaymentResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(PathPaymentResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == PathPaymentResultCode::PATH_PAYMENT_SUCCESS as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_SUCCESS
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_MALFORMED as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_MALFORMED
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_UNDERFUNDED as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_UNDERFUNDED
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_SRC_NO_TRUST as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_SRC_NO_TRUST
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_SRC_NOT_AUTHORIZED as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_SRC_NOT_AUTHORIZED
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_NO_DESTINATION as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_NO_DESTINATION
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_NO_TRUST as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_NO_TRUST
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_NOT_AUTHORIZED as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_NOT_AUTHORIZED
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_LINE_FULL as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_LINE_FULL
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_NO_ISSUER as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_NO_ISSUER
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_TOO_FEW_OFFERS as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_TOO_FEW_OFFERS
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_OFFER_CROSS_SELF as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_OFFER_CROSS_SELF
                    }
                    x if x == PathPaymentResultCode::PATH_PAYMENT_OVER_SENDMAX as i32 => {
                        PathPaymentResultCode::PATH_PAYMENT_OVER_SENDMAX
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PathPaymentResultSuccess {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PathPaymentResultSuccess, usize)> {
        let mut sz = 0;
        Ok((
            PathPaymentResultSuccess {
                offers: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
                last: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PaymentOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PaymentOp, usize)> {
        let mut sz = 0;
        Ok((
            PaymentOp {
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                asset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PaymentResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PaymentResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => PaymentResult::PAYMENT_SUCCESS,
                _ => PaymentResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PaymentResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(PaymentResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == PaymentResultCode::PAYMENT_SUCCESS as i32 => {
                        PaymentResultCode::PAYMENT_SUCCESS
                    }
                    x if x == PaymentResultCode::PAYMENT_MALFORMED as i32 => {
                        PaymentResultCode::PAYMENT_MALFORMED
                    }
                    x if x == PaymentResultCode::PAYMENT_UNDERFUNDED as i32 => {
                        PaymentResultCode::PAYMENT_UNDERFUNDED
                    }
                    x if x == PaymentResultCode::PAYMENT_SRC_NO_TRUST as i32 => {
                        PaymentResultCode::PAYMENT_SRC_NO_TRUST
                    }
                    x if x == PaymentResultCode::PAYMENT_SRC_NOT_AUTHORIZED as i32 => {
                        PaymentResultCode::PAYMENT_SRC_NOT_AUTHORIZED
                    }
                    x if x == PaymentResultCode::PAYMENT_NO_DESTINATION as i32 => {
                        PaymentResultCode::PAYMENT_NO_DESTINATION
                    }
                    x if x == PaymentResultCode::PAYMENT_NO_TRUST as i32 => {
                        PaymentResultCode::PAYMENT_NO_TRUST
                    }
                    x if x == PaymentResultCode::PAYMENT_NOT_AUTHORIZED as i32 => {
                        PaymentResultCode::PAYMENT_NOT_AUTHORIZED
                    }
                    x if x == PaymentResultCode::PAYMENT_LINE_FULL as i32 => {
                        PaymentResultCode::PAYMENT_LINE_FULL
                    }
                    x if x == PaymentResultCode::PAYMENT_NO_ISSUER as i32 => {
                        PaymentResultCode::PAYMENT_NO_ISSUER
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PeerAddress {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PeerAddress, usize)> {
        let mut sz = 0;
        Ok((
            PeerAddress {
                ip: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                port: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                numFailures: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PeerAddressIp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PeerAddressIp, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => PeerAddressIp::IPv4({
                    let (v, fsz) = {
                        let mut buf: [u8; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 4i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => PeerAddressIp::IPv6({
                    let (v, fsz) = {
                        let mut buf: [u8; 16i64 as usize] = unsafe { ::std::mem::uninitialized() };
                        let sz =
                            xdr_codec::unpack_opaque_array(input, &mut buf[..], 16i64 as usize)?;
                        (buf, sz)
                    };
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Price {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Price, usize)> {
        let mut sz = 0;
        Ok((
            Price {
                n: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                d: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PublicKey {
    fn unpack(input: &mut In) -> xdr_codec::Result<(PublicKey, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => PublicKey::PUBLIC_KEY_TYPE_ED25519({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for PublicKeyType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(PublicKeyType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == PublicKeyType::PUBLIC_KEY_TYPE_ED25519 as i32 => {
                        PublicKeyType::PUBLIC_KEY_TYPE_ED25519
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPBallot {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPBallot, usize)> {
        let mut sz = 0;
        Ok((
            SCPBallot {
                counter: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                value: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPEnvelope {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPEnvelope, usize)> {
        let mut sz = 0;
        Ok((
            SCPEnvelope {
                statement: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signature: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPHistoryEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPHistoryEntry, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SCPHistoryEntry::Const0({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPHistoryEntryV0 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPHistoryEntryV0, usize)> {
        let mut sz = 0;
        Ok((
            SCPHistoryEntryV0 {
                quorumSets: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
                ledgerMessages: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPNomination {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPNomination, usize)> {
        let mut sz = 0;
        Ok((
            SCPNomination {
                quorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                votes: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
                accepted: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPQuorumSet {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPQuorumSet, usize)> {
        let mut sz = 0;
        Ok((
            SCPQuorumSet {
                threshold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                validators: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
                innerSets: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatement {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatement, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatement {
                nodeID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                slotIndex: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                pledges: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementPledges {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementPledges, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SCPStatementPledges::SCP_ST_PREPARE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => SCPStatementPledges::SCP_ST_CONFIRM({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => SCPStatementPledges::SCP_ST_EXTERNALIZE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => SCPStatementPledges::SCP_ST_NOMINATE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementPledgesConfirm {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementPledgesConfirm, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatementPledgesConfirm {
                ballot: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nPrepared: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nCommit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nH: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                quorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementPledgesExternalize {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementPledgesExternalize, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatementPledgesExternalize {
                commit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nH: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                commitQuorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementPledgesPrepare {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementPledgesPrepare, usize)> {
        let mut sz = 0;
        Ok((
            SCPStatementPledgesPrepare {
                quorumSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ballot: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                prepared: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                preparedPrime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nC: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                nH: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SCPStatementType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(SCPStatementType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == SCPStatementType::SCP_ST_PREPARE as i32 => {
                        SCPStatementType::SCP_ST_PREPARE
                    }
                    x if x == SCPStatementType::SCP_ST_CONFIRM as i32 => {
                        SCPStatementType::SCP_ST_CONFIRM
                    }
                    x if x == SCPStatementType::SCP_ST_EXTERNALIZE as i32 => {
                        SCPStatementType::SCP_ST_EXTERNALIZE
                    }
                    x if x == SCPStatementType::SCP_ST_NOMINATE as i32 => {
                        SCPStatementType::SCP_ST_NOMINATE
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SetOptionsOp {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SetOptionsOp, usize)> {
        let mut sz = 0;
        Ok((
            SetOptionsOp {
                inflationDest: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                clearFlags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                setFlags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                masterWeight: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                lowThreshold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                medThreshold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                highThreshold: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                homeDomain: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signer: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SetOptionsResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SetOptionsResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SetOptionsResult::SET_OPTIONS_SUCCESS,
                _ => SetOptionsResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SetOptionsResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(SetOptionsResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == SetOptionsResultCode::SET_OPTIONS_SUCCESS as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_SUCCESS
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_LOW_RESERVE as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_LOW_RESERVE
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_TOO_MANY_SIGNERS as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_TOO_MANY_SIGNERS
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_BAD_FLAGS as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_BAD_FLAGS
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_INVALID_INFLATION as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_INVALID_INFLATION
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_CANT_CHANGE as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_CANT_CHANGE
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_UNKNOWN_FLAG as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_UNKNOWN_FLAG
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_THRESHOLD_OUT_OF_RANGE as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_THRESHOLD_OUT_OF_RANGE
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_BAD_SIGNER as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_BAD_SIGNER
                    }
                    x if x == SetOptionsResultCode::SET_OPTIONS_INVALID_HOME_DOMAIN as i32 => {
                        SetOptionsResultCode::SET_OPTIONS_INVALID_HOME_DOMAIN
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Signature {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Signature, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, Some(64i64 as usize))?;
                sz = usz;
                Signature(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SignatureHint {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SignatureHint, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], 4i64 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                SignatureHint(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Signer {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Signer, usize)> {
        let mut sz = 0;
        Ok((
            Signer {
                key: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                weight: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SignerKey {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SignerKey, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => SignerKey::SIGNER_KEY_TYPE_ED25519({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => SignerKey::SIGNER_KEY_TYPE_PRE_AUTH_TX({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => SignerKey::SIGNER_KEY_TYPE_HASH_X({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SignerKeyType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(SignerKeyType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == SignerKeyType::SIGNER_KEY_TYPE_ED25519 as i32 => {
                        SignerKeyType::SIGNER_KEY_TYPE_ED25519
                    }
                    x if x == SignerKeyType::SIGNER_KEY_TYPE_PRE_AUTH_TX as i32 => {
                        SignerKeyType::SIGNER_KEY_TYPE_PRE_AUTH_TX
                    }
                    x if x == SignerKeyType::SIGNER_KEY_TYPE_HASH_X as i32 => {
                        SignerKeyType::SIGNER_KEY_TYPE_HASH_X
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for SimplePaymentResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(SimplePaymentResult, usize)> {
        let mut sz = 0;
        Ok((
            SimplePaymentResult {
                destination: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                asset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                amount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for StellarMessage {
    fn unpack(input: &mut In) -> xdr_codec::Result<(StellarMessage, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => StellarMessage::ERROR_MSG({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (13i32 as i32) => StellarMessage::HELLO({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (2i32 as i32) => StellarMessage::AUTH({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (3i32 as i32) => StellarMessage::DONT_HAVE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (4i32 as i32) => StellarMessage::GET_PEERS,
                x if x == (5i32 as i32) => StellarMessage::PEERS({
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(100i64 as usize))?;
                    sz += fsz;
                    v
                }),
                x if x == (6i32 as i32) => StellarMessage::GET_TX_SET({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (7i32 as i32) => StellarMessage::TX_SET({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (8i32 as i32) => StellarMessage::TRANSACTION({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (9i32 as i32) => StellarMessage::GET_SCP_QUORUMSET({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (10i32 as i32) => StellarMessage::SCP_QUORUMSET({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (11i32 as i32) => StellarMessage::SCP_MESSAGE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                x if x == (12i32 as i32) => StellarMessage::GET_SCP_STATE({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for StellarValue {
    fn unpack(input: &mut In) -> xdr_codec::Result<(StellarValue, usize)> {
        let mut sz = 0;
        Ok((
            StellarValue {
                txSetHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                closeTime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                upgrades: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(6i64 as usize))?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for StellarValueExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(StellarValueExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => StellarValueExt::STELLAR_VALUE_BASIC,
                x if x == (1i32 as i32) => StellarValueExt::STELLAR_VALUE_SIGNED({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for StellarValueType {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(StellarValueType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == StellarValueType::STELLAR_VALUE_BASIC as i32 => {
                        StellarValueType::STELLAR_VALUE_BASIC
                    }
                    x if x == StellarValueType::STELLAR_VALUE_SIGNED as i32 => {
                        StellarValueType::STELLAR_VALUE_SIGNED
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for ThresholdIndexes {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(ThresholdIndexes, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == ThresholdIndexes::THRESHOLD_MASTER_WEIGHT as i32 => {
                        ThresholdIndexes::THRESHOLD_MASTER_WEIGHT
                    }
                    x if x == ThresholdIndexes::THRESHOLD_LOW as i32 => {
                        ThresholdIndexes::THRESHOLD_LOW
                    }
                    x if x == ThresholdIndexes::THRESHOLD_MED as i32 => {
                        ThresholdIndexes::THRESHOLD_MED
                    }
                    x if x == ThresholdIndexes::THRESHOLD_HIGH as i32 => {
                        ThresholdIndexes::THRESHOLD_HIGH
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Thresholds {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Thresholds, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; 4i64 as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], 4i64 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                Thresholds(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TimeBounds {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TimeBounds, usize)> {
        let mut sz = 0;
        Ok((
            TimeBounds {
                minTime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                maxTime: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Transaction {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Transaction, usize)> {
        let mut sz = 0;
        Ok((
            Transaction {
                sourceAccount: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                fee: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                seqNum: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                timeBounds: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                memo: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                operations: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(MAX_OPS_PER_TX as usize))?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionEnvelope {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionEnvelope, usize)> {
        let mut sz = 0;
        Ok((
            TransactionEnvelope {
                tx: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                signatures: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, Some(20i64 as usize))?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => TransactionExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionHistoryEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionHistoryEntry, usize)> {
        let mut sz = 0;
        Ok((
            TransactionHistoryEntry {
                ledgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                txSet: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionHistoryEntryExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionHistoryEntryExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => TransactionHistoryEntryExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionHistoryResultEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionHistoryResultEntry, usize)> {
        let mut sz = 0;
        Ok((
            TransactionHistoryResultEntry {
                ledgerSeq: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                txResultSet: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionHistoryResultEntryExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionHistoryResultEntryExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => TransactionHistoryResultEntryExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionMeta {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionMeta, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => TransactionMeta::Const0({
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                }),
                x if x == (1i32 as i32) => TransactionMeta::Const1({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionMetaV1 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionMetaV1, usize)> {
        let mut sz = 0;
        Ok((
            TransactionMetaV1 {
                txChanges: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                operations: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResult, usize)> {
        let mut sz = 0;
        Ok((
            TransactionResult {
                feeCharged: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                result: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResultCode {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResultCode, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == TransactionResultCode::txSUCCESS as i32 => {
                        TransactionResultCode::txSUCCESS
                    }
                    x if x == TransactionResultCode::txFAILED as i32 => {
                        TransactionResultCode::txFAILED
                    }
                    x if x == TransactionResultCode::txTOO_EARLY as i32 => {
                        TransactionResultCode::txTOO_EARLY
                    }
                    x if x == TransactionResultCode::txTOO_LATE as i32 => {
                        TransactionResultCode::txTOO_LATE
                    }
                    x if x == TransactionResultCode::txMISSING_OPERATION as i32 => {
                        TransactionResultCode::txMISSING_OPERATION
                    }
                    x if x == TransactionResultCode::txBAD_SEQ as i32 => {
                        TransactionResultCode::txBAD_SEQ
                    }
                    x if x == TransactionResultCode::txBAD_AUTH as i32 => {
                        TransactionResultCode::txBAD_AUTH
                    }
                    x if x == TransactionResultCode::txINSUFFICIENT_BALANCE as i32 => {
                        TransactionResultCode::txINSUFFICIENT_BALANCE
                    }
                    x if x == TransactionResultCode::txNO_ACCOUNT as i32 => {
                        TransactionResultCode::txNO_ACCOUNT
                    }
                    x if x == TransactionResultCode::txINSUFFICIENT_FEE as i32 => {
                        TransactionResultCode::txINSUFFICIENT_FEE
                    }
                    x if x == TransactionResultCode::txBAD_AUTH_EXTRA as i32 => {
                        TransactionResultCode::txBAD_AUTH_EXTRA
                    }
                    x if x == TransactionResultCode::txINTERNAL_ERROR as i32 => {
                        TransactionResultCode::txINTERNAL_ERROR
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResultExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResultExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => TransactionResultExt::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResultPair {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResultPair, usize)> {
        let mut sz = 0;
        Ok((
            TransactionResultPair {
                transactionHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                result: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResultResult {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResultResult, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => TransactionResultResult::txSUCCESS({
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                }),
                x if x == (-1i32 as i32) => TransactionResultResult::txFAILED({
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                }),
                _ => TransactionResultResult::default,
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionResultSet {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionResultSet, usize)> {
        let mut sz = 0;
        Ok((
            TransactionResultSet {
                results: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionSet {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionSet, usize)> {
        let mut sz = 0;
        Ok((
            TransactionSet {
                previousLedgerHash: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                txs: {
                    let (v, fsz) = xdr_codec::unpack_flex(input, None)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionSignaturePayload {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TransactionSignaturePayload, usize)> {
        let mut sz = 0;
        Ok((
            TransactionSignaturePayload {
                networkId: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                taggedTransaction: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TransactionSignaturePayloadTaggedTransaction {
    fn unpack(
        input: &mut In,
    ) -> xdr_codec::Result<(TransactionSignaturePayloadTaggedTransaction, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (2i32 as i32) => {
                    TransactionSignaturePayloadTaggedTransaction::ENVELOPE_TYPE_TX({
                        let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                        sz += fsz;
                        v
                    })
                }
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TrustLineEntry {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TrustLineEntry, usize)> {
        let mut sz = 0;
        Ok((
            TrustLineEntry {
                accountID: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                asset: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                balance: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                limit: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                flags: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TrustLineEntryExt {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TrustLineEntryExt, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => TrustLineEntryExt::Const0,
                x if x == (1i32 as i32) => TrustLineEntryExt::Const1({
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                }),
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TrustLineEntryExtV1 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TrustLineEntryExtV1, usize)> {
        let mut sz = 0;
        Ok((
            TrustLineEntryExtV1 {
                liabilities: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
                ext: {
                    let (v, fsz) = xdr_codec::Unpack::unpack(input)?;
                    sz += fsz;
                    v
                },
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TrustLineEntryExtV1Ext {
    fn unpack(input: &mut In) -> xdr_codec::Result<(TrustLineEntryExtV1Ext, usize)> {
        let mut sz = 0;
        Ok((
            match {
                let (v, dsz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += dsz;
                v
            } {
                x if x == (0i32 as i32) => TrustLineEntryExtV1Ext::Const0,
                v => return Err(xdr_codec::Error::invalidcase(v as i32)),
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for TrustLineFlags {
    #[inline]
    fn unpack(input: &mut In) -> xdr_codec::Result<(TrustLineFlags, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (e, esz): (i32, _) = xdr_codec::Unpack::unpack(input)?;
                sz += esz;
                match e {
                    x if x == TrustLineFlags::AUTHORIZED_FLAG as i32 => {
                        TrustLineFlags::AUTHORIZED_FLAG
                    }
                    e => return Err(xdr_codec::Error::invalidenum(e)),
                }
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for UpgradeType {
    fn unpack(input: &mut In) -> xdr_codec::Result<(UpgradeType, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, Some(128i64 as usize))?;
                sz = usz;
                UpgradeType(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for Value {
    fn unpack(input: &mut In) -> xdr_codec::Result<(Value, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_opaque_flex(input, None)?;
                sz = usz;
                Value(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for string32 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(string32, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, Some(32i64 as usize))?;
                sz = usz;
                string32(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for string64 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(string64, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = xdr_codec::unpack_string(input, Some(64i64 as usize))?;
                sz = usz;
                string64(v)
            },
            sz,
        ))
    }
}

impl<In: xdr_codec::Read> xdr_codec::Unpack<In> for uint256 {
    fn unpack(input: &mut In) -> xdr_codec::Result<(uint256, usize)> {
        let mut sz = 0;
        Ok((
            {
                let (v, usz) = {
                    let mut buf: [u8; 32i64 as usize] = unsafe { ::std::mem::uninitialized() };
                    let sz = xdr_codec::unpack_opaque_array(input, &mut buf[..], 32i64 as usize)?;
                    (buf, sz)
                };
                sz = usz;
                uint256(v)
            },
            sz,
        ))
    }
}
