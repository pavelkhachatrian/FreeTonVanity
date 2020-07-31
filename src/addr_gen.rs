use ton_sdk::{ContractImage};
use std::fs::OpenOptions;
use ed25519_dalek::Keypair;
use rand::rngs::ThreadRng;
use bip39::{MnemonicType, Language, Mnemonic};
use crate::addr_gen::hdkey::{HDPrivateKey, KeyPair, sign_keypair_from_secret_key};

mod hdkey;

const HD_PATH: &str = "m/44'/396'/0'/0/0";


pub struct Account {
    pub account_id: String,
    pub keypair: Keypair,
    pub seed: String,
    pub tvc: u8,
}

impl Account {
    pub fn public_as_string(&self) -> String {
        hex::encode(self.keypair.public.as_bytes())
    }
    pub fn secret_as_string(&self) -> String {
        hex::encode(self.keypair.secret.as_bytes())
    }
}

pub struct AccountGenerator {
    pub contract_image: ContractImage,
    pub csprng: ThreadRng,
    pub tvc: u8
}


impl AccountGenerator {
    pub fn from_tvc_file(path: &str) -> Result<Self, String> {
        let mut state_init = OpenOptions::new().read(true).open(path)
            .map_err(|e| format!("unable to open contract file: {}", e))?;

        let contract_image = ton_sdk::ContractImage::from_state_init(&mut state_init)
            .map_err(|e| format!("unable to load contract image: {}", e))?;
        let csprng = rand::thread_rng();

        Ok(Self { contract_image, csprng, tvc: 1})
    }

    pub fn generate_keyair(&mut self) -> Keypair {
        Keypair::generate(&mut self.csprng)
    }

    fn generate_address(&self) -> String {
        let mut addr = hex::encode(self.contract_image.account_id().cell().cell_data().data());
        addr.truncate(64);
        addr
    }

    fn generate_account(&self, keypair: Keypair) -> Account {
        self::Account { account_id: self.generate_address(), keypair, seed: String::new(), tvc: self.tvc}
    }

    #[allow(unused_must_use)]
    pub fn generate_account_from_keypair(&mut self, keypair: Keypair) -> Account {
        self.contract_image.set_public_key(&keypair.public);
        self.generate_account(keypair)
    }

    #[allow(unused_must_use)]
    pub fn generate_random_account(&mut self) -> Account {
        let keypair = self.generate_keyair();
        self.contract_image.set_public_key(&keypair.public);
        self::Account { account_id: self.generate_address(), keypair, seed: String::new(), tvc: self.tvc }
    }


    #[allow(unused_must_use)]
    pub fn generate_account_from_random_seed(&mut self) -> Account {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let seed: String = mnemonic.phrase().into();
        let hdk = HDPrivateKey::from_mnemonic(&seed)
            .derive_path(&HD_PATH.to_string(), false);

        let keypair: KeyPair = sign_keypair_from_secret_key(hdk.secret());
        let keypair = keypair.decode();
        self.contract_image.set_public_key(&keypair.public);
        let account_id= self.generate_address();
        self::Account { account_id, keypair, seed , tvc: self.tvc}

    }
}

