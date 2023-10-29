
pub fn root<O: Clone, H:Fn(&[u8]) -> O>(hasher: H, leaves: &[&[u8]]) -> O where Vec<u8>: From<O> {

    if leaves.is_empty() {

        hasher(&[])

    } else {

        let mut level: Vec<O> = leaves
            .iter()
            .map(|&x| hasher(x))
            .collect();
            
        while level.len() > 1 {

            level = climb(&hasher, level);

        }
        
        level[0].clone()

    }

}

fn climb<O: Clone, H:Fn(&[u8]) -> O>(hasher: H, level: Vec<O>) -> Vec<O> where Vec<u8>: From<O> {

    let t = level.len() / 2;

    let mut res: Vec<O> = (0..t)
        .into_iter()
        .map(|x| {

            let i = x * 2;

            let a_bytes: Vec<u8> = level[i].clone().into();

            let b_bytes: Vec<u8> = level[i + 1].clone().into();

            let concatenated = [a_bytes, b_bytes].concat();

            hasher(&concatenated)

        })
        .collect();

    let r = level.len() % 2;

        if r != 0 {
    
            let i = level.len() - 1;
    
            let l_bytes: Vec<u8> = level[i].clone().into();
    
            res.push(hasher(&l_bytes))
            
        }
    
    res

}
