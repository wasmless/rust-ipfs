#![allow(dead_code)]
use crate::error::Error;
use crate::path::{IpfsPath, PathRoot};
use crate::repo::RepoTypes;
use crate::Ipfs;

mod dnslink;
mod entry;
mod ipns_pb {
    include!(concat!(env!("OUT_DIR"), "/ipns_pb.rs"));
}

#[derive(Clone, Debug)]
pub struct Ipns<Types: RepoTypes> {
    ipfs: Ipfs<Types>,
}

impl<Types: RepoTypes> Ipns<Types> {
    pub fn new(ipfs: Ipfs<Types>) -> Self {
        Ipns { ipfs }
    }

    /// Resolves a ipns path to an ipld path.
    pub async fn resolve(&self, path: &IpfsPath) -> Result<IpfsPath, Error> {
        let path = path.to_owned();
        match path.root() {
            PathRoot::Ipld(_) => Ok(path),
            PathRoot::Ipns(_) => Err(anyhow::anyhow!("unimplemented")),
            PathRoot::Dns(domain) => Ok(dnslink::resolve(domain).await?),
        }
    }
}
