# Introducción

### Fases principales del compilador
1. Análisis Léxico (Lexer)
2. Análisis Sintáctico (Parser)
3. Análisis Semántico
4. Generación de Código Intermedio (IR)
5. Optimización del Código
6. Generación de Código Final
7. Enlazado y Ensamblaje

## Arquitectura

NitroCompiler sigue una arquitectura modular, donde cada componente del proceso de compilación está separado en módulos:

- `lexer/`: Módulo para el análisis léxico.
- `parser/`: Módulo para la generación del árbol sintáctico abstracto (AST).
- `semantic/`: Módulo para el análisis semántico.
- `ir/`: Generación de código intermedio.
- `codegen/`: Generación de código para múltiples arquitecturas.
- `optimization/`: Optimización del código intermedio.
- `assembler/`: Enlazado y ensamblaje.

Cada módulo es independiente y puede ser extendido sin afectar los demás.
