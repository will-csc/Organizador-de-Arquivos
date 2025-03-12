use std::fs::{self};
use std::path::Path;
use std::io;

enum FileExtension {
    Txt,
    Pdf,
    Docx,
    Jpg,
    Png,
}

impl FileExtension {
    fn as_str(&self) -> &str {
        match self {
            FileExtension::Txt => "txt",
            FileExtension::Pdf => "pdf",
            FileExtension::Docx => "docx",
            FileExtension::Jpg => "jpg",
            FileExtension::Png => "png",
            FileExtension::Xlsx => "xlsx",
            FileExtension::Xlsm => "xlsm",
            FileExtension::Xls => "xls",
        }
    }

    fn from_str(ext: &str) -> Option<FileExtension> {
        match ext {
            "txt" => Some(FileExtension::Txt),
            "pdf" => Some(FileExtension::Pdf),
            "docx" => Some(FileExtension::Docx),
            "jpg" => Some(FileExtension::Jpg),
            "png" => Some(FileExtension::Png),
            "xlsx" => Some(FileExtension::Xlsx),
            "xls" => Some(FileExtension::Xls),
            "xlsm" => Some(FileExtension::Xlsm),
            _ => None,
        }
    }
}

fn mv_files(src: &str, dst: &str) -> io::Result<()> {
    let path_src = Path::new(src);
    let path_dst = Path::new(dst);

    // Verifica se o diretório de dst existe, se não, cria
    if !path_dst.exists() {
        fs::create_dir_all(path_dst)?;
    }

    // Lê todos os arquivos do diretório de src
    for entry in fs::read_dir(path_src)? {
        let entry = entry?;
        
        if let Some(ext) = entry.path().extension().and_then(|ext| ext.to_str()) {
            if let Some(extensao) = FileExtension::from_str(ext) {
                let dst_dir = path_dst.join(extensao.as_str());

                if !dst_dir.exists() {
                    fs::create_dir_all(&dst_dir)?;
                }

                // Move o arquivo para o diretório de dst com o nome da extensão
                let dst_final = dst_dir.join(entry.file_name());
                fs::rename(entry.path(), dst_final)?;
            }
        }
    }

    Ok(())
}

fn main() {
    let src = "C:\\Users\\WILLIAM\\Downloads";
    let dst = "C:\\Users\\WILLIAM\\Downloads\\Organizados";
    
    match mv_files(src, dst) {
        Ok(_) => println!("Arquivos movidos com sucesso!"),
        Err(e) => eprintln!("Erro ao mover arquivos: {}", e),
    }
}
