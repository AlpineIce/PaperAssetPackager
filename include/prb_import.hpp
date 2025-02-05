#pragma once

#include <vector>
#include <string>
#include <cstdint>
#include <fstream>

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
        std::vector<char> vertex_data = {};
        std::vector<char> index_data = {};
        std::vector<LODData> lods = {};
        std::string model_name = "";
        AABB aabb = {};
    };

    std::vector<ModelData> get_model_data(const char* path)
    {
        //load file
        std::ifstream file(path, std::ios::binary | std::ios::ate);

        if(!file) throw std::runtime_error("Failed to open input file");

        //set pointer to 0
        file.seekg(0);

        //read entry count
        uint64_t entry_count = 0;
        file.read((char*)&entry_count, 8);

        //initialize return data
        std::vector<ModelData> return_data = {};
        return_data.reserve(entry_count);

        //move pointer to 8
        file.seekg(8);

        //read file entries
        const uint64_t entry_offset = 8;
        for(uint64_t entry_index = 0; entry_index < entry_count; entry_index++)
        {
            //read entry
            struct PrbHeaderEntry
            {
                uint64_t id = 0;
                uint64_t offset = 0;
            } header_entry = {};
            file.seekg(entry_offset + (entry_index * sizeof(PrbHeaderEntry)));
            file.read((char*)&header_entry, sizeof(PrbHeaderEntry));

            //read model data
            struct ModelReadData
            {
                uint64_t vb_location = 0;
                uint64_t ib_location = 0;
                uint64_t vb_size = 0;
                uint64_t ib_size = 0;
                uint64_t lod_count = 0;
                uint64_t lods_location = 0;
                uint64_t name_size = 0;
                AABB aabb = {};
            } model_read_data = {};
            file.seekg(header_entry.offset);
            file.read((char*)&model_read_data, sizeof(ModelReadData));

            //read model name
            std::string model_name(model_read_data.name_size, 0);
            file.seekg(header_entry.offset + sizeof(ModelReadData));
            file.read(model_name.data(), model_name.size());
            
            //iterate LODs
            const uint64_t lod_offset = model_read_data.lods_location;
            std::vector<LODData> lods = {};
            lods.reserve(model_read_data.lod_count);
            for(uint32_t lod_index = 0; lod_index < model_read_data.lod_count; lod_index++)
            {
                //read LOD data
                struct LODReadData
                {
                    float screen_size;
                    uint32_t mesh_count;
                    uint64_t meshes_location;
                } lod_read_data = {};
                file.seekg(lod_offset + (lod_index * sizeof(LODReadData)));
                file.read((char*)&lod_read_data, sizeof(LODReadData));
                
                //iterate meshes
                const uint64_t mesh_offset = lod_read_data.meshes_location;
                std::vector<MeshData> meshes = {};
                meshes.reserve(lod_read_data.mesh_count);
                for(uint32_t mesh_index = 0; mesh_index < lod_read_data.mesh_count; mesh_index++)
                {
                    //read mesh data
                    MeshData mesh_data = {};
                    file.seekg(mesh_offset + (mesh_index * sizeof(MeshData)));
                    file.read((char*)&mesh_data, sizeof(MeshData));

                    //push back mesh
                    meshes.push_back(mesh_data);
                }

                //push back lod
                lods.push_back({
                    .screen_size = lod_read_data.screen_size,
                    .meshes = std::move(meshes)
                });
            }

            //get vertex buffer
            std::vector<char> vertex_buffer(model_read_data.vb_size);
            file.seekg(model_read_data.vb_location);
            file.read(vertex_buffer.data(), vertex_buffer.size());

            //get index buffer
            std::vector<char> index_buffer(model_read_data.ib_size);
            file.seekg(model_read_data.ib_location);
            file.read(index_buffer.data(), index_buffer.size());

            //push back model data
            return_data.push_back({
                .vertex_data = std::move(vertex_buffer),
                .index_data = std::move(index_buffer),
                .lods = std::move(lods),
                .model_name = std::move(model_name),
                .aabb = model_read_data.aabb
            });
        }

        return return_data;
    }
}