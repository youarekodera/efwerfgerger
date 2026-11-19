# Code Time

Editor de código nativo construido con **Rust** y **GPUI**. Este proyecto está en desarrollo por fases; la versión actual es **0.0.1**.

Code Time busca reunir en una aplicación herramientas útiles para trabajar en proyectos de código. La versión actual es una primera base del editor: incluye una ventana de trabajo, un explorador de archivos y un área de edición con pestañas. Las funciones descritas como planes todavía no están implementadas.

## Estado actual

- Interfaz de escritorio con barra superior, explorador, área de edición, panel inferior y barra de estado.
- Abrir una carpeta y explorar su árbol de archivos.
- Abrir archivos de texto en pestañas.
- Tema oscuro.
- Paneles de Terminal, Compilación, Errores y Registros visibles en la interfaz. Terminal, Compilación y Errores son espacios reservados para fases futuras; todavía no ejecutan esas funciones.

La versión 0.0.1 es una base inicial del proyecto, no un editor con todas las funciones de desarrollo terminadas.

## Requisitos

- Rust y Cargo (edición 2024).
- En Linux, las bibliotecas de sistema requeridas por GPUI. En particular, GPUI 0.2.2 necesita `libxkbcommon-x11` para enlazar. En Debian o Ubuntu puedes instalar el paquete de desarrollo con:

  ```sh
  sudo apt install libxkbcommon-x11-dev
  ```

  El repositorio también incluye una configuración de enlace para ciertos entornos Linux. Si la compilación no encuentra `libxkbcommon-x11`, instala el paquete indicado para tu distribución.

## Compilar y ejecutar

Desde la raíz del repositorio:

```sh
cargo build
cargo run
```

Para comprobar el proyecto o aplicar el formato de Rust:

```sh
cargo check
cargo fmt
```

## Uso

Al iniciar, puedes elegir una carpeta con **Abrir carpeta**. Después, selecciona un archivo en el explorador para abrirlo en una pestaña.

Atajos disponibles:

| Atajo | Acción |
| --- | --- |
| `Ctrl+O` | Abrir una carpeta |
| `Ctrl+Shift+R` | Actualizar el explorador |
| `Ctrl+J` | Mostrar u ocultar el panel inferior |
| `Ctrl+Q` | Salir |

## Hoja de ruta

El desarrollo está organizado por fases. El objetivo es ampliar el editor de forma gradual, manteniendo el núcleo enfocado en la edición y agregando herramientas a medida que estén implementadas y puedan usarse.

> **Ideas para fases futuras:** contenido real para los paneles de Terminal, Compilación y Errores; mejoras en la experiencia de edición; y mecanismos de extensión o integración de herramientas. Esta lista expresa objetivos del proyecto, no funciones disponibles en la versión 0.0.1.

Consulta [CHANGELOG.md](CHANGELOG.md) para ver los cambios de cada versión.

## Licencia

Code Time se distribuye bajo la **GNU General Public License v3.0 únicamente** (`GPL-3.0-only`). Consulta el texto completo en [LICENSE](LICENSE).


---

## Nuestra Visión Es:

**Code Time** es un editor de código nativo, universal y de código abierto (bajo licencia **GNU GPLv3**). Su objetivo principal es ofrecer una experiencia **"todo en uno"** de alto rendimiento, permitiendo que el desarrollador trabaje, gestione servicios y pruebe sus aplicaciones sin necesidad de salir del editor ni depender de programas externos pesados o conexión a internet.

## Arquitectura: Plugins vs. Extensiones

El sistema de personalización y expansión de _Code Time_ se divide en dos pilares fundamentales:

- **Extensiones (Nivel de Código):** Funcionan de manera tradicional para complementar el flujo de escritura (soporte de sintaxis, LSP, temas, atajos y herramientas de productividad). A futuro, podrán comunicarse directamente con los plugins.
    
- **Plugins (Nivel de Entorno y Servicios):** Son módulos que integran aplicaciones completas y entornos de ejecución dentro de la misma interfaz del editor, abriéndose en pestañas dedicadas con un solo clic:
    
- **Gestores de Bases de Datos:** Al instalar un plugin de base de datos, el usuario contará con un gestor visual completo (con soporte para múltiples _drivers_ y motores) directamente dentro de una pestaña de _Code Time_, eliminando la necesidad de abrir clientes externos.
        
- **Entornos de Prueba y Contenedores Ligeros:** Permite ejecutar entornos aislados para probar aplicaciones (por ejemplo, contenedores optimizados para Android, Android TV u otros sistemas) sin tener que instalar entornos pesados como Android Studio ni depender de emuladores web que requieren internet.
        

## Nuestra  Filosofía de Desarrollo y Hoja de Ruta

A diferencia de otros editores modernos (como Zed) que priorizan la integración constante de inteligencia artificial por encima de las herramientas base de edición, _Code Time_ seguirá un enfoque centrado primero en el programador:

