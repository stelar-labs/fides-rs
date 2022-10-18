
pub fn root<L,H>(leaves: &[&L]) -> H

    where
    
        L: Into<H> + Clone,

        H: Into<Vec<u8>> + From<Vec<u8>> + From<L> + Clone

            {

                if leaves.is_empty() {

                    vec![].into()

                } else {

                    let mut hashes: Vec<H> = leaves
                        .iter()
                        .map(|&x| x.clone().into())
                        .collect();
                    
                    while hashes.len() > 1 {

                        let mut next = vec![];

                        let mut intermediate = vec![];

                        for h in hashes {
                            
                            intermediate.push(h);
                            
                            if intermediate.len() == 2 {

                                let join = intermediate
                                    .iter()
                                    .fold(
                                        vec![], |acc, x| {

                                            let x_bytes: Vec<u8> = x.clone().into();
                                            
                                            [acc, x_bytes].concat()
                                        }
                                    );

                                let join_hash: H = join.into();
                                    
                                next.push(join_hash);

                                intermediate.clear()

                            }

                        }

                        if !intermediate.is_empty() {
                            next.push(intermediate[0].clone());
                        }

                        hashes = next
                        
                    }

                    hashes[0].clone()

                }

}
