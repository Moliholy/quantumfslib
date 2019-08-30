use crate::models::manifest::Manifest;
use crate::types::ipfs::IpfsHash;
use crate::models::catalog::Catalog;
use crate::models::revision::Revision;
use web3::types::Address;
use std::collections::HashMap;
use web3::contract::Error;


#[derive(Debug)]
pub struct Repository {
    manifest: Manifest,
    catalogs: HashMap<IpfsHash, Catalog>,
}

impl Repository {
    pub fn new(address: Address) -> Self {

        let manifest = Manifest::new(address);
        let catalogs: HashMap<IpfsHash, Catalog> = HashMap::new();
        Self {
            manifest,
            catalogs,
        }
    }

    pub fn load_revision(&'static self, revision_number: u128) -> Result<Revision, Error> {
        let tag = self.manifest.fetch_revision(revision_number)?;
        Ok(Revision::new(self, tag))
    }

    pub fn load_current_revision(&'static self) -> Result<Revision, Error> {
        let tag = self.manifest.fetch_last_revision()?;
        Ok(Revision::new(self, tag))
    }
}