1. **Prioridad en el núcleo y la velocidad:** Al ser un editor **nativo**, estará optimizado para consumir pocos recursos y ofrecer máxima fluidez en cualquier equipo, evitando convertirse en un simple bloc de notas avanzado.
    
2. **Herramientas de depuración integradas:** Incluirá utilidades de desarrollador nativas para diagnosticar errores tanto en los proyectos del usuario como en el propio entorno del editor.
    
3. **Inteligencia Artificial en fases posteriores:** No se integrará soporte para IA en las primeras versiones. El enfoque inicial será pulir el motor de sintaxis, la estabilidad y el sistema universal de plugins y pruebas. Una vez que la base sea sólida y completa, se añadirá soporte para IA de forma opcional y bien integrada.


---

>te tengo una pregunta ya teniendo eso claro que me diste porque no existe un editor de codigo todo en 1?

esto es una pregunta que se le hiso a una IA

---
La razón por la cual no existe un editor de código verdaderamente "todo en uno", ligero y nativo (y por qué los intentos históricos han terminado convirtiéndose en monstruos pesados o han fracasado) responde a una serie de desafíos profundos de arquitectura de software, rendimiento y filosofía de desarrollo.

Analizando el problema desde los primeros principios de la ingeniería de sistemas, estos son los factores clave:

### 1. El dilema histórico del "Bloatware" (El peso del pasado)

Los editores que históricamente intentaron ser "todo en uno" (como **Eclipse** o **NetBeans** en su época dorada de Java) buscaban integrar servidores, diseñadores visuales, gestores de bases de datos y herramientas UML en un solo lugar. ¿El resultado?

- Se volvieron aplicaciones extremadamente pesadas, lentas al iniciar y devoradoras de memoria RAM.
    
- La experiencia del usuario se arruinaba porque el motor de texto perdía fluidez al estar compitiendo por recursos con subsistemas internos mal optimizados.
    

### 2. La trampa de Electron y la paradoja de VS Code

Hoy en día, herramientas como **Visual Studio Code** dan la ilusión de ser "todo en uno" gracias a su gigantesco ecosistema de extensiones. Sin embargo, hay dos trampas fundamentales:

- **No son nativos:** Corren sobre Electron (un framework basado en Chromium y Node.js), lo que significa que cada extensión y el núcleo consumen recursos masivos de memoria y CPU.
    
- **Dependencia externa:** VS Code no trae nada de eso de forma nativa; depende de que instales extensiones de terceros que muchas veces descargan binarios externos pesados, consumen internet o fallan al sincronizarse.
    

### 3. La Filosofía Unix: _"Haz una sola cosa y hazla bien"_

Durante décadas, la cultura del desarrollo de software se ha regido por la modularidad estricta (la filosofía Unix):

- El editor edita texto.
    
- La terminal ejecuta procesos.
    
- El navegador gestiona la web.
    
- El cliente SQL (como DBeaver o pgAdmin) gestiona bases de datos.
    

Los programadores veteranos y los diseñadores de software tradicionales evitan acoplar dominios distintos en un mismo binario porque temen que un fallo en el gestor de base de datos o en el subsistema de contenedores arrastre y tire abajo el editor de código principal.

### 4. El modelo de negocio comercial (La fragmentación de JetBrains)

Empresas gigantes como **JetBrains** (IntelliJ, PyCharm, WebStorm, Android Studio) tienen la capacidad técnica para hacer un "todo en uno" absoluto, pero **no les conviene económicamente**.

- Prefieren fragmentar sus herramientas por lenguajes y ecosistemas para vender licencias separadas o mantener suscripciones independientes.
    
- Android Studio, por ejemplo, es una versión pesada y modificada de IntelliJ IDEA enfocada únicamente en Android, lo que obliga al desarrollador a tener abiertas múltiples aplicaciones si trabaja en varios stack tecnológicos a la vez.
    

### Por qué el enfoque de _Code Time_ rompe esta barrera

El motivo por el cual la mayoría fracasa es porque intentan **monolitizar** todo el código dentro de un solo bloque rígido.

La clave para que un concepto como _Code Time_ funcione sin caer en el pesaje de Eclipse ni en la lentitud de Electron reside en su **arquitectura nativa combinada con un aislamiento de procesos eficiente**:

- **El núcleo se mantiene puro y rápido:** Escrito en un lenguaje de bajo nivel o compilado (como Rust), dedicado exclusivamente al renderizado de texto, la gestión del árbol de archivos y el análisis sintáctico.
    
- **Los plugins como microservicios o procesos desacoplados:** En lugar de meter el código del gestor de bases de datos o del entorno Android dentro del editor, el plugin actúa como un demonio o proceso hijo ligero que se comunica por IPC (Inter-Process Communication) o sockets locales, renderizando su interfaz gráfica de forma optimizada solo cuando el usuario abre esa pestaña específica.
    

Crear un editor así requiere desafiar el paradigma tradicional de separar drásticamente las herramientas de desarrollo, priorizando la ergonomía mental del programador por encima de los dogmas clásicos de la computación.
