typedef opaque Value<>;

struct SCPBallot
{
    uint32 counter; // n
    Value value;    // x
};

enum SCPStatementType
{
    SCP_ST_PREPARE = 0,
    SCP_ST_CONFIRM = 1,
    SCP_ST_EXTERNALIZE = 2,
    SCP_ST_NOMINATE = 3
};

struct SCPNomination
{
    Hash quorumSetHash; // D
    Value votes<>;      // X
    Value accepted<>;   // Y
};

struct SCPStatementPledgesPrepare
{
    Hash quorumSetHash;       // D
    SCPBallot ballot;         // b
    SCPBallot* prepared;      // p
    SCPBallot* preparedPrime; // p'
    uint32 nC;                // c.n
    uint32 nH;                // h.n
};

struct SCPStatementPledgesConfirm
{
    SCPBallot ballot;   // b
    uint32 nPrepared;   // p.n
    uint32 nCommit;     // c.n
    uint32 nH;          // h.n
    Hash quorumSetHash; // D
};

struct SCPStatementPledgesExternalize
{
    SCPBallot commit;         // c
    uint32 nH;                // h.n
    Hash commitQuorumSetHash; // D used before EXTERNALIZE
};

union SCPStatementPledges switch (SCPStatementType type)
{
case SCP_ST_PREPARE:
    SCPStatementPledgesPrepare prepare;
case SCP_ST_CONFIRM:
    SCPStatementPledgesConfirm confirm;
case SCP_ST_EXTERNALIZE:
    SCPStatementPledgesExternalize externalize;
case SCP_ST_NOMINATE:
    SCPNomination nominate;
};

struct SCPStatement
{
    NodeID nodeID;    // v
    uint64 slotIndex; // i

    SCPStatementPledges pledges;
};

struct SCPEnvelope
{
    SCPStatement statement;
    Signature signature;
};

// supports things like: A,B,C,(D,E,F),(G,H,(I,J,K,L))
// only allows 2 levels of nesting
struct SCPQuorumSet
{
    uint32 threshold;
    PublicKey validators<>;
    SCPQuorumSet innerSets<>;
};
