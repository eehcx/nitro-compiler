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

Para más detalles, consulta la [documentación completa](./docs/introduction.md).
