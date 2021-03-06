
// use formatting::MT;
use cl_h::{self, cl_platform_id, cl_device_id, cl_device_type, cl_context};

/// An OpenCL context for a particular platform and set of device types.
///
/// Wraps a 'cl_context' such as that returned by 'clCreateContext'.
pub struct Context {
	platform_opt: Option<cl_platform_id>,
	device_ids: Vec<cl_device_id>,
	obj: cl_context,
}

impl Context {
	/// Constructs a new `Context` within a specified platform and set of device types.
	/// 
	/// The desired platform may be specified by passing a valid index from a list 
	/// obtainable from the ocl::get_platform_ids() function and wrapping it with a 
	/// Some (ex. `Some(2)`). Pass `None` to use the first platform available (0). 
	/// 
	/// The device types mask may be specified using a union of any or all of the 
	/// following flags:
	/// - `CL_DEVICE_TYPE_DEFAULT`: The default OpenCL device in the system.
	/// - `CL_DEVICE_TYPE_CPU`: An OpenCL device that is the host processor. The host processor runs the OpenCL implementations and is a single or multi-core CPU.
	/// - `CL_DEVICE_TYPE_GPU`: An OpenCL device that is a GPU. By this we mean that the device can also be used to accelerate a 3D API such as OpenGL or DirectX.
	/// - `CL_DEVICE_TYPE_ACCELERATOR`: Dedicated OpenCL accelerators (for example the IBM CELL Blade). These devices communicate with the host processor using a peripheral interconnect such as PCIe.
	/// - `CL_DEVICE_TYPE_ALL`: A union of all flags.
	///
	/// Passing `None` will use the flag: `CL_DEVICE_TYPE_GPU`.
	///
	/// # Examples
	///	
	/// ```notest
	///	// use ocl;
	///
	///	fn main() {
	///		// Create a context with the first available platform and the default device type.
	///		let ocl_context = ocl::Context::new(None, None);
	///		
	///		// Do fun stuff...
	/// }
	/// ```
	///
	///
	/// ```notest
	/// // use ocl;
	/// 
	/// fn main() {
	/// 	//let platform_ids = ocl::get_platform_ids();
	/// 
	/// 	let device_types = ocl::CL_DEVICE_TYPE_GPU | ocl::CL_DEVICE_TYPE_CPU;
	///
	/// 	// Create a context using the 1st platform and both CPU and GPU devices.
	/// 	let ocl_context = ocl::Context::new(Some(0), Some(device_types));
	/// 	
	/// 	// ...
	/// }
	/// ```	
	///
	/// # Panics
	///    - `get_device_ids()` (work in progress)
	///
	/// # Failures
	/// - No platforms.
	/// - Invalid platform index.
	/// - No devices.
	///
	/// # TODO:
	/// - Add a more in-depth constructor which accepts an arbitrary list of devices (or sub-devices) and a list of cl_context_properties.
	///
	/// # Maybe Someday TODO:
	/// - Handle context callbacks.
	///
	pub fn new(platform_idx_opt: Option<usize>, device_types_opt: Option<cl_device_type>) 
			-> Result<Context, &'static str>
	{
		let platforms = super::get_platform_ids();
		if platforms.len() == 0 { return Err("\nNo OpenCL platforms found!\n"); }

		let platform = match platform_idx_opt {
			Some(pf_idx) => {
				match platforms.get(pf_idx) {
					Some(&pf) => pf,
					None => return Err("Invalid OpenCL platform index specified. \
						Use 'get_platform_ids()' for a list."),
				}				
			},

			None => platforms[super::DEFAULT_PLATFORM],
		};
		
		let device_ids: Vec<cl_device_id> = super::get_device_ids(platform, device_types_opt);
		if device_ids.len() == 0 { return Err("\nNo OpenCL devices found!\n"); }

		// println!("{}OCL::NEW(): device list: {:?}", MT, device_ids);

		let obj: cl_context = super::create_context(&device_ids);

		Ok(Context {
			platform_opt: Some(platform),
			device_ids: device_ids,
			obj: obj,
		})
	}

	pub fn resolve_device_id(&self, device_idx: Option<usize>) -> cl_device_id {
		match device_idx {
			Some(di) => self.valid_device(di),
			None => self.device_ids()[super::DEFAULT_DEVICE],
		}
	}

	/// Returns the current context as a `*mut libc::c_void`.
	pub fn obj(&self) -> cl_context {
		self.obj
	}

	/// Returns a list of `*mut libc::c_void` corresponding to devices valid for use in this context.
	pub fn device_ids(&self) -> &Vec<cl_device_id> {
		&self.device_ids
	}

	/// Returns the platform our context pertains to.
	pub fn platform(&self) -> Option<cl_platform_id> {
		self.platform_opt
	}

	/// Returns a valid device regardless of whether or not the index passed is valid by performing a modulo operation on it.
	pub fn valid_device(&self, selected_idx: usize) -> cl_device_id {
		let valid_idx = selected_idx % self.device_ids.len();
		self.device_ids[valid_idx]
	}

	/// Releases the current context.
	pub fn release(&mut self) {		
    	unsafe {
			cl_h::clReleaseContext(self.obj);
		}
	}
}

