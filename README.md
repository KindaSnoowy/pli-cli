# Pills Line Interface (PLI)

**Pills Line Interface (PLI)** é um programa de linha de comando (CLI) simples que busca um padrão dentro de um arquivo de texto.

## Uso

Execute o comando:
```sh
pli <padrão> <arquivo>
```
Isso fará com que o programa procure pelo padrão especificado dentro do arquivo e exiba todas as ocorrências encontradas.

### Exemplo de saída

```sh
$ pli pills text.txt
Looking for: "pills" in "text.txt"
Pattern found at [2:27]
Pattern found at [3:7]
Pattern found at [6:12]
Pattern found at [8:12]
Pattern found at [9:12]
```

## Objetivo do projeto

Este projeto foi criado como um exercício de aprendizado para explorar a linguagem Rust e conceitos de manipulação de strings. Embora não tenha sido projetado para ser a solução mais eficiente, ele pode ser útil em algumas situações e serve como um marco no meu progresso com Rust.
