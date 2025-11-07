mod expressao;
use expressao::Expressao;
use expressao::OperadorBinario;
use expressao::OperadorUnario;

fn main() {
    use Expressao::*;
    use OperadorBinario::*;
    use OperadorUnario::*;

    // exp1 - número simples
    let exp1 = Numero(42);
    println!("");
    exp1.imprimir();
    println!("Resultado: {:?}", exp1.avaliar());
    println!("Árvore binária:");
    exp1.imprimir_arvore();

    // exp2 - soma simples
    let exp2 = OperacaoBinaria(
        Box::new(Numero(5)),
        Soma,
        Box::new(Numero(7)),
    );
    println!("");
    exp2.imprimir();
    println!("Resultado: {:?}", exp2.avaliar());
    println!("Árvore binária:");
    exp2.imprimir_arvore();

    // exp3 - subtração com negativos
    let exp3 = OperacaoBinaria(
        Box::new(Numero(-5)),
        Subtracao,
        Box::new(Numero(3)),
    );
    println!("");
    exp3.imprimir();
    println!("Resultado: {:?}", exp3.avaliar());
    println!("Árvore binária:");
    exp3.imprimir_arvore();

    // exp4 - multiplicação simples
    let exp4 = OperacaoBinaria(
        Box::new(Numero(4)),
        Multiplicacao,
        Box::new(Numero(6)),
    );
    println!("");
    exp4.imprimir();
    println!("Resultado: {:?}", exp4.avaliar());
    println!("Árvore binária:");
    exp4.imprimir_arvore();

    // exp5 - divisão simples
    let exp5 = OperacaoBinaria(
        Box::new(Numero(20)),
        Divisao,
        Box::new(Numero(5)),
    );
    println!("");
    exp5.imprimir();
    println!("Resultado: {:?}", exp5.avaliar());
    println!("Árvore binária:");
    exp5.imprimir_arvore();

    // exp6 - resto (módulo)
    let exp6 = OperacaoBinaria(
        Box::new(Numero(17)),
        Resto,
        Box::new(Numero(5)),
    );
    println!("");
    exp6.imprimir();
    println!("Resultado: {:?}", exp6.avaliar());
    println!("Árvore binária:");
    exp6.imprimir_arvore();

    // exp7 - operação composta com precedência (10 + 2 * 3)
    let exp7 = OperacaoBinaria(
        Box::new(Numero(10)),
        Soma,
        Box::new(OperacaoBinaria(
            Box::new(Numero(2)),
            Multiplicacao,
            Box::new(Numero(3)),
        )),
    );
    println!("");
    exp7.imprimir();
    println!("Resultado: {:?}", exp7.avaliar());
    println!("Árvore binária:");
    exp7.imprimir_arvore();

    // exp8 - operação unária simples (-10)
    let exp8 = OperacaoUnaria(
        Negacao,
        Box::new(Numero(10)),
    );
    println!("");
    exp8.imprimir();
    println!("Resultado: {:?}", exp8.avaliar());
    println!("Árvore binária:");
    exp8.imprimir_arvore();

    // exp9 - unária + binária misturada (-(10 + 20) * 2)
    let exp9 = OperacaoBinaria(
        Box::new(OperacaoUnaria(
            Negacao,
            Box::new(OperacaoBinaria(
                Box::new(Numero(10)),
                Soma,
                Box::new(Numero(20)),
            )),
        )),
        Multiplicacao,
        Box::new(Numero(2)),
    );
    println!("");
    exp9.imprimir();
    println!("Resultado: {:?}", exp9.avaliar());
    println!("Árvore binária:");
    exp9.imprimir_arvore();

    // exp10 - expressão complexa ((5 + 3) * (2 + (4 * 2)))
    let exp10 = OperacaoBinaria(
        Box::new(OperacaoBinaria(
            Box::new(Numero(5)),
            Soma,
            Box::new(Numero(3)),
        )),
        Multiplicacao,
        Box::new(OperacaoBinaria(
            Box::new(Numero(2)),
            Soma,
            Box::new(OperacaoBinaria(
                Box::new(Numero(4)),
                Multiplicacao,
                Box::new(Numero(2)),
            )),
        )),
    );
    println!("");
    exp10.imprimir();
    println!("Resultado: {:?}", exp10.avaliar());
    println!("Árvore binária:");
    exp10.imprimir_arvore();

    // exp11 - divisão por zero (erro esperado)
    let exp11 = OperacaoBinaria(
        Box::new(Numero(10)),
        Divisao,
        Box::new(Numero(0)),
    );
    println!("");
    exp11.imprimir();
    println!("Resultado: {:?}", exp11.avaliar());
    println!("Árvore binária:");
    exp11.imprimir_arvore();

    // exp12 - expressão complexa ((-(10 + 20) + 30 + 40 + (50 + 60)) * -5)
    let exp12 = OperacaoBinaria(
        Box::new(OperacaoBinaria(
            Box::new(OperacaoBinaria(
                Box::new(OperacaoBinaria(
                    Box::new(OperacaoUnaria(
                        Negacao,
                        Box::new(OperacaoBinaria(
                            Box::new(Numero(10)),
                            Soma,
                            Box::new(Numero(20)),
                        )),
                    )),
                    Soma,
                    Box::new(Numero(30)),
                )),
                Soma,
                Box::new(Numero(40)),
            )),
            Soma,
            Box::new(OperacaoBinaria(
                Box::new(Numero(50)),
                Soma,
                Box::new(Numero(60)),
            )),
        )),
        Multiplicacao,
        Box::new(OperacaoUnaria(
            Negacao,
            Box::new(Numero(5)))),
    );

    println!("");
    println!("// exp12");
    exp12.imprimir();
    println!("Resultado: {:?}", exp12.avaliar());
    println!("Árvore binária:");
    exp12.imprimir_arvore();
}