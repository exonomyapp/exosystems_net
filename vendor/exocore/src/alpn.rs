// Pattern: /exosystem/<app>/<protocol>/<version>

pub mod exotalk {
    pub const GOSSIP: &[u8] = b"/exosystem/exotalk/v1/gossip";
    pub const BLOBS: &[u8] = b"/exosystem/exotalk/v1/blobs";
    pub const STREAM: &[u8] = b"/exosystem/exotalk/v1/stream";
}

pub mod exonomy {
    pub const GOSSIP: &[u8] = b"/exosystem/exonomy/v1/gossip";
    pub const BLOBS: &[u8] = b"/exosystem/exonomy/v1/blobs";
}

pub mod exocracy {
    pub const GOSSIP: &[u8] = b"/exosystem/exocracy/v1/gossip";
    pub const BLOBS: &[u8] = b"/exosystem/exocracy/v1/blobs";
}

pub mod republet {
    pub const GOSSIP: &[u8] = b"/exosystem/republet/v1/gossip";
    pub const BLOBS: &[u8] = b"/exosystem/republet/v1/blobs";
}

pub mod conscia {
    pub const GOSSIP: &[u8] = b"/exosystem/conscia/v1/gossip";
    pub const BLOBS: &[u8] = b"/exosystem/conscia/v1/blobs";
    pub const AUTH: &[u8] = b"/exosystem/conscia/v1/auth";
}

pub fn all_alpns() -> Vec<&'static [u8]> {
    vec![
        exotalk::GOSSIP,
        exotalk::BLOBS,
        exotalk::STREAM,
        exonomy::GOSSIP,
        exonomy::BLOBS,
        exocracy::GOSSIP,
        exocracy::BLOBS,
        republet::GOSSIP,
        republet::BLOBS,
        conscia::GOSSIP,
        conscia::BLOBS,
        conscia::AUTH,
        iroh_gossip::ALPN,
        iroh_blobs::ALPN,
    ]
}
