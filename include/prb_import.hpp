#pragma once

#include <vector>
#include <string>
#include <cstdint>

namespace PaperAssetPackager
{
    struct AABB
    {
        float min_x = 0.0f;
        float max_x = 0.0f;
        float min_y = 0.0f;
        float max_y = 0.0f;
        float min_z = 0.0f;
        float max_z = 0.0f;
    };

    struct MeshData
    {
        uint32_t vertex_stride = 0;
        uint32_t index_stride = 0;
        uint64_t vbo_offset = 0;
        uint64_t ibo_offset = 0;
        uint64_t vbo_size = 0;
        uint64_t ibo_size = 0;
    };

    struct LODData
    {
        float screen_size = 0;
        std::vector<MeshData> meshes = {};
    };

    struct ModelData
    {
        std::vector<uint8_t> vertex_data = {};
        std::vector<uint8_t> index_data = {};
        std::vector<LODData> lods = {};
        std::string model_name = "";
        AABB aabb = {};
    };

    ModelData get_model_data()
    {
        ModelData returnData = {};

        //WHOLE BUNCH OF TODO

        return returnData;
    }
}