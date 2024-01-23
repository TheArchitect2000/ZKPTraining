use std::marker::PhantomData;
use rand_core::OsRng;
use halo2_proofs::{
    arithmetic::*, 
    circuit::*, 
    plonk::*, 
    dev::*, pasta::*, 
    transcript::*, 
    poly::Rotation, 
    poly::commitment::Params};

// Fibonacci Halo2 Circuit
// This circuit calculates the 10th Fibonacci number
// The first two numbers are 1, 1
// The next number is the sum of the previous two numbers
// The 10th Fibonacci number is 55
// The circuit is designed to be able to verify the 10th Fibonacci number is 55
// The circuit is designed to be able to verify the 10th Fibonacci number is 55 if the 2nd Fibonacci number is 7

#[derive(Debug, Clone)]
struct FibonacciConfig {
    pub col_advice_1: Column<Advice>,
    pub col_advice_2: Column<Advice>,
    pub col_advice_3: Column<Advice>,
    pub col_selector_1: Selector,
    pub col_instance_1: Column<Instance>,
}

// The chip which holds the circuit config
#[derive(Debug, Clone)]
struct FibonacciChip<F: Field> {
    config: FibonacciConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> FibonacciChip<F> {
    pub fn construct(config: &FibonacciConfig) -> Self {
        Self {
            config: config.clone(),
            _marker: PhantomData,
        }
    }

    pub fn configure(meta: &mut ConstraintSystem<F>) -> FibonacciConfig {
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

        FibonacciConfig {
            col_advice_1,
            col_advice_2,
            col_advice_3,
            col_selector_1,
            col_instance_1,
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn assign_first_row(
        &self,
        mut layouter: impl Layouter<F>,
    ) -> Result<(AssignedCell<F, F>, AssignedCell<F, F>, AssignedCell<F, F>), Error> {
        layouter.assign_region(
            || "First Row",
            |mut region| {
                self.config.col_selector_1.enable(&mut region, 0)?;

                let col1_row0 = region.assign_advice_from_instance(
                    || "F(0)",
                    self.config.col_instance_1,
                    0,
                    self.config.col_advice_1,
                    0)?;

                let col2_row0 = region.assign_advice_from_instance(
                    || "F(1)",
                    self.config.col_instance_1,
                    1,
                    self.config.col_advice_2,
                    0)?;

                let col3_row0 = region.assign_advice(
                    || "F(0) + F(1)",
                    self.config.col_advice_3,
                    0,
                    || col1_row0.value().copied() + col2_row0.value(),
                )?;

                Ok((col1_row0, col2_row0, col3_row0))
            },
        )
    }

    pub fn assign_row(
        &self,
        mut layouter: impl Layouter<F>,
        col2_row_previous_any_except_0: &AssignedCell<F, F>,
        col3_row_previous_any_except_0: &AssignedCell<F, F>,
    ) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "Next Row",
            |mut region| {
                self.config.col_selector_1.enable(&mut region, 0)?;

                // Copy the value from b & c in previous row to a & b in current row
                col2_row_previous_any_except_0.copy_advice(
                    || "col_advice_1",
                    &mut region,
                    self.config.col_advice_1,
                    0,
                )?;
                col3_row_previous_any_except_0.copy_advice(
                    || "col_advice_2",
                    &mut region,
                    self.config.col_advice_2,
                    0,
                )?;

                let col3_row_any_except_0 = region.assign_advice(
                    || "col_advice_3",
                    self.config.col_advice_3,
                    0,
                    || col2_row_previous_any_except_0.value().copied() + col3_row_previous_any_except_0.value(),
                )?;

                Ok(col3_row_any_except_0)
            },
        )
    }

}

// The configuration of the circuit
#[derive(Default, Clone)]
struct MyCircuit<F>(PhantomData<F>);

impl<F: Field> Circuit<F> for MyCircuit<F> {
    type Config = FibonacciConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        FibonacciChip::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let chip = FibonacciChip::construct(&config);
        let (_, mut prev_col_advice_2, mut prev_col_advice_3) =
            chip.assign_first_row(layouter.namespace(|| "First Row"))?;

        for _i in 3..10 {
            let current_col_advice_3 = chip.assign_row(layouter.namespace(|| "Next Row"), &prev_col_advice_2, &prev_col_advice_3)?;
            prev_col_advice_2 = prev_col_advice_3;
            prev_col_advice_3 = current_col_advice_3;
        }

        // Expose the final value
        layouter.constrain_instance( prev_col_advice_3.cell(), config.col_instance_1, 2)?;
        Ok(())
    
    }
}

fn main() {
    let k = 4;

    let fib0 = Fp::from(1); // fib[0]
    let fib1 = Fp::from(1); // fib[1]
    let _fib1_incorrect = Fp::from(7); // incorrect fib[1]
    let fibn = Fp::from(55); // fib[9]
    let _fibn_incorrect = Fp::from(30); // incorrect fib[9]

    let circuit = MyCircuit(PhantomData);
    let public_input = vec![fib0, fib1, fibn];

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

    ///////////// incorrect result 
   // let public_input_incorrect = vec![fib0, fib1, _fibn_incorrect];
    // let public_input_prover = &public_input_incorrect;

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
    let public_input_verifier = vec![fib0, fib1, fibn];
 //   let public_input_verifier = vec![fib0, _fib1_incorrect, fibn];
    let public_input_verify = &public_input_verifier;

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
//    use plotters::prelude::*;

    // let root = BitMapBackend::new("fibonacci-layout.png", (1024, 3096)).into_drawing_area();
    // root.fill(&WHITE).unwrap();
    // let root = root.titled("Fibonacci Layout", ("sans-serif", 60)).unwrap();

    // let circuit = MyCircuit::<Fp>(PhantomData);
    // halo2_proofs::dev::CircuitLayout::default()
    //     .render(4, &circuit, &root)
    //     .unwrap();

}