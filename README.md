# PC de Pokémon

## Descripción del Proyecto
Este proyecto es un Smart Contract desarrollado en **Rust** utilizando el framework **Anchor** para la blockchain de **Solana**. 

Simula el clásico "Sistema de Almacenamiento de Pokémon" (PC de Bill en la Primera Generación) de los videojuegos, adaptado a la arquitectura Web3. Permite a los usuarios (Entrenadores) inicializar múltiples cajas de almacenamiento, capturar/depositar Pokémon, visualizar su inventario y liberar espacio de forma segura y descentralizada.

---

## Características Principales
* **Arquitectura Multi-Caja (PDAs Dinámicas):** Cada entrenador puede tener hasta 32 cajas independientes. Cada caja es una *Program Derived Address* (PDA) única, generada a partir de la billetera del usuario y el número de caja.
* **Optimización de Memoria:** Uso de `Enums` para atributos de lista cerrada (Naturaleza y Sexo), reduciendo drásticamente los costos de renta en la blockchain en comparación con el uso de `Strings`.
* **Identificadores Únicos (IDs):** Sistema auto-incremental por caja para asignar un ID único a cada Pokémon, evitando conflictos al editar o borrar "clones" (ej. dos Bulbasaur del mismo nivel).
* **Tracking del Entrenador Original (OT):** Inmutabilidad del registro de captura, guardando la `Pubkey` y el nombre de quien depositó al Pokémon por primera vez.
* **Manejo del Tiempo Universal:** Registro de fecha de captura utilizando *Unix Timestamps* para compatibilidad global.

---

## Cómo Usar y Probar el Proyecto (Solana Playground)

Este proyecto está optimizado para ser probado directamente en [Solana Playground (beta.solpg.io)](https://beta.solpg.io/).

### Paso 0: Prerrequisitos
Asegúrate de estar conectado a la red **Devnet** y tener saldo de prueba (SOL) para pagar las transacciones. Si no tienes, escribe `solana airdrop 2` en la terminal de Playground (abajo) y presiona Enter.

### Paso 1: Preparación
1. Pega el código fuente en el archivo `lib.rs` dentro de la carpeta `src`.
2. Ve a la pestaña **Build & Deploy** (icono de herramientas a la izquierda).
3. Haz clic en el botón **Build**.
4. Copia el **Program ID** generado en la sección "Program Credentials" y pégalo en la línea `declare_id!("...");` al inicio de tu código.
5. Vuelve a hacer **Build** y luego haz clic en **Deploy** para subir el contrato a la blockchain.

### Paso 2: Interacción (Pestaña "Test")
Para probar las funciones correctamente, debes configurar los candados criptográficos (Seeds) en la interfaz visual. 

**Regla de Oro:** Siempre que una función pida la cuenta `pcCaja`, deberás derivar la dirección manualmente usando las semillas correspondientes.

#### A. Inicializar una Caja (`inicializar_pc`)
1. Expande la función y llena los argumentos (`numero_caja` ej. `1`, y tu `nombre_entrenador`).
2. En la sección **Accounts -> pcCaja**, haz clic en el icono de configuración (o selecciona *Custom / From seed*).
3. Configura exactamente **3 Semillas (Seeds)**:
   * **Seed 1:** `String` ➔ Valor: `pc-caja`
   * **Seed 2:** `Pubkey` ➔ Valor: Selecciona `Current Wallet`
   * **Seed 3:** `u8` (Número) ➔ Valor: `1` *(Debe coincidir obligatoriamente con el numero_caja de los argumentos).*
4. Haz clic en **Generate** y luego en **Test**.

#### B. Depositar Pokémon (`depositar_pokemon`)
1. Llena los argumentos con los datos estadísticos del Pokémon. **Asegúrate de que el `numero_caja` sea el mismo que inicializaste.**
2. En la sección **Accounts**, repite la configuración exacta de las 3 Semillas (Seeds) detallada en el paso anterior y dale a **Generate**.
3. Haz clic en **Test**.

#### C. Ver Caja (`ver_pc`)
1. Pon el `numero_caja` que deseas consultar en los argumentos.
2. Configura las 3 Semillas apuntando a ese número de caja y genérala.
3. Haz clic en **Test**. 
4. Revisa la terminal negra de Playground para ver el reporte detallado. **Nota:** Fíjate bien en el `id` único que el sistema le asignó a tu Pokémon, lo necesitarás para los siguientes pasos.

#### D. Alternar Estado Shiny (`alternar_shiny` - UPDATE)
1. Llena los argumentos: el `numero_caja` y el **`pokemon_id`** exacto del Pokémon que quieres modificar (ej. `1`).
2. En la sección **Accounts**, vuelve a configurar y generar las 3 Semillas correspondientes a tu caja.
3. Haz clic en **Test**. (Puedes usar `ver_pc` nuevamente para confirmar que el valor `es_shiny` cambió).

#### E. Liberar Pokémon (`liberar_pokemon` - DELETE)
1. Llena los argumentos: el `numero_caja` y el **`pokemon_id`** del Pokémon que deseas liberar.
2. En la sección **Accounts**, configura y genera por última vez las 3 Semillas.
3. Haz clic en **Test**. El Pokémon habrá sido eliminado permanentemente del vector, liberando espacio en la red.

  
## Funciones (CRUD)

| Función | Descripción |
| :--- | :--- |
| `inicializar_pc` | **CREATE**: Reserva el espacio en la blockchain para una Caja específica (1-32) y establece al firmante como dueño. |
| `depositar_pokemon`| **INSERT**: Genera un ID único y guarda un nuevo Pokémon en el vector de la caja correspondiente. |
| `ver_pc` | **READ**: Imprime en los logs de la consola el perfil completo de todos los Pokémon almacenados en una caja. |
| `alternar_shiny` | **UPDATE**: Cambia el estado de un Pokémon (Normal ↔ Shiny) buscando por su ID único. |
| `liberar_pokemon` | **DELETE**: Elimina permanentemente a un Pokémon del arreglo de la caja usando su ID, liberando espacio. |

---

## Seguridad y Validaciones
* **Dueño Exclusivo:** La directiva `has_one = owner` asegura que nadie pueda modificar una caja si no firmó la transacción con la billetera creadora.
* **Validación de Cajas:** Restricciones lógicas (`constraint = numero_caja >= 1 && <= 32`) evitan la creación de cuentas "basura" fuera de los límites del juego.
* **Límites de Espacio:** Cada caja admite exactamente 30 Pokémon.

---
*Desarrollado en Anchor para Solana Web3.*
