fn exibir_dados_usuario() {
    let idade: i32 = 42;
    let pontos = 100;
    println!("Idade: {}, Pontos acumulados: {}", idade, pontos);
}

fn calcular_financas() {
    let receita = 10_000;
    let despesas = 3_000;

    println!("Lucro: {}", receita - despesas);
    println!("Dobro da receita: {}", receita * 2);
    println!("Quociente: {}", receita / despesas);
    println!("Resto da divisão: {}", receita % despesas);
}

fn comparar_metas() {
    let meta_atingida = 5 + 5;
    let meta_esperada = 10;
    println!("Meta atingida: {} = Meta esperada: {} ? {}", meta_atingida, meta_esperada, meta_atingida == meta_esperada);
}

fn linha() {
    println!("----------------------------")
}

fn main() {
    linha();
    exibir_dados_usuario();
    linha();
    calcular_financas();
    linha();
    comparar_metas();
    linha();
}
