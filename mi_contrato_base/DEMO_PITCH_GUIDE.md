# brevik — Guion de Video de Demostración y Pitch (3 Minutos Máximo)

> Este guion está estructurado específicamente para obtener la calificación máxima (8-10 en cada rubro) según la rúbrica oficial de evaluación del Hackathon (100% de los criterios).

---

## Estructura del Video (Duración Total: 2:50 - 3:00 minutos)

| Tiempo | Sección | Criterio de la Rúbrica Cubierto |
| :--- | :--- | :--- |
| **0:00 - 0:40** | Problema, Usuario y Por Qué Stellar | **Ajuste entre producto y problema (20%)** |
| **0:40 - 1:15** | Pipeline Autónomo: IA + Auto-reparación + Cargo Test | **Ejecución técnica (40%)** |
| **1:15 - 1:55** | Despliegue en Testnet y Propiedad con Freighter | **Ejecución técnica (40%)** |
| **1:55 - 2:30** | Consola Interactiva y Verificación en Stellar Expert | **Impacto y Ejecución técnica (60%)** |
| **2:30 - 2:55** | Beneficiarios (UNAM/Latam), Atribución y Cierre | **Impacto (20%) y Demo/Claridad (20%)** |

---

## Guion Segundo a Segundo

### Minuto 0:00 - 0:40 | El Problema y Ajuste con Stellar (20%)
* **Qué mostrar en pantalla:** La interfaz principal de **brevik** (`http://127.0.0.1:8000`), destacando el logo de brevik y la conexión con Freighter.
* **Voz en off / Narrador:**
  > "Construir soluciones financieras descentralizadas en Soroban suele requerir semanas dominando Rust sin recolector de basura, pruebas unitarias avanzadas y herramientas complejas de terminal. Para estudiantes de la UNAM, desarrolladores y PyMEs en Latinoamérica, esta barrera técnica frena la adopción de Web3.
  > 
  > Presentamos **brevik**: el estudio autónomo que transforma requerimientos de negocio en lenguaje natural a Smart Contracts en Soroban completamente testeados, auditados y desplegados en Stellar Testnet en menos de 60 segundos. 
  > 
  > ¿Por qué en Stellar? Porque es la única red con comisiones de fracciones de centavo ($0.00001), liquidación instantánea en 3 a 5 segundos y soporte nativo para stablecoins como USDC, haciendo financieramente viables micropagos, nóminas y custodias comerciales que en otras cadenas costarían fortunas."

---

### Minuto 0:40 - 1:15 | Demostración Técnica: IA y Lazo de Auto-reparación (40%)
* **Qué mostrar en pantalla:**
  1. Conectar la wallet Freighter (se ve la dirección pública en la barra superior).
  2. Seleccionar una plantilla del mundo real (ej. **Custodia Comercial - Escrow** o **Nómina & Pagos por Hitos**).
  3. Clic en **"Generar, Validar Tests y Desplegar a Testnet"**.
  4. Mostrar las etapas en la barra de progreso.
* **Voz en off / Narrador:**
  > "Conectamos nuestra wallet Freighter: nuestra dirección pública se convertirá automáticamente en el Administrador del contrato on-chain.
  > 
  > Seleccionamos un caso de uso real: un contrato de Custodia Comercial o Nómina por Hitos. Al presionar generar, nuestro backend orquesta el pipeline:
  > 1. Gemini genera el código estricto en Rust y su suite de pruebas unitarias.
  > 2. Si el compilador detecta un error de sintaxis o tipos, nuestro **Lazo de Auto-reparación** intercepta el log de error y lo envía de regreso a la IA para corregirlo automáticamente.
  > 3. Se ejecuta `cargo test`. Si las pruebas fallan, el despliegue se bloquea por seguridad. Solo el código con 100% de tests aprobados pasa a producción."

---

### Minuto 1:15 - 1:55 | Auditoría Soroban Guard y Despliegue On-Chain (40%)
* **Qué mostrar en pantalla:**
  1. Aparece el panel de resultados con el **Contract ID** generado (`C...`).
  2. Mostrar el badge de **Cargo Test Aprobado**.
  3. Mostrar el reporte de **Auditoría Soroban Guard AI** con su puntuación (Score) y los hallazgos de seguridad mitigados (`require_auth`, validación de montos, TTL).
* **Voz en off / Narrador:**
  > "El contrato pasa por **Soroban AI Guard**, una auditoría automatizada que valida permisos con `require_auth`, tiempo de vida del almacenamiento persistente (TTL) y ausencia de desbordamientos aritméticos, otorgando una calificación de seguridad.
  > 
  > El binario WASM optimizado es compilado con `stellar contract build` y desplegado en vivo a Stellar Testnet, registrando al usuario como Dueño legítimo mediante la función `initialize`."

---

### Minuto 1:55 - 2:30 | Consola Interactiva: Cerrando el Ciclo en Testnet (40% + 20%)
* **Qué mostrar en pantalla:**
  1. Bajar a la **Consola de Ejecución Soroban**.
  2. Mostrar cómo las funciones del contrato fueron detectadas automáticamente (`faucet`, `balance`, `transfer`, `depositar`, etc.).
  3. Presionar un botón rápido (ej. `faucet` o `balance`) o ejecutar con **"Mi Wallet"**.
  4. Mostrar la salida en terminal con el hash de transacción (`tx_hash`).
  5. Hacer clic en el enlace a **Stellar Expert** y mostrar la transacción real verificada en la red Testnet de Stellar.
* **Voz en off / Narrador:**
  > "Aquí cerramos el ciclo completo: no solo entregamos código, entregamos uso real. 
  > 
  > Desde nuestra consola interactiva, invocamos en vivo la función del contrato directamente contra Stellar Testnet. Como pueden ver, la transacción se firma, se transmite a la blockchain y aquí tenemos el hash de transacción confirmado. Hacemos clic y lo verificamos en vivo en el explorador oficial Stellar Expert."

---

### Minuto 2:30 - 2:55 | Impacto en UNAM / Latam y Declaración de Transparencia (20% + 20%)
* **Qué mostrar en pantalla:**
  1. Mostrar el repositorio público en GitHub con la documentación impecable y la sección de Atribución de IA.
  2. Mostrar la evidencia on-chain pre-cargada.
* **Voz en off / Narrador:**
  > "Impacto concreto: **brevik** empodera a la comunidad de la UNAM y a desarrolladores de toda Latinoamérica para crear soluciones financieras descentralizadas en minutos, acelerando la adopción masiva del ecosistema Stellar.
  > 
  > Declaración de transparencia: la arquitectura de backend en FastAPI, el lazo de auto-reparación, el harness de pruebas `cargo test`, la consola interactiva y la auditoría Soroban Guard fueron desarrollados 100% durante el hackathon, utilizando la API de Google Gemini como motor de inferencia de código.
  > 
  > El proyecto está listo para clonar y ejecutar en 3 simples pasos. ¡Muchas gracias!"

---

## Consejos para la Grabación
1. Graba a resolución 1080p (Full HD) a pantalla completa.
2. Si la conexión a internet de Testnet tarda unos segundos durante la invocación en vivo, edita o acelera esa espera (2-3 segundos) para mantener el ritmo ágil.
3. Asegúrate de que el audio sea claro y sin eco.
4. Muestra siempre los enlaces de Stellar Expert abriéndose en una pestaña nueva para que el juez vea que las transacciones y contratos son 100% reales en la red.
