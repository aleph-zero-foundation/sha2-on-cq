use crate::{
    sha_ops::{choose, majority, rot0, rot1, witness_op1, witness_op2},
    types::{decompose, Limb, Word, WordSum},
};

trait LookupTable {
    type Query;
    type Result;

    fn lookup(&self, query: Self::Query) -> Self::Result;
}

macro_rules! lookup_table {
    ($table_name:ident, $query:ty, $result:ty, $action:expr) => {
        pub struct $table_name;
        impl LookupTable for $table_name {
            type Query = $query;
            type Result = $result;

            fn lookup(&self, query: Self::Query) -> Self::Result {
                $action(query)
            }
        }
    };
}

lookup_table!(TMaj, [Limb; 3], Limb, |query: [Limb; 3]| majority(
    query[0], query[1], query[2]
));
lookup_table!(TCh, [Limb; 3], Limb, |query: [Limb; 3]| choose(
    query[0], query[1], query[2]
));
lookup_table!(TRot0, [Limb; 3], Word, |query| rot0(query));
lookup_table!(TRot1, [Limb; 3], Word, |query| rot1(query));
lookup_table!(TDec, WordSum, [Limb; 3], |query| decompose(
    &(query as Word)
));
lookup_table!(TW1, Word, Word, witness_op1);
lookup_table!(TW2, Word, Word, witness_op2);
lookup_table!(TMod, WordSum, Word, |query| query as Word);
