# 🌳 Avaliador e Impressão de Árvores de Expressões em Rust

[![Feito com Rust](https://img.shields.io/badge/Feito%20com-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

Este projeto implementa um **avaliador de expressões aritméticas** em Rust, com suporte a **operações unárias e binárias**, **precedência de operadores** e uma **impressão visual em formato de árvore**.  

---

## 🚀 Características principais

- **Avaliação de expressões** com operadores `+`, `-`, `*`, `÷` e `%`.  
- **Suporte a negação unária** (`-10`, `-(5 + 3)` etc).  
- **Respeito à precedência** e **uso de parênteses**.  
- **Impressão da árvore sintática** com estrutura hierárquica visual.  
- **Verificação de overflow** e **divisão por zero** segura usando `Option<i64>`.  

---

## ⚙️ Como executar

1. **Clone o repositório:**
   ```sh
   git clone https://github.com/seuusuario/avaliador-arvore-rust.git
   ```

2. **Navegue até o diretório do projeto:**
   ```sh
   cd avaliador-arvore-rust
   ```

3. **Execute com Cargo:**
   ```sh
   cargo run
   ```

---

## 🧠 Exemplos rápidos

**Avaliação**
```
10 + 2 * 3 → Some(16)
-(10 + 20) * 2 → Some(-60)
10 ÷ 0 → None
```

**Árvore (para `(10 + 20) * 30`)**
```
*
├─ +
│  ├─ 10
│  └─ 20
└─ 30
```

---

## 🌿 Exemplo de uso (trecho de código)

```rust
fn main() {
    use crate::{Expressao, OperadorBinario, OperadorUnario};

    let exp = Expressao::OperacaoBinaria(
        Box::new(Expressao::OperacaoBinaria(
            Box::new(Expressao::Numero(10)),
            OperadorBinario::Soma,
            Box::new(Expressao::Numero(20)),
        )),
        OperadorBinario::Multiplicacao,
        Box::new(Expressao::Numero(30)),
    );

    exp.imprimir();          // (10 + 20) * 30
    exp.imprimir_arvore();   // imprime a árvore
    println!("Resultado: {:?}", exp.avaliar());
}
```

---

## 📚 Estrutura geral

- **`OperadorUnario` e `OperadorBinario`** – enums que definem símbolos e precedências.  
- **`Expressao`** – enum que representa números, operações unárias e binárias.  
- **Métodos principais:**  
  - `avaliar()` → calcula o resultado da expressão.  
  - `imprimir()` → exibe a expressão em notação infixa.  
  - `imprimir_arvore()` → mostra a estrutura hierárquica.

