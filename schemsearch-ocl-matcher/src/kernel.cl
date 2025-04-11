// Use 3d_img
// Weniger Allocs an Buffern
// Pattern Parallelisieren mit Local Workern?
// To Match on GPU
// Weniger Worker, Mehr Parameter!
// Pattern als Kernel Konstante

__kernel void add(__global int *result, __global uint *schem,
                  __constant uint *pattern, const int width, const int height,
                  const int depth, const int p_width, const int p_height,
                  const int p_depth, const uint air_id, const int ignore_air,
                  const int air_as_any, const int skipamount) {
  int x = get_global_id(0);
  int y = get_global_id(2);
  int z = get_global_id(1);

  int wrong_blocks = 0;
  for (int py = 0; py < p_height; py++) {
    for (int pz = 0; pz < p_depth; pz++) {
      for (int px = 0; px < p_width; px++) {
        // if ((ignore_air && schem_block != air_id) || (air_as_any &&
        // pattern_block != air_id)) {
        //     continue; // TODO: PROBLEM!
        // }

        wrong_blocks +=
            schem[(x + px) + width * ((z + pz) + (y + py) * depth)] !=
            pattern[px + p_width * (pz + py * p_depth)];
      }
    }
  }

  int idx = x + z * width + y * width * depth;
  result[idx] = wrong_blocks;
}
