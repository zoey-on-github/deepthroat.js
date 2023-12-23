import init, { deep_throat } from "./rust_deps/pkg/rust_deps.js";

// Define a function that encapsulates the logic
async function deepthroat(input) {
  await init();

  return deep_throat(input);
}

// Export the function
export default deepthroat;
