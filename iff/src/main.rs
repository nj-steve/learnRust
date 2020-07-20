fn main() {
    for node in 0..=10{
        let hash = if node > 3 {
            node * 2
        } else {
            node -1
        };
        println!("hhhhh = {}",hash);
    }

    println!("Hello, world!");
    let mut num = 5;
    let num_ref = &mut num;
    *num_ref = 100;
    println!("{} sizeof &i32 {}", num, std::mem::size_of::<&i32>());

    let mut b = 1;
    let mut bp = &mut b;
    println!("in main bp's addr:{:p}", bp);
    //println!("in main b's value:{}", b);
    pointer_addr(bp);
    println!("in main pointer_addr bp's addr:{}", *bp);
    //println!("in main b's value:{}", b);
    pointer_addr1(bp);
    println!("in main pointer_addr1 bp's addr:{}", *bp);
    println!("in main b's value:{}", b);
}

fn pointer_addr(p: &mut i32) -> &i32 {
    println!("in pointer_addr p's addr:{:p}", p);
    *p = 100;
    p
}

fn pointer_addr1(p: &mut i32) -> &i32 {
    println!("in pointer_addr1 p's addr:{:p}", p);
    *p = 200;
    p
}
/*
use rayon::prelude::*;
let retproof: Vec<_> = inputs
.par_iter()
.map(|(pub_inputs, priv_inputs)| {
let provebr_time = SystemTime::now();
info!("generate_post pub_inputs id {:?}", pub_inputs.sector_id);
let proof =
ElectionPoStCompound::prove(&pub_params, &pub_inputs, &priv_inputs, &groth_params)?;
//proofs.push(proof.to_vec()?);
let proveafyer_time = SystemTime::now();
let difference = proveafyer_time.duration_since(provebr_time)
.expect("Clock may have gone backwards");
info!("generate_post replica prove proveafyer_time difference {:?} sectorid = {:?}", difference, pub_inputs.sector_id);
Ok(proof.to_vec()?)
})
.collect::<Result<_>>()?;

/*for (pub_inputs, priv_inputs) in &inputs {
    info!("generate_post pub_inputs id {:?}", pub_inputs.sector_id);
    let proof =
        ElectionPoStCompound::prove(&pub_params, &pub_inputs, &priv_inputs, &groth_params)?;
    proofs.push(proof.to_vec()?);
}*/

let proof_time = SystemTime::now();
let difference = proof_time.duration_since(merkle_tree_time)
.expect("Clock may have gone backwards");
info!("generate_post replica proof_time difference {:?}", difference);

info!("generate_post:finish");

//Ok(proofs)
Ok(retproof)
*/

/*
pub_sectors_chunk
.iter()
.zip(priv_sectors_chunk.iter())
.enumerate()
.par_iter()
.map(|(i,(pub_sector, priv_sector))|
{
let tree = priv_sector.tree;
let sector_id = pub_sector.id;
let tree_leafs = tree.leafs();

trace!(
"Generating proof for tree leafs {} and arity {}",
tree_leafs,
Tree::Arity::to_usize(),
);

let inclusion_proofs = (0..pub_params.challenge_count)
.into_par_iter()
.map(|n| {
let challenge_index = ((j * num_sectors_per_chunk + i)
* pub_params.challenge_count
+ n) as u64;
let challenged_leaf_start = generate_leaf_challenge(
pub_params,
pub_inputs.randomness,
sector_id.into(),
challenge_index,
)?;

tree.gen_cached_proof(challenged_leaf_start as usize, None)
})
.collect::<Result<Vec<_>>>()?;

proofs.push(SectorProof {
inclusion_proofs,
comm_c: priv_sector.comm_c,
comm_r_last: priv_sector.comm_r_last,
});
Ok("")
});
*/