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