# NitroCompiler

## Descripción del proyecto
> **NitroCompiler** es un compilador escrito en ***Rust*** para generar código optimizado para diferentes fines. Con una arquitectura modular cada fase del proceso  se organiza en módulos independientes creando la capacidad de escalar.

### Especificaciones Técnicas

- **Lenguaje Principal**: Rust
- **Arquitectura**: Modular
- **Librerías Utilizadas**:
  - `logos`: Para la tokenización eficiente.
  - `lalrpop`: Para el análisis sintáctico.
  - `cranelift`: Para la generación y optimización de código intermedio.
  - `log` y `env_logger`: Para manejar los logs y la depuración.

### Requisitos

- Rust (versión más reciente)
- Cargo (gestor de dependencias de Rust)

Dependencias de Cargo:

```toml
[dependencies]
logos = "0.12.0"
lalrpop = "0.19.10"
cranelift = "0.93.0"
log = "0.4.20"
env_logger = "0.10.0"
```

### **Instalación**
Instala y ejecuta el proyecto.

Para más detalles, consulta la [documentación completa](./docs/introduction.md).
