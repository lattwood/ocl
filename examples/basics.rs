use ocl::{Context, ProQue, BuildConfig, SimpleDims, Envoy};
extern crate ocl;

const RESULTS_TO_PRINT: usize = 20;

fn main() {
	// Set our data set size and coefficent to arbitrary values:
	let data_set_size = 900000;
	let coeff = 5432.1;

	// Create a context with the first avaliable platform and default device type:
	let ocl_cxt = Context::new(None, None).unwrap();

	// Create a program/queue with the first available device: 
	let mut ocl_pq = ProQue::new(&ocl_cxt, None);

	// Declare our kernel source code:
	let kernel_src = r#"
		__kernel void multiply_by_scalar(
					__global float const* const src,
					__private float const coeff,
					__global float* const res)
		{
			uint const idx = get_global_id(0);

			res[idx] = src[idx] * coeff;
		}
	"#;

	// Create a basic build configuration using above source: 
	let build_config = BuildConfig::new().kern_embed(kernel_src);

	// Build with our configuration and check for errors:
	ocl_pq.build(build_config).expect("ocl program build");

	// Set up our work dimensions / data set size:
	let dims = SimpleDims::OneDim(data_set_size);

	// Create an envoy (a local array + a remote buffer) as a data source:
	let source_envoy = Envoy::scrambled(&dims, 0.0f32, 20.0f32, &ocl_pq.queue());

	// Create another empty envoy for results:
	let mut result_envoy = Envoy::new(&dims, 0.0f32, &ocl_pq.queue());

	// Create a kernel with three arguments corresponding to those in the kernel:
	let kernel = ocl_pq.create_kernel("multiply_by_scalar", dims.work_size())
		.arg_env(&source_envoy)
		.arg_scl(coeff)
		.arg_env(&mut result_envoy)
	;

	// Enqueue kernel depending on and creating no events:
	kernel.enqueue(None, None);

	// Read results:
	result_envoy.read_wait();

	// Check results and print the first 20:
	for idx in 0..data_set_size {
		// Check:
		assert_eq!(result_envoy[idx], source_envoy[idx] * coeff);

		// Print:
		if idx < RESULTS_TO_PRINT { 
			println!("source_envoy[idx]: {}, coeff: {}, result_envoy[idx]: {}",
			source_envoy[idx], coeff, result_envoy[idx]); 
		}
	}
}
