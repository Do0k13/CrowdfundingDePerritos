# LovePaws Crowdfunding
===================

Smart Contract desarrollado en la Blockchain de NEAR protocol, es un ejemplo de implementación de un contrato que sigue los estándares NEAR ([near-contract-standards]) y los estándares de pruebas de sumulación ([simulation]) .

 [near-contract-standards]: https://github.com/near/near-sdk-rs/tree/master/near-contract-standards
  [simulation]: https://github.com/near/near-sdk-rs/tree/master/near-sdk-sim

# Iniciativa
===================

Presentación de la Iniciativa: https://docs.google.com/presentation/d/e/2PACX-1vRE7iMxj0v_ypMXnvHAP0YYS-Mrt-elylu1gnikHQE60MhtVXPkB0v_GYfg9QwbbyUyaQWQEReSnYrQ/pub?start=true&loop=false&delayms=10000

Nuestra propuesta es enfocada a apoyar a perritos en situación de abandono, ya sea en la calle o en albergues, la meta inicial es conseguir apoyo para gastos como alimentación, veterinario y aseo. Nuestro contrato permite recibir donaciones desde wallets NEAR, dar de alta nuevas subcuentas en la wallet, dar de alta perritos con detalles como donación requerida y el albergue en el que se encuentran y consultar datos sobre algún perrito en específico; los puntos a mejorar serían los siguientes en una versión 2.0 :

*Recibir donativos en especie y generar nuevas wallets para los donadores

*Presentar integración con wallet NEAR directamente desde el portal web

*Enviar unos Tokens PRTO de recompensa a donadores

*Alianzas estratégicas con proveedores para promociones con tokens PRTO

![image](https://user-images.githubusercontent.com/20521029/171917425-ef2a05ec-3396-4188-9093-9a9e8e28bf62.png)



Prerequisitos
=============

1. Asegúrate de que Rust se encuentre correctamente instalado [`near-sdk-rs`](https://github.com/near/near-sdk-rs#pre-requisites)
2. Asegúrate que `near-cli` esté instalado ejecutando `near --version`. Si no está instalado, realizarlo con: `npm install -g near-cli`

## Compilación

Para compilar ejecuta:
```bash
cd scripts && ./build.sh
```

Usando este Contrato
===================

Este smart contract será lanzado a tu cuenta NEAR. Para este ejemplo es necesario generar inicialmente la cuenta NEAR. Si quieres ejecutar este ejemplo en una cuenta NEAR que ha tenido anteriormente contratos lanzados, de favor usa el comando `near-cli` y el comando `near delete`, y después regenerarlo en la billetera. PAra generar (o regenerarl) una cuenta, de favor sigue las instrucciones de: [NEAR Wallet](https://wallet.near.org/).

In the project root, log in to your newly created account  with `near-cli` by following the instructions after this command:

    near login

To make this tutorial easier to copy/paste, we're going to set an environment variable for your account id. In the below command, replace `MY_ACCOUNT_NAME` with the account name you just logged in with, including the `.near`:

    ID=refugiolomito.testnet

You can tell if the environment variable is set correctly if your command line prints the account name after this command:

    echo $ID

Now we can deploy the compiled contract in this example to your account:

    near deploy --wasmFile res/fungible_token.wasm --accountId $ID

FT contract should be initialized before usage. You can read more about metadata at ['nomicon.io'](https://nomicon.io/Standards/FungibleToken/Metadata.html#reference-level-explanation). Modify the parameters and create a token:

    near call $ID new '{"owner_id": "'$ID'", "total_supply": "1000000", "metadata": { "spec": "ft-1.0.0", "name": "Perrito Token", "symbol": "PRTO", "decimals": 8 }}' --accountId $ID

Get metadata:

    near view $ID ft_metadata


Ejemplo de Transferencia de Token
---------------

Usaremos la cuenta do0k13.testnet para trasferir fondos.

Se agrega la variable storage_deposit para la cuenta:

    near call $ID storage_deposit '{"account_id": "do0k13.testnet", "registration_only": true }' --accountId do0k13.testnet --amount 0.00125


Se valida el balance de la cuenta, actualmente debería ser cero:

    near view $ID ft_balance_of '{"account_id": "'do0k13.testnet'"}'

Se transfieren tokens a la wallet desde el contrato original, se debe adjuntar exactamente 1 yoctoNEAR de depósito:

    near call $ID ft_transfer '{"receiver_id": "'do0k13.testnet'", "amount": "19"}' --accountId $ID --amount 0.000000000000000000000001


Se valida nuevamente el balance y debería ser `19`.

## Testing

As with many Rust libraries and contracts, there are tests in the main fungible token implementation at `ft/src/lib.rs`.

Additionally, this project has [simulation] tests in `tests/sim`. Simulation tests allow testing cross-contract calls, which is crucial to ensuring that the `ft_transfer_call` function works properly. These simulation tests are the reason this project has the file structure it does. Note that the root project has a `Cargo.toml` which sets it up as a workspace. `ft` and `test-contract-defi` are both small & focused contract projects, the latter only existing for simulation tests. The root project imports `near-sdk-sim` and tests interaction between these contracts.

You can run unit tests with the following command:

```bash
cd ft && cargo test -- --nocapture --color=always
```

You can run integration tests with the following commands:
*Rust*
```bash
cd integration-tests/rs && cargo run --example integration-tests
```
*TypeScript*
```bash
cd integration-tests/ts && yarn && yarn test
```

## Notes

 - The maximum balance value is limited by U128 (`2**128 - 1`).
 - JSON calls should pass U128 as a base-10 string. E.g. "100".
 - This does not include escrow functionality, as `ft_transfer_call` provides a superior approach. An escrow system can, of course, be added as a separate contract or additional functionality within this contract.

## No AssemblyScript?

[near-contract-standards] is currently Rust-only. We strongly suggest using this library to create your own Fungible Token contract to ensure it works as expected.

Someday NEAR core or community contributors may provide a similar library for AssemblyScript, at which point this example will be updated to include both a Rust and AssemblyScript version.

## Contributing

When making changes to the files in `ft` or `test-contract-defi`, remember to use `./build.sh` to compile all contracts and copy the output to the `res` folder. If you forget this, **the simulation tests will not use the latest versions**.

Note that if the `rust-toolchain` file in this repository changes, please make sure to update the `.gitpod.Dockerfile` to explicitly specify using that as default as well.


