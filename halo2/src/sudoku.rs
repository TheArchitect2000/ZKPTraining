use std::marker::PhantomData;
use pasta_curves::group::ff::PrimeField;
use rand_core::OsRng;
use halo2_proofs::{
    circuit::*, 
    plonk::*,
    dev::MockProver,
    plonk::Assigned, 
    pasta::*, 
    transcript::*, 
    poly::Rotation, 
    poly::commitment::Params};

// Sudoku Halo2 Circuit
// This circuit verifies a Sudoku solution.
// The circuit is designed to be able to verify a Sudoku solution.

#[derive(Debug, Clone)]
pub struct SudokuConfig {
    pub col_advice_1: Column<Advice>, 
    pub col_advice_2: Column<Advice>,
    pub col_advice_3: Column<Advice>,
    pub col_selector_1: Selector,
    pub col_instance_1: Column<Instance>,
}

// The chip which holds the circuit config
#[derive(Debug, Clone)]
struct SudokuChip<F> {
    config: SudokuConfig,
    _marker: PhantomData<F>,
}

impl<F> SudokuChip<F> {
    pub fn construct(config: &SudokuConfig) -> Self {
        Self {
            config: config.clone(),
            _marker: PhantomData,
        }
    }

    pub fn configure(meta: &mut ConstraintSystem<Fp>) -> SudokuConfig {
        let col_advice_1 = meta.advice_column();
        let col_advice_2 = meta.advice_column();
        let col_advice_3 = meta.advice_column();
        let col_selector_1 = meta.selector();
        let col_instance_1 = meta.instance_column();

        meta.enable_equality(col_advice_1);
        meta.enable_equality(col_advice_2);
        meta.enable_equality(col_advice_3);
        meta.enable_equality(col_instance_1);

        meta.create_gate("add", |meta| {
            //
            // col_advice_1 | col_advice_2 | col_advice_3 | col_selector_1
            //       a               b             c                 s
            //
            let s = meta.query_selector(col_selector_1);
            let a = meta.query_advice(col_advice_1, Rotation::cur());
            let b = meta.query_advice(col_advice_2, Rotation::cur());
            let c = meta.query_advice(col_advice_3, Rotation::cur());
            vec![s * (a + b - c)]
        });

        SudokuConfig {
            col_advice_1,
            col_advice_2,
            col_advice_3,
            col_selector_1,
            col_instance_1,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn add_instruction(
        &self,
        mut layouter: impl Layouter<Fp>,
        col_advice_1: &AssignedCell<Fp, Fp>,
        col_advice_2: &AssignedCell<Fp, Fp>,
    ) -> Result<AssignedCell<Fp, Fp>, Error> {
        layouter.assign_region(
            || "Next Row",
            |mut region| {
                self.config.col_selector_1.enable(&mut region, 0)?;

                // Copy the value from b & c in previous row to a & b in current row
                col_advice_1.copy_advice(
                    || "col_advice_1",
                    &mut region,
                    self.config.col_advice_1,
                    0,
                )?;
                col_advice_2.copy_advice(
                    || "col_advice_2",
                    &mut region,
                    self.config.col_advice_2,
                    0,
                )?;

                let col3_row_any_except_0 = region.assign_advice(
                    || "col_advice_3",
                    self.config.col_advice_3,
                    0,
                    || col_advice_1.value().copied() + col_advice_2.value(),
                )?;

                Ok(col3_row_any_except_0)

            },
        )
    }

}

// The configuration of the circuit
#[derive(Default, Debug, Clone)]
pub struct MyCircuit {
    pub sudoku: [[u8; 9]; 9],
}

impl Circuit<Fp> for MyCircuit {
    type Config = SudokuConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        SudokuChip::<Fp>::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> 
    where 
        for<'v> Assigned<Fp>: From<&'v Fp> 
    {
         let chip: SudokuChip<Fp> = SudokuChip::construct(&config);

        let vars: Vec<Vec<AssignedCell<_, _>>> = self
            .sudoku
            .iter()
            .map(|line| {
                let u: Vec<AssignedCell<_, _>> = line
                    .iter()
                    .map(|x| {
                        layouter.assign_region(
                            || "sudoku_cell",
                            |mut region
                            | region.assign_advice(
                                || "sudoku_cell",
                                 config.col_advice_1, 
                                 0,
                                 || Value::known(pallas::Base::from_u128(*x as u128))),
                        ).unwrap()
                    })
                    .collect();
                u
            })
            .collect();


        let mut cpt = 0;
        for i in 0..9 {
            let mut line = 
            layouter.assign_region(
                || "lhs init",
                |mut region
                | region.assign_advice(
                    || "lhs init",
                     config.col_advice_1,
                      0, 
                      || Value::known(Fp::zero())),
            ).unwrap();

            let mut col = 
            layouter.assign_region(
                || "lhs init",
                |mut region
                | region.assign_advice(
                    || "lhs init", 
                    config.col_advice_1, 
                    0, 
                    || Value::known(Fp::zero())),
            ).unwrap();

            let mut sub = 
            layouter.assign_region(
                || "lhs init",
                |mut region
                | region.assign_advice(
                    || "lhs init", 
                    config.col_advice_1,
                     0,
                     || Value::known(Fp::zero())),
            ).unwrap();

            for j in 0..9 {
                line = chip.add_instruction(
                    layouter.namespace(|| "line + vars[i][j]"),
                    &line,
                    &vars[i][j],
                )
                .unwrap();

                col = chip.add_instruction(
                    layouter.namespace(|| "col + vars[j][i]"),
                    &col,
                    &vars[j][i],
                )
                .unwrap();

                sub = chip.add_instruction(
                    layouter.namespace(|| "sub + vars[((i / 3) * 3) + j / 3][i % 3 * 3 + j % 3]"),
                    &sub,
                    &vars[((i / 3) * 3) + j / 3][i % 3 * 3 + j % 3],
                )
                .unwrap();
            }

            layouter
                .constrain_instance(line.cell(), config.col_instance_1, cpt)
                .unwrap();
            cpt += 1;

            layouter
                .constrain_instance(col.cell(), config.col_instance_1, cpt)
                .unwrap();
            cpt += 1;

            layouter
                .constrain_instance(sub.cell(), config.col_instance_1, cpt)
                .unwrap();
            cpt += 1;
        }

        Ok(())
    
    }
}

fn main() {
    let k = 9;

    let sudoku = [
        [7, 6, 9, 5, 3, 8, 1, 2, 4],
        [2, 4, 3, 7, 1, 9, 6, 5, 8],
        [8, 5, 1, 4, 6, 2, 9, 7, 3],
        [4, 8, 6, 9, 7, 5, 3, 1, 2],
        [5, 3, 7, 6, 2, 1, 4, 8, 9],
        [1, 9, 2, 8, 4, 3, 7, 6, 5],
        [6, 1, 8, 3, 5, 4, 2, 9, 7],
        [9, 7, 4, 2, 8, 6, 5, 3, 1],
        [3, 2, 5, 1, 9, 7, 8, 4, 6],
    ];

    let circuit = MyCircuit{sudoku};
    let public_input = vec![Fp::from(45); 27];

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();
    prover.assert_satisfied();
    println!("MockProver is done.");

    // create proof
    let params_tmp: Params<EqAffine> = halo2_proofs::poly::commitment::Params::<EqAffine>::new(k);
    let params = &params_tmp;
    let vk = &keygen_vk(&params, &circuit).expect("keygen_vk should not fail");
    let pk = &keygen_pk(&params, vk.clone(), &circuit).expect("keygen_pk should not fail");
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    let public_input_prover = &public_input;

    let status = create_proof(
        params,
        pk,
        &[circuit.clone()],
        &[&[public_input_prover]],
        OsRng,
        &mut transcript,
    ).is_ok();

    if status {
        println!("Proof generated!");
    } else {
        println!("Proof failed to generate!");    
    }     
    let proof: Vec<u8> = transcript.finalize();

    // verify proof
    let strategy = SingleVerifier::new(&params);
    let mut transcript_verify = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    let public_input_verify = &public_input;

    let status = verify_proof(
        params,
        vk,
        strategy,
        &[&[public_input_verify]],
        &mut transcript_verify,
    ).is_ok();

    if status {
        println!("Proof verified!");
    } else {
        println!("Proof failed to verify!");    
    }  

    // Draws the layout of the circuit. 
    use plotters::prelude::*;

    let root = BitMapBackend::new("sudoku-layout.png", (1024, 3096)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.titled("Sudoku Layout", ("sans-serif", 60)).unwrap();

    let circuit = MyCircuit{sudoku};
    halo2_proofs::dev::CircuitLayout::default()
        .render(9, &circuit, &root)
        .unwrap();

}
