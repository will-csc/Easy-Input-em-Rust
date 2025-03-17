# Facilitando Input do Usuário em Rust

Este projeto contém uma função para ler entradas do usuário em Rust de forma simples e reutilizável. Ele utiliza um módulo personalizado `read` que contém a função `input`, que permite ao usuário fornecer entradas de tipos específicos, como `i32` e `String`.

![image](https://github.com/user-attachments/assets/6bf22264-79d3-44bc-89ea-2ba155334097)

## Estrutura do Projeto

- **`main.rs`**: Contém a função `main` que solicita entradas ao usuário e exibe as respostas.
- **`read.rs`**: Contém a função `input` que lê uma linha do usuário e a converte para o tipo desejado.

## Como Funciona

1. O usuário é solicitado a inserir um valor, como um número ou um nome.
2. A função `input` no módulo `read` lê a entrada do usuário.
3. O valor inserido é convertido para o tipo desejado (por exemplo, `i32` ou `String`).
4. O valor é então exibido de volta ao usuário.

## Como Usar

### Passo 1: Estrutura de Arquivos

O código é organizado da seguinte maneira:

src/ 
├── main.rs
|── read.rs


### Passo 2: Adicionar o Módulo `read`

Em `main.rs` ou no seu código, o módulo `read` é importado com:

```rust
mod read;
```

### Passo 3: Reexportar a Função input
Dentro de main.rs, a função input do módulo read é reexportada para que possa ser chamada diretamente:
```Rust
pub use read::input;
```

Agora você já pode utilizar input diretamente em main.rs:

### Regras da função
 Para função funcionar, a declaração da variável deve ser tipada, contendo o seu tipo de dado. Tambeém, deve haver uma mensagem para input.

Exemplo:
```Rust
let num: i32 = input("Digite um número: ");
```
