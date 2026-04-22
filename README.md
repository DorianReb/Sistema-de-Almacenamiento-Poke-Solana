# 🖥️ PC de Pokémon - Smart Contract en Solana (GameFi)

## 📖 Descripción del Proyecto
Este proyecto es un Smart Contract desarrollado en **Rust** utilizando el framework **Anchor** para la blockchain de **Solana**. 

Simula el clásico "Sistema de Almacenamiento de Pokémon" (PC de Bill) de los videojuegos, adaptado a la arquitectura Web3. Permite a los usuarios (Entrenadores) inicializar múltiples cajas de almacenamiento, capturar/depositar Pokémon, visualizar su inventario y liberar espacio de forma segura y descentralizada.

---

## ✨ Características Principales
* **Arquitectura Multi-Caja (PDAs Dinámicas):** Cada entrenador puede tener hasta 32 cajas independientes. Cada caja es una *Program Derived Address* (PDA) única, generada a partir de la billetera del usuario y el número de caja.
* **Optimización de Memoria:** Uso de `Enums` para atributos de lista cerrada (Naturaleza y Sexo), reduciendo drásticamente los costos de renta en la blockchain en comparación con el uso de `Strings`.
* **Identificadores Únicos (IDs):** Sistema auto-incremental por caja para asignar un ID único a cada Pokémon, evitando conflictos al editar o borrar "clones" (ej. dos Bulbasaur del mismo nivel).
* **Tracking del Entrenador Original (OT):** Inmutabilidad del registro de captura, guardando la `Pubkey` y el nombre de quien depositó al Pokémon por primera vez.
* **Manejo del Tiempo Universal:** Registro de fecha de captura utilizando *Unix Timestamps* para compatibilidad global.

---

## 🛠️ Cómo Usar y Probar el Proyecto (Solana Playground)

Este proyecto está optimizado para ser probado en [Solana Playground (beta.solpg.io)](https://beta.solpg.io/).

### Paso 1: Preparación
1. Pega el código fuente en el archivo `lib.rs` dentro de la carpeta `src`.
2. Ve a la pestaña **Build & Deploy** (icono de herramientas a la izquierda).
3. Haz clic en **Build**.
4. Copia el **Program ID** generado en "Program Credentials" y pégalo en la línea `declare_id!("...");` al inicio de tu código.
5. Vuelve a hacer **Build** y luego haz clic en **Deploy**.

### Paso 2: Interacción (Pestaña "Test")
Para probar las funciones correctamente, debes configurar los candados criptográficos (Seeds) en la interfaz de pruebas.

#### A. Inicializar una Caja (`inicializar_pc`)
1. Expande la función y llena los argumentos (`numero_caja` ej. `1`, y `nombre_entrenador`).
2. En la sección **Accounts -> pcCaja**, despliega el menú y selecciona **Seeds**.
3. Configura exactamente **3 Semillas (Seeds)**:
   * **Seed 1:** `String` ➔ Valor: `pc-caja`
   * **Seed 2:** `Pubkey` ➔ Valor: `owner` (o Current Wallet)
   * **Seed 3:** `u8` ➔ Valor: `1` *(Debe coincidir con el numero_caja de los argumentos).*
4. Haz clic en **Test**.

#### B. Depositar Pokémon (`depositar_pokemon`)
1. Llena los datos estadísticos del Pokémon. **Asegúrate de que el `numero_caja` sea el mismo que inicializaste.**
2. En la sección **Accounts**, repite la configuración exacta de las 3 Semillas (Seeds) detallada en el paso anterior.
3. Haz clic en **Test**.

#### C. Ver Caja (`ver_pc`)
1. Pon el `numero_caja` que deseas consultar.
2. Configura las 3 Semillas apuntando a ese número de caja.
3. Haz clic en **Test**. 
4. Revisa la consola de Playground (abajo) para ver el reporte detallado del contenido de la caja.

---

## 📚 Funciones (CRUD)

| Función | Descripción |
| :--- | :--- |
| `inicializar_pc` | **CREATE**: Reserva el espacio en la blockchain para una Caja específica (1-32) y establece al firmante como dueño. |
| `depositar_pokemon`| **INSERT**: Genera un ID único y guarda un nuevo Pokémon en el vector de la caja correspondiente. |
| `ver_pc` | **READ**: Imprime en los logs de la consola el perfil completo de todos los Pokémon almacenados en una caja. |
| `alternar_shiny` | **UPDATE**: Cambia el estado de un Pokémon (Normal ↔ Shiny) buscando por su ID único. |
| `liberar_pokemon` | **DELETE**: Elimina permanentemente a un Pokémon del arreglo de la caja usando su ID, liberando espacio. |

---

## 🔒 Seguridad y Validaciones
* **Dueño Exclusivo:** La directiva `has_one = owner` asegura que nadie pueda modificar una caja si no firmó la transacción con la billetera creadora.
* **Validación de Cajas:** Restricciones lógicas (`constraint = numero_caja >= 1 && <= 32`) evitan la creación de cuentas "basura" fuera de los límites del juego.
* **Límites de Espacio:** Cada caja admite exactamente 30 Pokémon.

---
*Desarrollado en Anchor para Solana Web3.*
