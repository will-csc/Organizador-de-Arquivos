# Organizador de Arquivos

Este é um programa simples escrito em Rust que organiza arquivos em um diretório de origem (`src`) movendo-os para subdiretórios correspondentes à sua extensão em um diretório de destino (`dst`). O programa suporta arquivos com as seguintes extensões: `.txt`, `.pdf`, `.docx`, `.jpg`,  `.png` e demais extensões do excel.

<hr>

## Como Funciona

O programa faz o seguinte:

1. **Verifica o Diretório de Destino**: Se o diretório de destino não existir, ele é criado automaticamente.
    
2. **Lê os Arquivos no Diretório de Origem**: O programa lê todos os arquivos no diretório de origem.
    
3. **Classifica por Extensão**: Cada arquivo é movido para um subdiretório dentro do diretório de destino, correspondente à sua extensão. Por exemplo, arquivos `.txt` são movidos para um subdiretório chamado `txt`.
    
4. **Move os Arquivos**: Os arquivos são movidos para os subdiretórios apropriados.

<hr>

## Execução

1. **Compile o Programa**: Certifique-se de ter o Rust instalado. Em seguida, compile o programa usando o comando:
    
   ```bash
    cargo build --release
    ```
    
2. **Execute o Programa**: Após a compilação, execute o programa:
    
    ```bash
    cargo run --release
    ```
    
    O programa moverá os arquivos do diretório de origem (`src`) para o diretório de destino (`dst`), organizando-os por extensão.
    
3. **Personalize os Diretórios**: Se desejar, você pode alterar os diretórios de origem e destino diretamente no código-fonte:
    
    ```rust
    let src = "C:\\Users\\WILLIAM\\Downloads";
    let dst = "C:\\Users\\WILLIAM\\Downloads\\Organizados";
    ```
    
    Substitua os caminhos pelos diretórios que deseja usar.

## Estrutura do Código

- **Enum `FileExtension`**: Define as extensões de arquivo suportadas e fornece métodos para converter entre strings e o enum.
    
- **Função `mv_files`**: Realiza a leitura dos arquivos no diretório de origem e os move para os subdiretórios apropriados no diretório de destino.
    
- **Função `main`**: Define os diretórios de origem e destino e chama a função `mv_files`.

## Exemplo

Suponha que você tenha os seguintes arquivos no diretório `C:\Users\WILLIAM\Downloads`:

- `documento.pdf`
    
- `foto.jpg`
    
- `texto.txt`
    
- `planilha.docx`
    
- `imagem.png`
    

Após executar o programa, os arquivos serão organizados da seguinte forma:

```
C:\Users\WILLIAM\Downloads\Organizados\
    ├── pdf\
    │   └── documento.pdf
    ├── jpg\
    │   └── foto.jpg
    ├── txt\
    │   └── texto.txt
    ├── docx\
    │   └── planilha.docx
    └── png\
        └── imagem.png
```
