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
    exp1.imprimir();
    println!("Resultado: {:?}\n", exp1.avaliar());

    // exp2 - soma simples
    let exp2 = OperacaoBinaria(
        Box::new(Numero(5)),
        Soma,
        Box::new(Numero(7)),
    );
    exp2.imprimir();
    println!("Resultado: {:?}\n", exp2.avaliar());

    // exp3 - subtração com negativos
    let exp3 = OperacaoBinaria(
        Box::new(Numero(-5)),
        Subtracao,
        Box::new(Numero(3)),
    );
    exp3.imprimir();
    println!("Resultado: {:?}\n", exp3.avaliar());

    // exp4 - multiplicação simples
    let exp4 = OperacaoBinaria(
        Box::new(Numero(4)),
        Multiplicacao,
        Box::new(Numero(6)),
    );
    exp4.imprimir();
    println!("Resultado: {:?}\n", exp4.avaliar());

    // exp5 - divisão simples
    let exp5 = OperacaoBinaria(
        Box::new(Numero(20)),
        Divisao,
        Box::new(Numero(5)),
    );
    exp5.imprimir();
    println!("Resultado: {:?}\n", exp5.avaliar());

    // exp6 - resto (módulo)
    let exp6 = OperacaoBinaria(
        Box::new(Numero(17)),
        Resto,
        Box::new(Numero(5)),
    );
    exp6.imprimir();
    println!("Resultado: {:?}\n", exp6.avaliar());

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
    exp7.imprimir();
    println!("Resultado: {:?}\n", exp7.avaliar());

    // exp8 - operação unária simples (-10)
    let exp8 = OperacaoUnaria(
        Negacao,
        Box::new(Numero(10)),
    );
    exp8.imprimir();
    println!("Resultado: {:?}\n", exp8.avaliar());

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
    exp9.imprimir();
    println!("Resultado: {:?}\n", exp9.avaliar());

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
    exp10.imprimir();
    println!("Resultado: {:?}\n", exp10.avaliar());

    // exp11 - divisão por zero (erro esperado)
    let exp11 = OperacaoBinaria(
        Box::new(Numero(10)),
        Divisao,
        Box::new(Numero(0)),
    );
    exp11.imprimir();
    println!("Resultado: {:?}\n", exp11.avaliar());
}