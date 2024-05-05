use crate::episode::Episode;
use nestify::nest;
use serde::{Deserialize, Serialize};

nest! {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]*
    pub struct Source {
        pub id: Option<u32>,
        pub name: String,
        pub priority: u32,
        pub types: Vec<
            #[derive(Clone, Copy, Eq)]
            pub enum SourceTypes {
                Raw,
                EnglishDub,
                PortugueseDub,
                EnglishSub,
                PortugueseSub,
                NarcoSub,
                NarcoDub,
            }
        >,
        pub episodes: Vec<Episode>
    }
}
