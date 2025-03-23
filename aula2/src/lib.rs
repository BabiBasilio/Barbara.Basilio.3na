/// Multiplica todos os elementos de um slice de inteiros.
/// Agora usa slice ao invés de ponteiros e remove o `unsafe`.
pub fn multiply_array(array: &[i32]) -> i32 {
    array.iter().fold(1, |acc, &x| acc * x) // Usa `fold` pra multiplicar tudo de forma segura
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let array = [2, 3, 4];
        
        // Agora a função recebe um slice direto, sem precisar de `unsafe`.
        assert_eq!(multiply_array(&array), 24);
    }
}
