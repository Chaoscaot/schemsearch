#include <stdint.h>
#include <stddef.h>

int32_t isMatching(
        const int32_t *schem_data,
        const int32_t *pattern_data,
        size_t pattern_data_length,
        int32_t x,
        int32_t y,
        int32_t z,
        int32_t schem_width,
        int32_t schem_length,
        int32_t pattern_width,
        int32_t pattern_height,
        int32_t pattern_length,
        int32_t *w_ptr
        ) {
    for (int j = 0; j < pattern_height; ++j) {
        for (int k = 0; k < pattern_length; ++k) {
            int pattern_index_pre = k * pattern_width + j * pattern_width * pattern_length;
            int schem_index_pre = x + (k + z) * schem_width + (j + y) * schem_width * schem_length;
            for (int i = 0; i < pattern_width; ++i) {
                int pattern_index = i + pattern_index_pre;
                int schem_index = i + schem_index_pre;
                w_ptr[pattern_index] = schem_data[schem_index];
            }
        }
    }

    int32_t matching = 0;
    for (int i = 0; i < pattern_data_length; ++i) {
        matching += w_ptr[i] == pattern_data[i];
    }

    return matching;
}

void is_matching_all(
        const int32_t *schem_data,
        const int32_t *pattern_data,
        int32_t schem_width,
        int32_t schem_height,
        int32_t schem_length,
        int32_t pattern_width,
        int32_t pattern_height,
        int32_t pattern_length,
        int32_t *result
) {
    for (int32_t pz = 0; pz < pattern_length; ++pz) {
        int32_t maxZ = schem_length - pattern_length + pz + 1;
        for (int32_t py = 0; py < pattern_height; ++py) {
            int32_t maxY = schem_height - pattern_height + py + 1;
            for (int32_t px = 0; px < pattern_width; ++px) {
                int32_t pv = pattern_data[px + py * pattern_width + pz * pattern_width * pattern_height];
                int32_t maxX = schem_width - pattern_width + px + 1;
                for (int32_t z = pz; z < maxZ; ++z) {
                    int32_t sourceOffsetZ = z * schem_width * schem_height;
                    int32_t resultOffsetZ = (z - pz) * schem_width * schem_height - py * schem_width;
                    for (int32_t y = py; y < maxY; ++y) {
                        int32_t sourceOffsetY = sourceOffsetZ + y * schem_width;
                        int32_t resultOffsetY = resultOffsetZ + y * schem_width - px;
                        for (size_t x = px; x < maxX; ++x) {
                            result[resultOffsetY + x] += schem_data[sourceOffsetY + x] == pv;
                        }
                    }
                }
            }
        }
    }
}
