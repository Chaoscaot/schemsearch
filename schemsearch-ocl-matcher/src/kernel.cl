__kernel void add(__global int* result,
                 __global uint* schem,
                 __global uint* pattern,
                 const int p_width,
                 const int p_height,
                 const int p_depth,
                 const uint air_id,
                 const int ignore_air,
                 const int air_as_any,
                 const int skipamount) {
    int x = get_global_id(0);
    int y = get_global_id(1);
    int z = get_global_id(2);
    
    int width = get_global_size(0);
    int height = get_global_size(1);
    int depth = get_global_size(2);
    
    if (x > width - p_width || y > height - p_height || z > depth - p_depth) {
        return;
    }
    
    int wrong_blocks = 0;
    for (int py = 0; py < p_height; py++) {
        for (int pz = 0; pz < p_depth; pz++) {
            for (int px = 0; px < p_width; px++) {
                int s_idx = (x + px) + width * ((z + pz) + (y + py) * depth);
                int p_idx = px + p_width * (pz + py * p_depth);
                
                uint schem_block = schem[s_idx];
                uint pattern_block = pattern[p_idx];

                if ((ignore_air && schem_block != air_id) || (air_as_any && pattern_block != air_id)) {
                    continue;
                }

                if (schem_block != pattern_block) {
                    wrong_blocks++;
                    if (wrong_blocks > skipamount) {
                        int idx = x + z * width + y * width * depth;
                        result[idx] = wrong_blocks;
                        return;
                    }
                }
            }
        }
    }
    
    int idx = x + z * width + y * width * depth;
    result[idx] = wrong_blocks;
}
