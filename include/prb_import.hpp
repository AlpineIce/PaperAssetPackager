#pragma once

#include <vector>
#include <string>
#include <cstdint>
#include <fstream>
#include <optional>

namespace PaperAssetPackager
{
    struct AABB
    {
        float max_x = 0.0f;
        float min_x = 0.0f;
        float max_y = 0.0f;
        float min_y = 0.0f;
        float max_z = 0.0f;
        float min_z = 0.0f;
    };

    struct Mesh
    {
        uint32_t vertex_stride = 0;
        std::vector<char> vertex_data = {};
        uint32_t index_stride = 0;
        std::vector<char> index_data = {};
        uint64_t invoke_any_hit = 0; //treat this as a bool
    };

    struct LODData
    {
        float screen_size = 0.0;
        std::vector<Mesh> meshes = {};
    };

    struct ModelData
    {
        std::vector<LODData> lods = {};
        std::string model_name = "";
        uint64_t id = 0xFFFFFFFFFFFFFFFF;
        AABB aabb = {};
    };

    //Wrapper around prb file. Good strategy for multithreaded loading would to be to create multiple of these
    struct PaperBinaryFile
    {
        std::ifstream file;
    };

    //Returns PaperBinary if file is valid. Return value should be verified
    std::optional<PaperBinaryFile> get_paper_binary(const char* path) noexcept
    {
        std::ifstream file(path, std::ios::binary | std::ios::ate);

        if(!file)
        {
            return std::nullopt;
        }
        else
        {
            return std::optional<PaperBinaryFile>({ std::move(file) });
        }
    }

    //Get the model entry count of the file
    uint64_t get_entry_count(PaperBinaryFile& prb) noexcept
    {
        //set pointer to 0
        prb.file.seekg(0);

        //read entry count
        uint64_t entry_count = 0;
        prb.file.read((char*)&entry_count, 8);

        return entry_count;
    }

    //Function for loading a single model based on a known index. Returns nullopt if index does not exist
    std::optional<ModelData> load_model(PaperBinaryFile& prb, uint64_t index) noexcept
    {
        //return if index is greater than or equal to entry count
        if(get_entry_count(prb) <= index)
        {
            return std::nullopt;
        }

        //helper variable
        constexpr uint64_t entry_offset = 8;

        //read entry
        struct PrbHeaderEntry
        {
            uint64_t id = 0;
            uint64_t offset = 0;
        } header_entry = {};
        prb.file.seekg(entry_offset + (index * sizeof(PrbHeaderEntry)));
        prb.file.read((char*)&header_entry, sizeof(PrbHeaderEntry));

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
        prb.file.seekg(header_entry.offset);
        prb.file.read((char*)&model_read_data, sizeof(ModelReadData));

        //read model name
        std::string model_name(model_read_data.name_size, 0);
        prb.file.seekg(header_entry.offset + sizeof(ModelReadData));
        prb.file.read(model_name.data(), model_name.size());
        
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
            prb.file.seekg(lod_offset + (lod_index * sizeof(LODReadData)));
            prb.file.read((char*)&lod_read_data, sizeof(LODReadData));
            
            //iterate meshes
            const uint64_t mesh_offset = lod_read_data.meshes_location;
            std::vector<Mesh> meshes = {};
            meshes.reserve(lod_read_data.mesh_count);
            for(uint32_t mesh_index = 0; mesh_index < lod_read_data.mesh_count; mesh_index++)
            {
                //read mesh data 
                struct MeshData
                {
                    uint32_t vertex_stride = 0;
                    uint32_t index_stride = 0;
                    uint64_t vbo_offset = 0;
                    uint64_t ibo_offset = 0;
                    uint64_t vbo_size = 0;
                    uint64_t ibo_size = 0;
                    uint64_t invoke_any_hit = 0; //treat this as a bool
                } mesh_data = {};
                prb.file.seekg(mesh_offset + (mesh_index * sizeof(MeshData)));
                prb.file.read((char*)&mesh_data, sizeof(MeshData));

                //get vertex buffer
                std::vector<char> vertex_buffer(mesh_data.vbo_size);
                prb.file.seekg(model_read_data.vb_location + mesh_data.vbo_offset);
                prb.file.read(vertex_buffer.data(), vertex_buffer.size());

                //get index buffer
                std::vector<char> index_buffer(mesh_data.ibo_size);
                prb.file.seekg(model_read_data.ib_location + mesh_data.ibo_offset);
                prb.file.read(index_buffer.data(), index_buffer.size());

                //push back mesh
                meshes.emplace_back(
                    mesh_data.vertex_stride,
                    std::move(vertex_buffer),
                    mesh_data.index_stride,
                    std::move(index_buffer),
                    mesh_data.invoke_any_hit
                );
            }

            //push back lod
            lods.emplace_back(
                lod_read_data.screen_size,
                std::move(meshes)
            );
        }

        //push back model data
        return std::optional<ModelData>({
            std::move(lods),
            std::move(model_name),
            header_entry.id,
            model_read_data.aabb
        });
    }

    std::vector<ModelData> get_all_model_data(PaperBinaryFile& prb) noexcept
    {
        //get entry count
        const uint64_t entry_count = get_entry_count(prb);

        //initialize return data
        std::vector<ModelData> return_data = {};
        return_data.reserve(entry_count);

        //read all prb file entries
        for(uint64_t i = 0; i < entry_count; i++)
        {
            std::optional<ModelData> model_data = load_model(prb, i);

            if(model_data.has_value())
            {
                return_data.push_back(model_data.value());
            }
        }

        return return_data;
    }
}