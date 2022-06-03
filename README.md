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

# Mockups
===================

![image](https://user-images.githubusercontent.com/20521029/171917425-ef2a05ec-3396-4188-9093-9a9e8e28bf62.png)

![image](https://user-images.githubusercontent.com/20521029/171947238-7426d8b0-f745-4117-9f1f-50b6a5056139.png)

![image](https://user-images.githubusercontent.com/20521029/171947307-e3c3ce83-2593-49f9-9fac-ddfe6709626d.png)

![image](https://user-images.githubusercontent.com/20521029/171947376-1ecf46e1-d13a-4da1-b81b-45e0002e2822.png)

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

En la carpeta raiz del proyecto, inicia sesión en tu billetera con `near-cli` ejecutando el siguiente comando:

    near login

Para hacer el tutorial más fácil de replicar, asignaremos una variable de entorno para la cuenta. PAra replicar el ejercicio, en el comando siguiente reemplaza `dev-1654151703614-97110545887614` con la cuenta que utilizarás para realizar las pruebas, incluyendo `.near`:

    ID=dev-1654151703614-97110545887614

Puedes validar si la variable de entorno está asignada correctamente si se muestra al ejecutar el siguiente comando:

    echo $ID

Ahora podemos lanzar la compilación de este contrato dentro de tu cuenta:

    near deploy --wasmFile res/fungible_token.wasm --accountId $ID

Obtén el metadata:

    near view $ID ft_metadata


Ejemplo de Registro de Perrito
---------------

llamaremos la cuenta dev-1654151703614-97110545887614 para generar un nuevo registro.

    near call $ID set_registrar_perrito '{"refugio": "Refugio 8", "nombre": "Firulais", "meta": "100" }' --accountId $ID --amount 1



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


