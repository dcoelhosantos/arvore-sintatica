pub enum OperadorUnario {
    Negacao,
}
impl OperadorUnario {
    fn simbolo(&self) -> char {
        match self {
            OperadorUnario::Negacao => '-',
        }
    }
    fn precedencia(&self) -> u8 {
        match self {
            OperadorUnario::Negacao => 2,
        }
    }
}

pub enum OperadorBinario {
    Soma,
    Subtracao,
    Multiplicacao,
    Divisao,
    Resto   
}
impl OperadorBinario {
    fn simbolo(&self) -> char {
        match self {
            OperadorBinario::Soma => '+',
            OperadorBinario::Subtracao => '-',
            OperadorBinario::Multiplicacao => '*',
            OperadorBinario::Divisao => '/',
            OperadorBinario::Resto => '%',
        }
    }
    fn precedencia(&self) -> u8 {
        match self {
            OperadorBinario::Soma | 
            OperadorBinario::Subtracao => 
            0,
            OperadorBinario::Multiplicacao | 
            OperadorBinario::Divisao | 
            OperadorBinario::Resto => 
            1,
        }
    }
}

pub enum Expressao {
    Numero(i64),
    OperacaoBinaria(Box<Expressao>, OperadorBinario, Box<Expressao>),
    OperacaoUnaria(OperadorUnario, Box<Expressao>)
}

impl Expressao {
    pub fn avaliar(&self) -> Option<i64> {
        match self {
            Self::Numero(num) => Some(*num),
            Self::OperacaoBinaria(e1, op, e2) => {
                let a = e1.avaliar()?;
                let b = e2.avaliar()?;
                match op {
                    OperadorBinario::Soma => a.checked_add(b),
                    OperadorBinario::Subtracao => a.checked_sub(b),
                    OperadorBinario::Multiplicacao => a.checked_mul(b),
                    OperadorBinario::Divisao => {
                        if b == 0 { None } else { a.checked_div(b) }
                    }
                    OperadorBinario::Resto => {
                        if b == 0 { None } else { a.checked_rem(b) }
                    }
                }
            },
            Self::OperacaoUnaria(op, e) => {
                match op {
                    OperadorUnario::Negacao => Some(-e.avaliar()?)
                }
            }
        }
    }

    pub fn imprimir(&self) {
        self.imprimir_com_precedencia(0);
        println!();
    }

    fn imprimir_com_precedencia(&self, precedencia_pai:u8){
        match self {
            Self::Numero(num) => print!("{}", num),
            Self::OperacaoBinaria(e1, op , e2) => {
                let precedencia_atual = op.precedencia();
                if precedencia_atual < precedencia_pai {
                    print!("(");
                }
                e1.imprimir_com_precedencia(precedencia_atual);
                print!(" {} ", op.simbolo());
                e2.imprimir_com_precedencia(precedencia_atual);

                if precedencia_atual < precedencia_pai {
                    print!(")");
                }
            },
            Self::OperacaoUnaria(op, e) => {
                match op {
                    OperadorUnario::Negacao => print!("{}", op.simbolo()),
                }
                e.imprimir_com_precedencia(op.precedencia());
            }
        }
    }

    pub fn _imprimir_arvore(&self) {}
}