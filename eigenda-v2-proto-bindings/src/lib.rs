// It is important to maintain the same structure as in the proto.
// 'proto' directory name is replaced with 'generated'
pub mod generated {
    pub mod common {
        include!(concat!(env!("OUT_DIR"), "/common.rs"));

        pub mod v2 {
            include!(concat!(env!("OUT_DIR"), "/common.v2.rs"));
        }
    }

    pub mod disperser {
        pub mod v2 {
            include!(concat!(env!("OUT_DIR"), "/disperser.v2.rs"));
        }
    }

    pub mod encoder {
        pub mod v2 {
            include!(concat!(env!("OUT_DIR"), "/encoder.v2.rs"));
        }
    }

    pub mod retriever {
        pub mod v2 {
            include!(concat!(env!("OUT_DIR"), "/retriever.v2.rs"));
        }
    }

    pub mod validator {
        include!(concat!(env!("OUT_DIR"), "/validator.rs"));
    }
}

use generated::{common, disperser, encoder, retriever, validator};

pub fn dummy() {
    common::BlobCommitment::default();
    disperser::v2::DisperseBlobRequest::default();
    encoder::v2::FragmentInfo::default();
    retriever::v2::BlobRequest::default();
    validator::StoreChunksRequest::default();
}
