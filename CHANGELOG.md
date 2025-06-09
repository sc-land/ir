# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-06-08

### Added
- Sistema completo de representação intermediária (IR)
- Estrutura `Larvie` para modelagem de entidades
- Sistema de tipos `Flora` (Int, Str, Bool, Bug)
- Sistema de restrições `Seal` (Vital, Core, Root)
- Estrutura `Casts` para propriedades de campos
- Estrutura `Instinct` para modelagem comportamental
- Conversão automática de AST SC-DSL para IR
- Suporte completo a serialização JSON via Serde
- Documentação UML completa
- Cobertura abrangente de testes unitários

### Changed
- Migração de representação simples para sistema estruturado completo

### Technical Details
- Implementação de conversões `Bug` → `Larvie`
- Implementação de conversões `Gene` → `Casts`
- Implementação de conversões `Ethics` → `Instinct`
- Mapeamento automático de tipos `Specie` → `Flora`
- Integração completa com `sc-dsl` parser

## [0.1.0] - 2025-06-05

### Added
- Projeto inicial do IR
- Estrutura básica de módulos
- API pública `sc_tree_to_ir`
- Dependências base: sc-dsl, serde

### Infrastructure
- Configuração Cargo.toml
- Estrutura de módulos organizada
- Pipeline de testes básico
