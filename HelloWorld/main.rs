fn main() { //Asi se definen funciones en RUST, ademas, esta es especial, todo documento rs comienza con una funcion main().
    println!("Hello world, it's Matthew"); //Println se usa para imprimir en pantalla, usar el ! hace que se usen macros, sino llamaria como funcion. Ademas, lo que va dentro de () es lo que se imprime y siempre se cierra al final con un ;.
} //Esto cierra la funcion, todo lo que este dentro de estos dos forma parte de main().
//PASOS PARA CORRERLO:
//1- en el shell se corre: cd Projects y luego cd HelloWorld, asi hasta llegar al archivo como tal, de carpeta en carpeta hasta el archivo.
//2- Luego se corre rustc main.rs o el nombre del archivo a correr.
//3- Para terminar se corre .\main.exe o el nombre del archivo.
// NOTA: Todo esto en el WindowsShell.