use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc, Promise, ext_contract,AccountId, Balance, PromiseResult, PublicKey,};
use near_sdk::json_types::{Base58PublicKey};

//Función que nos regresa el valor de 1 NEAR en un u128
fn one_near() -> u128 {
    u128::from_str_radix("1000000000000000000000000", 10).unwrap()
}

/// Gas attached to the callback from account creation.
pub const ON_CREATE_ACCOUNT_CALLBACK_GAS: u64 = 20_000_000_000_000;

/// Access key allowance for linkdrop keys.
const ACCESS_KEY_ALLOWANCE: u128 = 1_000_000_000_000_000_000_000_000;

/// Indicates there are no deposit for a callback for better readability.
const NO_DEPOSIT: u128 = 0;

#[ext_contract(ext_self)]
pub trait ExtLinkDrop {
    /// Callback after plain account creation.
    fn on_account_created(&mut self, predecessor_account_id: AccountId, amount: u128) -> bool;

    /// Callback after creating account and claiming linkdrop.
    fn on_account_created_and_claimed(&mut self, amount: u128) -> bool;
}

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]

pub struct PerritoContract {
    perritos: UnorderedMap<String, Perrito>,
}

impl Default for PerritoContract {
    fn default() -> Self {
        Self {
            perritos: UnorderedMap::new(b"p".to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]

pub struct Perrito {
    pub cuenta: String,
    pub nombre: String,
    pub refugio: String,
    pub adoptado: bool,
    pub meta: u64,
}

impl Default for Perrito {
    fn default() -> Self {
        Perrito {
            cuenta: String::from(""),
            nombre: String::from(""),
            refugio: String::from(""),
            adoptado: false,
            meta: 0,
        }
    }
}

impl Perrito {
    pub fn new(cuenta: String, nombre: String ,refugio: String, meta: u64) -> Self {
        Self {
            cuenta,
            nombre,
            refugio,
            adoptado: false,
            meta,
        }
    }
}

#[near_bindgen]
impl PerritoContract
{
    #[payable]
    pub fn set_registrar_perrito(&mut self,refugio: String, nombre: String, meta: u64){
        let cuentaWallet =  "";
        let cuenta   =  env::signer_account_id();
        let deposito =  env::attached_deposit();

        assert!(refugio.len() > 0,"Escriba el nombre del Refugio.");
        assert!(nombre.len() >  0,"Debe de dar nombre al perrito");
        assert!(meta > 0,"Debe de establecer el monto a cumplir");
        assert!(deposito >= one_near(),"Debes de pagar 1 NEAR para registrarte.");

        let perrito = Perrito::new(String::from(&cuenta), String::from(&nombre), String::from(&refugio), meta);
        self.perritos.insert(&cuenta, &perrito);

        env::log(format!("Perrito registrado correctamente.").as_bytes());
    }

    #[payable]
    pub fn create_account(&mut self,new_account_id: AccountId,new_public_key: Base58PublicKey,) -> Promise {
        assert!(
            env::is_valid_account_id(new_account_id.as_bytes()),
            "Invalid account id"
        );
        let amount = env::attached_deposit();
        Promise::new(new_account_id)
            .create_account()
            .add_full_access_key(new_public_key.into())
            .transfer(amount)
            .then(ext_self::on_account_created(
                env::predecessor_account_id(),
                amount.into(),
                &env::current_account_id(),
                NO_DEPOSIT,
                ON_CREATE_ACCOUNT_CALLBACK_GAS,
            ))
    }
    
    pub fn get_perrito(&self, cuenta: String) -> Option<Perrito> {
        self.perritos.get(&cuenta)
    }

    pub fn get_perritos(&self) -> Vec<Perrito> {
        self.perritos.values_as_vector().to_vec()
    }

    fn ft_on_transfer(
        &mut self,
        receptor_id: String,
        monto: u128,
    ) 
    {
        // Verifying that we were called by fungible token contract that we expect.
        /*assert_eq!(
            &env::predecessor_account_id(),
            &self.fungible_token_account_id,
            "Only supports the one fungible token contract"
        );*/
        
        
        /*Promise::new(env::predecessor_account_id()).transfer(monto);*/

        /*match msg.asstr() {monto0)),=> {
                let prepaid_gas = env::prepaid_gas();
                let account_id = env::current_account_id();
                ext_self::value_please(
                    msg,
                    account_id,
                    NO_DEPOSIT,
                    prepaid_gas - Gas::from(GAS_FOR_FT_ON_TRANSFER),
                )
                .into()
            }
        }*/
    }

    pub fn set_pago(&mut self,cuenta: String,donador_id: String ,monto: u64,) -> bool{
        match self.perritos.get(&cuenta) {
            Some(mut perrito) => {
                perrito.meta = monto;

                ///Promise::new(String::from(&cuenta)).transfer(5 as u128);
                
                self.perritos.insert(&cuenta, &perrito);
                env::log(format!("Donación realizada.").as_bytes());
                
                self.ft_on_transfer(donador_id,1);

                true
            }
            None => {
                env::log(format!("No se pudo donar.").as_bytes());
                false
            }
        }

    }

    pub fn set_adoptado(&mut self, cuenta: String) -> bool {
        assert!(
            env::signer_account_id() == "dev-1654151703614-97110545887614",
            "No tienes permisos para ejecutar este comando."
        );

        match self.perritos.get(&cuenta) {
            Some(mut perrito) => {
                perrito.adoptado = true;

                ///Promise::new(String::from(&cuenta)).transfer(5 as u128);
                
                self.perritos.insert(&cuenta, &perrito);
                env::log(format!("Perrito adoptado.").as_bytes());

                true
            }
            None => {
                env::log(format!("El perrito no existe.").as_bytes());
                false
            }
        }

    }
}
